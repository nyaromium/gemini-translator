use eframe::egui;

pub(crate) fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native("Gemini Translator", options, Box::new(|cc| Ok(Box::new(TranslatorApp::new(cc))))).expect("TODO: panic message");
}

#[derive(Default)]
struct TranslatorApp {}

impl TranslatorApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for TranslatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, World!");
        });
    }
}