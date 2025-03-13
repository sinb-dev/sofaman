use crate::data_access::{DataContext, Repository, RequestParameters, Service, ServiceQuery};
use crate::accounting::models::Account;
pub trait AccountService<Q> {
    fn accounts(&self) -> Box<dyn Repository<Q>>;
    fn get_account_by_id(&self, id: u32) -> Account;
}

impl DataContext {
    // pub fn account_service(&self); -> impl ServiceQuery<Account> {
        
    // }
} 