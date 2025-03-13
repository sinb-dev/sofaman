// use crate::data_access::{Repository, RequestParameters, Service, ServiceQuery};
// use crate::accounting::data::AccountService;
// use crate::accounting::models::{Account};

// pub struct InMemoryAccountService {
    
// }

// impl Service for InMemoryAccountService {

// }
// impl AccountService for InMemoryAccountService {
//     fn get_account_by_id(&self, id: u32) -> Account {
//         todo!()
//     }
    
//     fn accounts(&self) -> impl Repository<Account>  {
//         InMemoryAccountRepository::new()
//     }

// }

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