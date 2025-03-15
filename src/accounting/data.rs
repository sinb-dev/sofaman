use crate::data_access::ServiceQuery;

use super::models::Account;


pub trait AccountService {
    fn get_account_by_id(&self, id: i64) -> Option<Account>;
    fn get_accounts(&self) -> Vec<Account>;
    //fn query(&self, query: AccountServiceQuery) -> Result<Vec<Account>, String>;
    fn query(&self) -> Box<dyn AccountServiceQuery>;
}

pub trait AccountServiceQuery {
    fn filter(&self, value: &str) -> Box<dyn AccountServiceQuery>;
    fn limit(&self, limit: usize) -> Box<dyn AccountServiceQuery>;
    fn offset(&self, offset: usize) -> Box<dyn AccountServiceQuery>;
    fn fetch(&self) -> Vec<Account>;
    //fn with_id(self, id: usize) -> Option<M>;
    // fn filter(self, value: &str) -> Self where Self : Sized;
    // fn limit(self, limit: usize) -> Self where Self : Sized;
    // fn offset(self, offset: usize) -> Self where Self : Sized;
    // fn fetch(self) -> Vec<Account>;
}
