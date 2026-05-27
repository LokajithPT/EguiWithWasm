# DOIT.md — egui native + web dual-target setup

You have an egui app that works natively with `cargo run`. Now you want `trunk serve` to serve it in a browser too. This doc covers the whole setup and every trap encountered.

## Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## File structure

```
your-project/
├── Cargo.toml
├── trunk.toml
├── index.html
└── src/
    ├── lib.rs        # shared app logic + wasm entry point
    └── main.rs       # native-only entry point
```

## Cargo.toml

```toml
[package]
name = "your-app"
version = "0.1.0"
edition = "2021"
default-run = "your-app-native"           # so `cargo run` picks the binary

[lib]
crate-type = ["cdylib", "rlib"]          # cdylib is required for wasm

[dependencies]
eframe = "0.34"
log = "0.4"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = ["Document", "Element", "HtmlCanvasElement", "Window"] }

[[bin]]
name = "your-app-native"                 # MUST differ from the lib name
path = "src/main.rs"                     # to avoid filename collision with the lib artifact
```

**Why this matters:**
- `crate-type = ["cdylib", "rlib"]` — the `cdylib` is what wasm-bindgen / trunk uses to produce the `.wasm` file.
- `wasm-bindgen-futures` is **required** when you use `#[wasm_bindgen]` on an `async fn` — the proc macro generates code that references it directly.
- The bin target name **must differ** from the lib name (otherwise both produce `your-app.wasm` and trunk sees duplicate artifacts).

## src/lib.rs

```rust
use eframe::egui;

pub struct YourApp {
    count: i32,
}

impl Default for YourApp {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl eframe::App for YourApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("your-app");
            if ui.button("Click me").clicked() {
                self.count += 1;
            }
            ui.label(format!("Clicked {} times", self.count));
        });
    }
}

// --- web entry point (wasm32 only) ---

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("the_canvas").unwrap();
    let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let runner = eframe::WebRunner::new();
    runner
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(|_cc| Ok(Box::new(YourApp::default()))),
        )
        .await
        .ok();
}
```

**Key points:**
- `fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)` — in eframe 0.34, this is the **required** method. The old `fn update(ctx, frame)` is deprecated.
- Use `show_inside` on panels when inside a `ui` context (`show` is deprecated).
- The `start()` function is exported via `#[wasm_bindgen]` so JavaScript can call it.
- `dyn_into` comes from `JsCast` trait (in `wasm_bindgen::prelude`).

## src/main.rs

```rust
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "your-app",
        options,
        Box::new(|_cc| Ok(Box::new(your_app::YourApp::default()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}
```

**Why the split:**
- `eframe::run_native` and `eframe::NativeOptions` are `#[cfg(not(target_arch = "wasm32"))]` — they don't exist on wasm.
- The wasm stub (`fn main() {}`) is needed because trunk still tries to compile the bin target for wasm32. Without it, the link fails.

## index.html

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>your-app</title>
    <script type="module">
        window.addEventListener("TrunkApplicationStarted", async () => {
            await window.wasmBindings.start();
        });
    </script>
    <link data-trunk rel="rust" data-target-name="your-app" />
    <style>
        html, body { margin: 0; padding: 0; height: 100%; overflow: hidden; }
        canvas { display: block; width: 100%; height: 100%; }
    </style>
</head>
<body>
    <canvas id="the_canvas"></canvas>
</body>
</html>
```

**Why this works:**
- `<link data-trunk rel="rust" data-target-name="your-app" />` tells trunk to build only the **lib** target. Without this, trunk sees multiple artifacts and errors out.
- The first `<script type="module">` registers an event listener **before** trunk's injected module executes. Module scripts execute in document order, so the listener is registered before trunk fires `TrunkApplicationStarted`.
- Trunk's injected module (which replaces the `<link>` tag) does: import wasm → `window.wasmBindings = bindings` → dispatch `TrunkApplicationStarted` → our handler calls `window.wasmBindings.start()`.
- CSS: `canvas { width: 100%; height: 100% }` makes it fill the viewport. `overflow: hidden` prevents scrollbars.

## trunk.toml

```toml
[build]
target = "index.html"
```

## Running

```bash
cargo run        # native desktop window
trunk serve      # browser at http://localhost:8080
```

## Common pitfalls

| Problem | Cause | Fix |
|---|---|---|
| `cannot find module or crate wasm_bindgen_futures` | `#[wasm_bindgen]` on async fn generates code referencing `wasm_bindgen_futures` | Add `wasm-bindgen-futures` to `[dependencies]` |
| `not all trait items implemented, missing: ui` | eframe 0.34 requires `fn ui` not `fn update` | Implement `fn ui(&mut self, ui: &mut Ui, frame: &mut Frame)` |
| `run_native / NativeOptions not found in eframe` | These are `#[cfg(not(target_arch = "wasm32"))]` — don't exist on wasm | Gate the native main with `#[cfg(not(target_arch = "wasm32"))]` |
| `found more than one target artifact` | Lib and bin both produce `your-app.wasm` | Rename the bin target (e.g. `your-app-native`) and use `<link data-trunk data-target-name="your-app">` to select the lib |
| `Uncaught ReferenceError: wasm_bindgen is not defined` | Script loads a non-existent `.js` file | Don't use manual `<script src="...">`; let trunk inject the module and use the `TrunkApplicationStarted` event |
| White screen, UI in top-left | Canvas not sized | CSS: `canvas { display: block; width: 100%; height: 100% }` and `body { overflow: hidden; }` |
