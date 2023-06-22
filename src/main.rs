use proto::{Engine, OutputDevice};
use eframe::egui;


fn main() {
    let engine = Engine::new(&OutputDevice::default().unwrap(), 64, 44100);
    let app = Nutrino{ engine };
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<Nutrino>::new(app)),
    ).unwrap();
}

struct Nutrino {
    engine: Engine,
}

impl eframe::App for Nutrino {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.label("Nutrino 0.0.1");
        });
    }
}