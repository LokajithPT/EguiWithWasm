use eframe::egui;

pub struct WasmandeguiApp {
    count: i32,
}

impl Default for WasmandeguiApp {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl eframe::App for WasmandeguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("wasmandegui");
            ui.label("Hello from egui!");
            if ui.button("Click me").clicked() {
                self.count += 1;
            }
            ui.label(format!("Clicked {} times", self.count));
        });
    }
}

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
            Box::new(|_cc| Ok(Box::new(WasmandeguiApp::default()))),
        )
        .await
        .ok();
}
