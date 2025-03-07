use eframe::egui;
use crate::pages::page::Page;
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

impl Page for WorkspacePage {

}

impl eframe::App for WorkspacePage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Welcome: {}", self.message));
        });

    }
}