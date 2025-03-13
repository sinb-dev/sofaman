#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

//use egui::{Context, TextEdit, Ui, Widget, WidgetText, Response, Pos2, Rect, Sense, Color32, FontId, TextStyle, FontFamily};
//use eframe::{egui::{self, TextBuffer}, glow::SHADER_COMPILER};

mod accounting {
    pub mod data;
    pub mod models;
}

mod data {
    pub mod InMemory;
    pub mod sqlite;
}

mod tests {
    mod accounting;
    mod data_access;
}
mod pages {
    pub mod page;
    pub mod login;
    pub mod workspace;
    pub mod account_list;
}
mod widgets {
    pub mod password_input;
    pub mod side_menu;
}
mod data_access;

mod app_state;
mod models {
    pub mod models;
    //pub mod accounting;
}


use data::sqlite::{SqliteAccountService, SqliteServiceManager};
use data_access::{DataContext, ServiceManager};
use eframe::egui::{self, ahash::HashMap};
use std::rc::Rc;
use std::cell::RefCell;
use pages::{account_list::AccountListPage, page::{Page, Pages}};
use app_state::AppState;

use pages::{
    login::LoginPage,
    workspace::WorkspacePage,
};
fn main() -> eframe::Result {
      
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Sofaman",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            catppuccin_egui::set_theme(&cc.egui_ctx, catppuccin_egui::MOCHA);
            let context: DataContext = DataContext {
                service_manager: Box::new(SqliteServiceManager::new())
            };
            let app_state = Rc::new(RefCell::new(AppState::new(context)));
            let my_app = Box::new(MyApp::new(Rc::clone(&app_state)));
   
            Ok(my_app)
        }),
    )
}


struct MyApp {
    app_state: Rc<RefCell<AppState>>,
    login_page: LoginPage,
    workspace_page: WorkspacePage,
    accounts_page: AccountListPage
}
impl MyApp {
    fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        
        let login_page = LoginPage::new(Rc::clone(&app_state));
        let workspace_page = WorkspacePage::from_app_state(Rc::clone(&app_state));
        let accounts_page = AccountListPage::from_app_state(Rc::clone(&app_state));
        Self {
            app_state : app_state,
            login_page : login_page,
            workspace_page : workspace_page,
            accounts_page : accounts_page,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut _active_page = Pages::Login;
        {
            let app_state = self.app_state.borrow();
            _active_page = app_state.active_page();
        }
        match _active_page {
            Pages::Login => self.login_page.update(ctx, _frame),
            Pages::Workspace => self.workspace_page.update(ctx, _frame),
            Pages::Accounts => self.accounts_page.update(ctx, _frame),
        };
    }
}

// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("My egui Application");
//             ui.horizontal(|ui| {
//                 let name_label = ui.label("Your name: ");
//                 ui.text_edit_singleline(&mut self.name)
//                     .labelled_by(name_label.id);
//                 self.pwd.ui(ui)
//             });
//             ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
//             if ui.button("Increment").clicked() {
//                 self.age += 1;
//             }
//             ui.label(format!("Hello '{}', age {}", self.name, self.age));

//             ui.image(egui::include_image!(
//                 "../assets/wtf.png"
//             ));
//         });
//     }
// }


