// src/pages/login.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

//use egui::{Context, TextEdit, Ui, Widget, WidgetText, Response, Pos2, Rect, Sense, Color32, FontId, TextStyle, FontFamily};
use eframe::egui;
use crate::app_state::AppState;
// Use the PasswordInput struct from the password_input module
use crate::widgets::password_input::PasswordInput;
use std::rc::Rc;
use std::cell::RefCell;

pub struct LoginPage {
    username: String,
    password: String,
    app_state: Rc<RefCell<AppState>>
}
impl LoginPage {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            app_state: app_state
        }
    }
    
}

impl eframe::App for LoginPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Please log in");
            let name_label = ui.label("Username: ");
            ui.text_edit_singleline(&mut self.username)
                .labelled_by(name_label.id);
            let mut password = PasswordInput::new(&mut self.password, '*');
            password.ui(ui);
            if ui.button("Log in").clicked() {
                let mut app_state = self.app_state.borrow_mut();
                app_state.logged_in();
            }
            
        });

    }
}
