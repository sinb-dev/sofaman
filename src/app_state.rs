use crate::{models::accounting::Account, pages::page::Pages};
pub struct AppState {
    is_logged_in: bool,
    page: Pages,
    accounts: Vec<Account>
}
impl AppState {
    pub fn new() -> Self {
        Self {
            is_logged_in : false,
            page : Pages::Login,
            accounts: vec!(Account::new("hoxer"), Account::new("oracin")),
        }
    }
    pub fn logged_in(&mut self) {
        self.is_logged_in = true;
        self.goto_workspace();
    }
    pub fn _is_logged_in(&self) -> bool {
        self.is_logged_in
    }
    pub fn active_page(&self) -> Pages {
        match self.page {
            Pages::Login => Pages::Login,
            Pages::Workspace => Pages::Workspace,
            Pages::Accounts => Pages::Accounts,
        }
    }
    pub fn goto_accounts(&mut self) {
        self.page = Pages::Accounts;
    }
    pub fn goto_workspace(&mut self) {
        self.page = Pages::Workspace;

    }
    pub fn get_accounts(&self) -> &Vec<Account> {
        &self.accounts
    }
}