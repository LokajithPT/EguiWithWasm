#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "wasmandegui",
        options,
        Box::new(|_cc| Ok(Box::new(wasmandegui::WasmandeguiApp::default()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {}
