use crate::{app_state::AppState, pages::page::{Page, Pages}};
use std::rc::Rc;
use std::cell::RefCell;
use eframe::egui::{self, Button, Vec2};

pub struct SideMenu {
    app_state: Rc<RefCell<AppState>>,

}

impl Page for SideMenu {
    fn from_app_state(app_state: Rc<RefCell<AppState>>) -> Self {
        Self {
            app_state: app_state
        }
    }
}

impl eframe::App for SideMenu {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("menu").show(ctx, |ui| {
            let mut workspace_btn = Button::new("Workspace");
            let mut accounts_btn = Button::new("Accounts");
            workspace_btn = workspace_btn.min_size(Vec2::new(120.0,40.0));
            accounts_btn = accounts_btn.min_size(Vec2::new(120.0,40.0));
            {
                let app_state = self.app_state.borrow();
                match app_state.active_page() {
                    Pages::Workspace => { workspace_btn = workspace_btn.selected(true); }
                    Pages::Accounts => { accounts_btn = accounts_btn.selected(true); }
                    _ => ()
                }
            }
            
            ui.vertical(|ui| {
                if ui.add(workspace_btn).clicked() {
                    let mut app_state = self.app_state.borrow_mut();
                    app_state.goto_workspace();
                }
                if ui.add(accounts_btn).clicked() {
                    let mut app_state = self.app_state.borrow_mut();
                    app_state.goto_accounts();
                }
            });
        });
    }
}