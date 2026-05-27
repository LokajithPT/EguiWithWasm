#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "wasmandegui",
        options,
        Box::new(|cc| Ok(Box::new(wasmandegui::WasmandeguiApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}
