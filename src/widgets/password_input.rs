#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use egui::{TextEdit, Ui, Widget, Response};
use eframe::egui;

pub struct PasswordInput<'a> {
    text: &'a mut String,
}

impl<'a> PasswordInput<'a> {
    pub fn new(text: &'a mut String) -> Self {
        Self { text }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        let text_edit = TextEdit::singleline(self.text)
            .hint_text("Password")
            .password(true);

        let response = text_edit.ui(ui);

        if response.changed() {
            // Handle the changed text if needed
        }

        response
    }
}


// impl<'a> Widget for PasswordInput<'a> {
//     fn ui(self, ui: &mut Ui) -> Response {
//         self.ui(ui)
//     }
// }
