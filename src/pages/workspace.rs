use eframe::egui;
use crate::pages::page::Page;
use std::any::Any;
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
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl eframe::App for WorkspacePage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Welcome: {}", self.message));
        });

    }
}