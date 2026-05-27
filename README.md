# EguiWithWasm

**egui app — native desktop + browser, same codebase.**

Write your UI once in Rust with [`egui`](https://github.com/emilk/egui). Run it as a native window with `cargo run`, or serve it in any browser with `trunk serve`.

## Quick start

```bash
# native
cargo run

# web
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve
```

## What's inside

```
src/
├── lib.rs      # your app logic + wasm entry point
└── main.rs     # native entry point (conditionally compiled)
```

| Command | Runs on | What happens |
|---|---|---|
| `cargo run` | Desktop (Linux, macOS, Windows) | Opens a native window using `eframe::run_native` |
| `trunk serve` | Browser | Cross-compiles to wasm, starts dev server at `http://localhost:8080` |

## How it works

The app is defined once in `lib.rs` as a struct implementing `eframe::App`. The same struct is launched two ways:

- **Native** — `main.rs` calls `eframe::run_native`. This function only exists on non-wasm targets, so the binary is `#[cfg]`-gated.
- **Web** — `lib.rs` exports a `#[wasm_bindgen]` async `start()` function. `index.html` loads it via trunk's ES module pipeline and calls `start()` once the wasm is ready.

The `start()` function grabs the `<canvas>` element from the page, initializes `eframe::WebRunner`, and hands it your app. From there, egui handles rendering, input, and resizing.

There's **no platform-specific code** in the app logic itself. Every `ui()` call draws to both targets identically.

## Common issues

| Symptom | Fix |
|---|---|
| `wasm_bindgen_futures` not found | `cargo add wasm-bindgen-futures` |
| `fn update` missing / `fn ui` required | eframe 0.34 uses `fn ui(&mut self, ui: &mut Ui, frame: &mut Frame)` |
| `run_native` doesn't exist on wasm | Gate `main.rs` with `#[cfg(not(target_arch = "wasm32"))]` |
| Trunk says "more than one target" | Rename the bin (e.g. `your-app-native`) and use `<link data-trunk data-target-name="your-app">` |
| Canvas stuck in top-left corner | Add `canvas { width: 100%; height: 100% }` and `body { overflow: hidden }` |
| White screen / JS errors | Don't use `<script src="wasm.js">` — let trunk inject the module, listen for `TrunkApplicationStarted` |

## Dependencies

- [`eframe`](https://crates.io/crates/eframe) 0.34 — egui framework for native + web
- [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen) — JS-Rust interop for the web build
- [`wasm-bindgen-futures`](https://crates.io/crates/wasm-bindgen-futures) — required by the proc macro on async exports
- [`web-sys`](https://crates.io/crates/web-sys) — browser DOM access (canvas, document, window)
- [`trunk`](https://trunkrs.dev) — wasm build tool and dev server

## License

MIT
