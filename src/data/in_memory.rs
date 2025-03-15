// use crate::data_access::{Repository, RequestParameters, Service, ServiceQuery};
// use crate::accounting::data::AccountService;
// use crate::accounting::models::{Account};

use crate::{accounting::{data::{AccountService, AccountServiceQuery}, models::Account}, data_access::{ServiceLayer, ServiceQuery}};
use std::rc::Rc;
use super::sqlite::SqliteAccountQuery;
#[derive(Clone)]
struct DataSource {
    accounts: Vec<Account>
}

pub struct InMemoryServiceLayer {
    account_service: Box<dyn AccountService>
}
impl InMemoryServiceLayer {
    pub fn new() -> Self {
        Self {
            account_service: Box::new(InMemoryAccountService::new())
        }
    }
}
impl ServiceLayer for InMemoryServiceLayer {
    fn accounting(&self) -> &Box<dyn AccountService> {
        &self.account_service
    }
}
pub struct InMemoryAccountService {
    data_source: DataSource
}

impl InMemoryAccountService {
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
            data_source: DataSource {
                accounts: vec!(Account::new(1, "hoxer"), Account::new(2, "oracin"), aricane_account)
            },
        }
    }
}

impl AccountService for InMemoryAccountService {
    fn get_account_by_id(&self, id: i64) -> Option<Account> {
        let result: Vec<Account> = self.data_source.accounts.iter()
            .filter(|account| account.get_id() == id as usize)
            .cloned()
            .collect();
        if let Some(account) = result.first() {
            Some(account.clone())
        } else {
            None
        }
    }

    fn get_accounts(&self) -> Vec<Account> {
        return self.query().fetch();
    }

    fn query(&self) -> Box<dyn crate::accounting::data::AccountServiceQuery> {
        Box::new(InMemoryAccountQuery::new(Rc::new(self.data_source.clone())))
    }
}

pub struct InMemoryAccountQuery {
    data_source: Rc<DataSource>,
    base: ServiceQuery
}
impl InMemoryAccountQuery {
    pub fn new(data_source: Rc<DataSource>) -> Self {
        Self {
            data_source: Rc::clone(&data_source),
            base: ServiceQuery::new()
        }
    }
}
impl AccountServiceQuery for InMemoryAccountQuery {
    fn filter(&self, value: &str) -> Box<dyn AccountServiceQuery> {
        Box::new(Self {
            data_source: Rc::clone(&self.data_source),
            base: ServiceQuery {
                filter: Some(String::from(value)),
                limit: self.base.limit,
                offset: self.base.offset
            }
        })
    }
    fn limit(&self, limit: usize) -> Box<dyn AccountServiceQuery> {
        Box::new(Self {
            data_source: Rc::clone(&self.data_source),
            base: ServiceQuery {
                filter: self.base.filter.clone(),
                limit: limit,
                offset: self.base.offset
            }
        })
    }
    fn offset(&self, offset: usize) -> Box<dyn AccountServiceQuery> {
        Box::new(Self {
            data_source: Rc::clone(&self.data_source),
            base: ServiceQuery {
                filter: self.base.filter.clone(),
                limit: self.base.limit,
                offset: offset
            }
        })
    }

    fn fetch(&self) -> Vec<Account> {
        if let Some(filter) = &self.base.filter {
            self.data_source.accounts.iter()
                .filter(|account| account.name.contains(filter))
                .skip(self.base.offset)
                .take(self.base.limit)
                .cloned()
                .collect()
        } else {
            self.data_source.accounts.iter()
                .skip(self.base.offset)
                .take(self.base.limit)
                .cloned()
                .collect()
        }
    }
}

// //////////////////////////
// pub struct InMemoryAccountRepository  {
//     accounts: Vec<Account>
// }

// impl Repository<Account> for InMemoryAccountRepository {
//     fn query(&self) -> impl ServiceQuery<Account> {
//         InMemoryAccountQuery {
//             request: RequestParameters::new(),
//             service: self
//         }
//     }
// }

// pub struct InMemoryAccountQuery<'a> {
//     request: RequestParameters,
//     service: &'a InMemoryAccountRepository,
// }
// impl<'a> ServiceQuery<Account> for InMemoryAccountQuery<'a> {
//     fn filter(mut self, value: &str) -> Self {
//         self.request.filter = Some(String::from(value));
//         self
//     }
//     fn limit(mut self, limit: usize) -> Self {
//         self.request.limit = limit;
//         self
//     }
//     fn offset(mut self, offset: usize) -> Self {
//         self.request.offset = offset;
//         self
//     }
//     fn fetch(mut self) -> Vec<Account> {
//         if let Some(filter) = &self.request.filter {
//             self.service.accounts.iter()
//                 .filter(|account| account.name.contains(filter))
//                 .skip(self.request.offset)
//                 .take(self.request.limit)
//                 .cloned()
//                 .collect()
//         } else {
//             self.service.accounts.iter()
//                 .skip(self.request.offset)
//                 .take(self.request.limit)
//                 .cloned()
//                 .collect()
//         }
//     }
//     fn with_id(mut self, id: usize) -> Option<Account> {
//         let result: Vec<Account> = self.service.accounts.iter()
//             .filter(|account| account.get_id() == id)
//             .skip(self.request.offset)
//             .take(self.request.limit)
//             .cloned()
//             .collect();
//         if let Some(account) = result.first() {
//             Some(account.clone())
//         } else {
//             None
//         }
//     }
// }
// impl InMemoryAccountRepository {
//     pub fn new() -> Self {
//         let mut aricane_account = Account::new(3, "Aricane");
//         aricane_account.deposit("Initial", 1000);
//         aricane_account.deposit("Bonus", 1000);
//         aricane_account.withdraw("Favors", 699);
//         aricane_account.deposit("Initial", 1000);
//         aricane_account.deposit("Bonus", 1000);
//         aricane_account.withdraw("Favors", 699);
//         aricane_account.deposit("Initial", 1000);
//         aricane_account.deposit("Bonus", 1000);
//         aricane_account.withdraw("Favors", 699);
//         aricane_account.deposit("Initial", 1000);
//         aricane_account.deposit("Bonus", 1000);
//         aricane_account.withdraw("Favors", 699);
//         aricane_account.deposit("Initial", 1000);
//         aricane_account.deposit("Bonus", 1000);
//         aricane_account.withdraw("Favors", 699);

//         Self {
//             accounts: vec!(Account::new(1, "hoxer"), Account::new(2, "oracin"), aricane_account),
//         }
//     }
// }