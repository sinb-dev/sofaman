use crate::{accounting::models::{Account, Transaction}, app_state::{self, AppState}, data_access::{Repository, Service}};
use crate::accounting::data;
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
    selected_account: Option<Account>,
    show_edit_account: bool,
    account_service: Box<dyn Service>
    //accounts: &'a Vec<Account>
}

impl Page for AccountListPage
{
    fn from_app_state(app_state: Rc<RefCell<crate::app_state::AppState>>) -> Self {

        let app_state_clone = Rc::clone(&app_state);

        let app_state_borrow = app_state_clone.borrow();
        let service_manager = &app_state_borrow.get_context().service_manager;

        Self {
            app_state: Rc::clone(&app_state),
            account_list: AccountList::from_app_state(Rc::clone(&app_state)),
            selected_account: None,
            show_edit_account: false,
            account_service: service_manager.accounts_service()
        }
    }
}

impl eframe::App for AccountListPage {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        let mut side_menu = SideMenu::from_app_state(Rc::clone(&self.app_state));
        side_menu.update(ctx, _frame);
        egui::TopBottomPanel::top("main").show(ctx, |ui| {
            ui.heading("Accounting");
            //let mut account_list_page = AccountListPage::from_app_state(Rc::clone(&self.app_state));
        });
        egui::TopBottomPanel::top("accounting_actions").show(ctx, |ui| {
            if ui.button("New account").clicked() {

            }
        });
        

        // egui::TopBottomPanel::bottom("details2").show(ctx, |_ui| {
        //     //let mut account_list_page = AccountListPage::from_app_state(Rc::clone(&self.app_state));
            
        //     // let Self { app_state, message, account_list_page} = self;
        //     // ui.add(Label::new("erhh?"));
        //     // ui.text_edit_singleline(message);
        // });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.account_list.ui(ui);
            if self.account_list.selected_account_id > 0
            {
                let app_state_clone = Rc::clone(&self.app_state);
                let app_state = app_state_clone.borrow();
                let result: Result<&Account, bool> = app_state.get_account_by_id(self.account_list.selected_account_id);
                //let mut account = result.cloned().unwrap();
                match (result.cloned()) {
                    Ok(account) => {
                        self.selected_account = Some(account);
                        self.show_edit_account = true;
                    },
                    Err(bool) => {
                        self.show_edit_account = false;
                    }
                }
            }
        });
        
        //if let Some(account) = &self.selected_account {
        if self.show_edit_account {
            egui::SidePanel::right("edit_account").show(ctx, |ui| {

                let name_label = ui.label("Name");
                ui.vertical(|ui| {

                    // let app_state_clone = Rc::clone(&self.app_state);
                    // let app_state = app_state_clone.borrow();
                    // let result: Result<&Account, bool> = app_state.get_account_by_id(self.selected_account_id);
                    // let mut account = result.cloned().unwrap();
                    if let Some(account) = &mut self.account_list.selected_account {
                        ui.text_edit_singleline(&mut account.name)
                            .labelled_by(name_label.id);
                        ui.add_space(25.0);
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                let app_state_clone = Rc::clone(&self.app_state);
                                let mut app_state = app_state_clone.borrow_mut();
                                if let Some(acc) = &self.selected_account {
                                    app_state.update_account(account.clone());
                                }
                                //let result: Result<&Account, bool> = app_state.get_account_by_id(self.selected_account_id);
                            }
                            ui.add_space(20.0);
                            if ui.button("Cancel").clicked() {
                                self.account_list.selected_account_id = 0;
                                self.show_edit_account = false;
                                self.selected_account = None;
                                self.account_list.transaction_list = TransactionList::from_app_state(Rc::clone(&self.app_state));
                                //self.selected_account = None;
                            }
                        });
                    }
                });
            });
        }

        match &self.account_list.transaction_list.account {
            Some(account) => {
                egui::TopBottomPanel::bottom("transaction_list").show(ctx, |ui| {
                    ui.set_min_height(200.0);
                    self.account_list.transaction_list.ui(ui);
                });
            },
            _ => ()
        }

    }
}

pub struct AccountList {
    app_state: Rc<RefCell<AppState>>,
    filter: String,
    selected_account: Option<Account>,
    selected_account_id: usize,
    transaction_list: TransactionList,
}
impl Page for AccountList
{
    fn from_app_state(app_state: Rc<RefCell<crate::app_state::AppState>>) -> Self {
        Self {
            app_state: Rc::clone(&app_state),
            filter: String::new(),
            selected_account: None,
            selected_account_id: 0,
            transaction_list: TransactionList::from_app_state(Rc::clone(&app_state)),
        }
    }
}
impl AccountList {

    fn ui(&mut self, ui: &mut Ui) -> Response {
        //let app_state = self.app_state.borrow();
        let Self { app_state, filter, selected_account, selected_account_id, transaction_list } = self;
        let app_state_br = app_state.borrow();
        let service_manager = &app_state_br.get_context().service_manager;
        let service = service_manager.accounts_service();
        let repository = service.accounts();


        // let accounts: &Vec<Account> = app_state_br.get_accounts();
        let accounts: Vec<Account> = repository.query().fetch();

        let search_label = ui.label("Search");
        ui.vertical(|ui| {
            ui.text_edit_singleline(filter)
                .labelled_by(search_label.id);
        });
        
        TableBuilder::new(ui)
            .min_scrolled_height(0.0)
            .max_scroll_height(f32::INFINITY)
            .striped(true)
            .sense(egui::Sense::click())
            .id_salt("account_list")
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
                    if filter != "" && !account.name.contains(filter.as_str()) {
                        continue;
                    }
                    body.row(20.0, |mut row| {
                        let mut clicked: bool = false;
                        let (_,response) = row.col(|ui| {
                            ui.label(account.name.to_string());
                        });
                        clicked = clicked || response.clicked();
                        
                        let (_,response) = row.col(|ui| {
                            ui.label(account.get_balance().to_string());
                        });
                        clicked = clicked || response.clicked();
                        
                        let (_,response) = row.col(|ui| {
                            if ui.button("Load").clicked() {
                                println!("Loading {}", account.name);
                            };
                        });

                        if clicked || response.clicked() {
                            *selected_account = Some(account.clone());
                            *transaction_list = TransactionList::from_account(Rc::clone(&app_state), account.clone());
                            *selected_account_id = account.get_id();
                        }
                        //ui.label(format!("${:.2}", account.balance));
                    });
                }
            });


        ui.response()
    }

}

struct TransactionList {
    app_state: Rc<RefCell<AppState>>,
    filter: String,
    account: Option<Account>,

}
impl Page for TransactionList
{
    fn from_app_state(app_state: Rc<RefCell<crate::app_state::AppState>>) -> Self {
        Self {
            app_state: app_state,
            filter: String::new(),
            account: None,
        }
    }
}
impl TransactionList {
    fn from_account(app_state: Rc<RefCell<crate::app_state::AppState>>, account: Account) -> Self {
        Self {
            app_state: app_state,
            filter: String::new(),
            account: Some(account),
        }
    }
    fn ui(&mut self, ui: &mut Ui) -> Response {
        //let app_state = self.app_state.borrow();
        let Self { app_state, filter, account, } = self;
        let mut transactions: Vec<Transaction> = vec!();
        if let Some(acc) = account {
            transactions = acc.get_transactions();
        }

        TableBuilder::new(ui)
            .min_scrolled_height(0.0)
            .max_scroll_height(f32::INFINITY)
            .striped(true)
            .sense(egui::Sense::click())
            .id_salt("account_list")
            .column(Column::initial(100.0).at_least(50.0).clip(true))
            .column(Column::initial(150.0).at_least(50.0).clip(true))
            .column(Column::initial(100.0).at_least(50.0).clip(true))
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.heading("Message");
                });
                header.col(|ui| {
                    ui.heading("Amount");
                });
                header.col(|ui| {
                    ui.heading("Date");
                });
            })
            .body(|mut body| {
                let mut ui = body.ui_mut();
                for tr in &transactions {
                    if filter != "" && !tr.message.contains(filter.as_str()) {
                        continue;
                    }
                    body.row(20.0, |mut row| {
                        let mut clicked: bool = false;
                        let (_,response) = row.col(|ui| {
                            ui.label(tr.message.to_string());
                        });
                        clicked = clicked || response.clicked();
                        
                        let (_,response) = row.col(|ui| {
                            ui.label(tr.amount.to_string());
                        });
                        clicked = clicked || response.clicked();
                        
                        let (_,response) = row.col(|ui| {
                            ui.label("09-03-2025");
        
                        });

                        if clicked || response.clicked() {
                        }
                        //ui.label(format!("${:.2}", account.balance));
                    });
                }
            });
            if transactions.is_empty() {
                ui.label("There's no transactions in this account");
            }
            

        ui.response()
    }
}