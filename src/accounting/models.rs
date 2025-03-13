#[derive(Clone)]
pub struct Account {
    id: usize,
    pub name: String,
    transactions: Vec<Transaction>,
    closed_date: i32,
    pub balance: i32,
}
impl Account {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id: id,
            name : name.to_string(),
            transactions : Vec::new(),
            closed_date : 0,
            balance: 0,
        }
    }
    
    pub fn get_id(&self) -> usize {
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
        } else {pub trait AccountService<'a, Q> {
            fn query(&'a self) -> Q;
            fn get_account_by_id(&self, id: u32) -> Account;
        }
        
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