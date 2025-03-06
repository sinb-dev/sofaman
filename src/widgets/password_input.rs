#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use egui::{Context, TextEdit, Ui, Widget, WidgetText, Response, Pos2, Rect, Sense, Color32, FontId, TextStyle, FontFamily};
use eframe::egui;

pub struct PasswordInput<'a> {
    text: &'a mut String,
    mask: char,
}

impl<'a> PasswordInput<'a> {
    pub fn new(text: &'a mut String, mask: char) -> Self {
        Self { text, mask }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Response {
        let mut text_edit = TextEdit::singleline(self.text)
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
