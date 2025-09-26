use eframe::egui;
use tokio::runtime::Runtime;

pub fn init_gemini() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gemini Translator",
        options,
        Box::new(|cc| Ok(Box::new(GeminiApp::new(cc))))
    ).expect("TODO: panic message");
}

#[derive(Default)]
struct GeminiApp {
    response_text: String,
    is_loading: bool,
    runtime: Option<Runtime>,
}

impl GeminiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            response_text: "Waiting for Gemini...".to_string(),
            is_loading: false,
            runtime: Some(Runtime::new().unwrap()),
        }
    }
}

impl eframe::App for GeminiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gemini Translator");

            if ui.button("Ask to Gemini").clicked() && !self.is_loading {
                self.is_loading = true;
                self.response_text = "Loading...".to_string();

                if let Some(runtime) = &self.runtime {
                    match runtime.block_on(crate::api::get_gemini_response()) {
                        Ok(response) => {
                            self.response_text = response;
                        }
                        Err(e) => {
                            self.response_text = format!("Error: {}", e);
                        }
                    }
                }
                self.is_loading = false;
            }

            ui.separator();

            ui.heading("Response:");
            ui.label(&self.response_text);
        });
    }
}