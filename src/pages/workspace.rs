use eframe::egui;
pub struct WorkspacePage {
    message: String
}

impl WorkspacePage {
    pub fn new() -> Self {
        Self {
            message: String::new()
        }
    }
}

impl eframe::App for WorkspacePage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Welcome: {}", self.message));
        });

    }
}