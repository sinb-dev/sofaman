use crate::{app_state::AppState, models::accounting::Account};
use std::rc::Rc;
use std::cell::RefCell;
use eframe::egui::{self};
use egui::{Ui, Response};
use egui_extras::{Column, TableBuilder};
use super::page::Page;
use crate::widgets::side_menu::SideMenu;
pub struct AccountListPage {
    app_state: Rc<RefCell<AppState>>,
    account_list: AccountList,
    //accounts: &'a Vec<Account>
}

impl Page for AccountListPage
{
    fn from_app_state(app_state: Rc<RefCell<crate::app_state::AppState>>) -> Self {
        Self {
            app_state: Rc::clone(&app_state),
            account_list: AccountList::from_app_state(Rc::clone(&app_state)),
        }
    }
}
// impl AccountListPage {

//     pub fn ui(&mut self, ui: &mut Ui) -> Response {
//         //let mut account_list = AccountList::from_app_state(Rc::clone(&self.app_state));
        
//         self.account_list.ui(ui)
//     }
// }

impl eframe::App for AccountListPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        let mut side_menu = SideMenu::from_app_state(Rc::clone(&self.app_state));
        side_menu.update(ctx, _frame);
        egui::TopBottomPanel::top("main").show(ctx, |ui| {
            ui.heading("Accounting");
            //let mut account_list_page = AccountListPage::from_app_state(Rc::clone(&self.app_state));
        });
        egui::TopBottomPanel::bottom("details").show(ctx, |_ui| {
            //let mut account_list_page = AccountListPage::from_app_state(Rc::clone(&self.app_state));
            // let Self { app_state, message, account_list_page} = self;
            // ui.add(Label::new("Message?"));
            // ui.text_edit_singleline(message);
        });
        egui::TopBottomPanel::bottom("details2").show(ctx, |_ui| {
            //let mut account_list_page = AccountListPage::from_app_state(Rc::clone(&self.app_state));
            
            // let Self { app_state, message, account_list_page} = self;
            // ui.add(Label::new("erhh?"));
            // ui.text_edit_singleline(message);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.account_list.ui(ui);

        });

    }
}



pub struct AccountList {
    app_state: Rc<RefCell<AppState>>,
    filter: String,
}
impl Page for AccountList
{
    fn from_app_state(app_state: Rc<RefCell<crate::app_state::AppState>>) -> Self {
        Self {
            app_state: app_state,
            filter: String::new(),
        }
    }
}
impl AccountList {
    fn ui(&mut self, ui: &mut Ui) -> Response {
        //let app_state = self.app_state.borrow();
        let Self { app_state, filter } = self;
        let app_state_br = app_state.borrow();
        let accounts: &Vec<Account> = app_state_br.get_accounts();

        let search_label = ui.label("Search");
        ui.vertical(|ui| {
            ui.text_edit_singleline(filter)
                .labelled_by(search_label.id);
        });
        
        TableBuilder::new(ui)
            .min_scrolled_height(0.0)
            .max_scroll_height(f32::INFINITY)
            .striped(true)
            .column(Column::initial(100.0).at_least(50.0).clip(true))
            .column(Column::initial(150.0).at_least(50.0).clip(true))
            .column(Column::initial(100.0).at_least(50.0).clip(true))
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Account");
                });
                header.col(|ui| {
                    ui.heading("Balance");
                });
                header.col(|ui| {
                    ui.heading("Actions");
                });
            })
            .body(|mut body| {
                for account in accounts {
                    if filter != "" && !account.get_name().contains(filter.as_str()) {
                        continue;
                    }
                    body.row(20.0, |mut row| {
                        row.col(|ui| {
                            ui.label(account.get_name().to_string());
                        });
                        row.col(|ui| {
                            ui.label(account.get_balance().to_string());
                        });
                        
                        row.col(|ui| {
                            if ui.button("Load").clicked() {
                                println!("Loading {}", account.get_name());
                            };
                        });
                        //ui.label(format!("${:.2}", account.balance));
                    });
                }
            });
            //.resizable(true)
            //.show();



        ui.response()
    }
}