use crate::{accounting::models::{Account, Transaction}, data_access::DataContext, pages::page::Pages};
pub struct ModelBuffer {
    pub account: Option<Account>,
    pub transaction: Option<Transaction>
}
pub struct AppState {
    is_logged_in: bool,
    page: Pages,
    accounts: Vec<Account>,
    context: DataContext,
    pub buffer: ModelBuffer
}
impl AppState {
    pub fn new(context: DataContext) -> Self {
        let mut aricane_account = Account::new(3, "Aricane");
        aricane_account.deposit("Initial", 1000);
        aricane_account.deposit("Bonus", 1000);
        aricane_account.withdraw("Favors", 699);
        aricane_account.deposit("Initial", 1000);
        aricane_account.deposit("Bonus", 1000);
        aricane_account.withdraw("Favors", 699);
        aricane_account.deposit("Initial", 1000);
        aricane_account.deposit("Bonus", 1000);
        aricane_account.withdraw("Favors", 699);
        aricane_account.deposit("Initial", 1000);
        aricane_account.deposit("Bonus", 1000);
        aricane_account.withdraw("Favors", 699);
        aricane_account.deposit("Initial", 1000);
        aricane_account.deposit("Bonus", 1000);
        aricane_account.withdraw("Favors", 699);
        Self {
            is_logged_in : false,
            page : Pages::Login,
            accounts: vec!(Account::new(1, "hoxer"), Account::new(2, "oracin"), aricane_account),
            context: context,
            buffer: ModelBuffer {
                account: None,
                transaction: None,
            }
        }
    }
    pub fn get_context(&self) -> &DataContext {
        &self.context
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
    pub fn get_account_by_id(&self, id: usize) -> Result<&Account,bool> {
        for account in &self.accounts {
            if account.get_id() == id {
                return Ok(&account);
            }
        }
        Err(false)
    }
    pub fn get_account_by_id_mut(&mut self, id: usize) -> Result<Account,bool> {
        let mut index : usize = 0;
        let mut success: bool = false;
        for account in &self.accounts {
            if account.get_id() == id {
                success = true;
                break;
            }
            index += 1;
        }
        if success {
            let account_clone: Account = self.accounts[index].clone();
            return Ok(account_clone);
        }
        Err(false)
    }
    pub fn update_account(&mut self, updated_account: Account) {
        println!("Account: {}", updated_account.name);
        let mut index: usize = 0;
        let mut success: bool = false;
        for acc in &self.accounts {
            if acc.get_id() == updated_account.get_id() {
                success = true;
                break;
            }
            index += 1;
        }
        if success {
            self.accounts[index] = updated_account;
        }
    }
}