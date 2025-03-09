use crate::data_access::{RequestParameters, ServiceRequest};

#[derive(Clone)]
pub struct Account {
    id: u32,
    pub name: String,
    transactions: Vec<Transaction>,
    closed_date: i32,
    pub balance: i32,
}
impl Account {
    pub fn new(id: u32, name: &str) -> Self {
        Self {
            id: id,
            name : name.to_string(),
            transactions : Vec::new(),
            closed_date : 0,
            balance: 0,
        }
    }
    
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn deposit(&mut self, message: &str, amount: u32) -> Result<bool, &str> {
        
        if self.closed_date > 0 {
            Err("Cannot deposit: Account is closed")
        } else {

            self.transactions.push( Transaction {
                message : message.to_string(),
                amount : amount as i32
            });
            self.calc_balance();
            Ok(true)
        }
    }
    pub fn withdraw(&mut self, message: &str, amount: u32) -> Result<bool, &str> {
        let i_amount = amount as i32;
        if self.closed_date > 0 {
            Err("Cannot withdraw: Account is closed")
        } else if i_amount > self.balance {
            Err("Balance too low")
        } else {
            self.transactions.push( Transaction {
                message : message.to_string(),
                amount : -i_amount
            });
            self.calc_balance();
            Ok(true)
        }
    }
    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn close_account(&mut self) {
        self.closed_date = 1;
    }
    pub fn get_transactions(&self) -> Vec<Transaction>{
        self.transactions.to_vec()
    }
    fn calc_balance(&mut self) {
        let mut new_balance = 0;
        for tr in &self.transactions {
            new_balance += tr.amount;
        }
        self.balance = new_balance;
    }
}

#[derive(Clone)]
pub struct Transaction {
    pub message: String,
    pub amount: i32
}
/////////////////////////////

pub trait AccountService : ServiceRequest {
    fn get_accounts(&self) -> Vec<Account>;
    fn get_account_by_id(&self, id: u32) -> Account;
}

pub struct InMemoryAccountStore {
    request: RequestParameters,
    accounts: Vec<Account>
}
impl InMemoryAccountStore {
    pub fn new() -> Self {
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
            request: RequestParameters::new(),
            accounts: vec!(Account::new(1, "hoxer"), Account::new(2, "oracin"), aricane_account),
        }
    }
}
impl ServiceRequest for InMemoryAccountStore {
    fn filter(&mut self, value: &str) -> &mut Self {
        self.request.filter = Some(String::from(value));
        self
    }
    fn limit(&mut self, limit: usize) -> &mut Self {
        self.request.limit = limit;
        self
    }
    fn offset(&mut self, offset: usize) -> &mut Self {
        self.request.offset = offset;
        self
    }
}
impl AccountService for InMemoryAccountStore {
    
    fn get_accounts(&self) -> Vec<Account>
    {
        if let Some(filter) = &self.request.filter {
            self.accounts.iter()
                .filter(|account| account.name.contains(filter))
                .skip(self.request.offset)
                .take(self.request.limit)
                .cloned()
                .collect()
        } else {
            self.accounts.iter()
                .skip(self.request.offset)
                .take(self.request.limit)
                .cloned()
                .collect()
        }
    }
    fn get_account_by_id(&self, id: u32) -> Account {
        todo!()
    }
}

