// src/pages/login.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

//use egui::{Context, TextEdit, Ui, Widget, WidgetText, Response, Pos2, Rect, Sense, Color32, FontId, TextStyle, FontFamily};
use eframe::egui;

// Use the PasswordInput struct from the password_input module
use crate::widgets::password_input::PasswordInput;
use crate::pages::page::Page;
use std::any::Any;
pub struct LoginEventArgs
{
    pub success: bool
}
impl LoginEventArgs {
    pub fn new(success: bool) -> Self {
        Self {
            success: success
        }
    }
}

pub struct LoginPage<'a> {
    username: String,
    password: String,
    pub on_login: Box<dyn Fn(LoginEventArgs) + 'a>,
}
impl LoginPage<'_> {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            on_login : Box::new(|_| {

            }),
        }
    }
    
}

impl eframe::App for LoginPage<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Please log in");
            let name_label = ui.label("Username: ");
            ui.text_edit_singleline(&mut self.username)
                .labelled_by(name_label.id);
            let mut password = PasswordInput::new(&mut self.password, '*');
            password.ui(ui);
            if ui.button("Log in").clicked() {
                //Attempt to log in
                (self.on_login)(LoginEventArgs::new(true));
            }
            
        });

    }
}
