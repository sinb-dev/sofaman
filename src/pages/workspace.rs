use eframe::egui::{self, Label};

use std::rc::Rc;
use std::cell::RefCell;
use super::page::Page;
use crate::app_state::AppState;
use crate::widgets::side_menu::SideMenu;
pub struct WorkspacePage {
    app_state: Rc<RefCell<AppState>>,
    message: String,

}

impl Page for WorkspacePage {
    fn from_app_state(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            message: String::from("Hello"),
            app_state: Rc::clone(&app_state),
        }
    }
}

impl eframe::App for WorkspacePage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut side_menu = SideMenu::from_app_state(Rc::clone(&self.app_state));
        side_menu.update(ctx, _frame);
        
        egui::TopBottomPanel::top("main").show(ctx, |ui| {
            ui.heading(format!("Accounts: {}", self.message));
            //let mut account_list_page = AccountListPage::from_app_state(Rc::clone(&self.app_state));
        });
        egui::TopBottomPanel::bottom("details").show(ctx, |ui| {
            //let mut account_list_page = AccountListPage::from_app_state(Rc::clone(&self.app_state));
            
            let Self { app_state: _, message} = self;
            ui.add(Label::new("Message?"));
            ui.text_edit_singleline(message);
        });
        egui::TopBottomPanel::bottom("details2").show(ctx, |ui| {
            //let mut account_list_page = AccountListPage::from_app_state(Rc::clone(&self.app_state));
            
            let Self { app_state: _, message} = self;
            ui.add(Label::new("erhh?"));
            ui.text_edit_singleline(message);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(Label::new("Workspace!!"));

        });

    }
}