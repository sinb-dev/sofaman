use sqlite::{Connection, State};
use std::rc::Rc;
use crate::accounting::data::{AccountService, AccountServiceQuery};
use crate::accounting::models::Account;
use crate::data_access::{RequestParameters, ServiceLayer, ServiceQuery};

pub struct SqliteServiceLayer
{
    connection: Rc<Connection>,
    account_service: Box<dyn AccountService>
}

impl SqliteServiceLayer {
    pub fn new(connection_string: &str) -> Self {
        let connection = sqlite::open(connection_string).unwrap();

        let query = "
            CREATE TABLE accounts (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, balance REAL);
            INSERT INTO accounts (name, balance) VALUES ('Alice', 42),
                ('Bob', 69),
                ('Charlie', 42),
                ('Dennis', 69);
        ";
        connection.execute(query).unwrap();
        let rc_connection = Rc::new(connection);
        Self {
            connection: Rc::clone(&rc_connection),
            account_service: Box::new(SqliteAccountService::new(Rc::clone(&rc_connection)))
        }
    }
}

impl ServiceLayer for SqliteServiceLayer {
    fn accounting(&self) -> &Box<dyn AccountService> {
        &self.account_service
    }
}



pub struct SqliteAccountService {
    connection: Rc<Connection>
}
impl SqliteAccountService {
    pub fn new(connection: Rc<Connection>) -> Self {
        Self {
            connection: connection
        }
    }
}
impl AccountService for SqliteAccountService {
    fn get_account_by_id(&self, id: i64) -> Option<Account> {
        let query = "SELECT id, name, balance FROM accounts WHERE id = :account_id";
        let mut statement = self.connection.prepare(query).unwrap();
        statement.bind((":account_id", id));
        if let Ok(State::Row) = statement.next() {
            let name: String = statement.read::<String, _>("name").unwrap();
            let mut account = Account::new(
                statement.read::<i64, _>("id").unwrap() as usize,
                &name
            );
            account.balance = statement.read::<i64, _>("id").unwrap() as i32;
            return Some(account);
        }
        None
    }
    
    fn get_accounts(&self) -> Vec<Account> {
        self.query().fetch()
    }

    fn query(&self) -> Box<dyn AccountServiceQuery> {
        Box::new(SqliteAccountQuery {
            connection: Rc::clone(&self.connection),
            base: ServiceQuery::new()
        })
    }
}

pub struct SqliteAccountQuery {
    base: ServiceQuery,
    connection: Rc<Connection>
}
impl SqliteAccountQuery {
    pub fn new(connection: Rc<Connection>) -> Self {
        Self {
            connection: Rc::clone(&connection),
            base: ServiceQuery::new()
        }
    }
}
impl AccountServiceQuery for SqliteAccountQuery {
    fn filter(&self, value: &str) -> Box<dyn AccountServiceQuery> {
        Box::new(Self {
            connection: Rc::clone(&self.connection),
            base: ServiceQuery {
                filter: Some(String::from(value)),
                limit: self.base.limit,
                offset: self.base.offset
            }
        })
    }
    fn limit(&self, limit: usize) -> Box<dyn AccountServiceQuery> {
        Box::new(Self {
            connection: Rc::clone(&self.connection),
            base: ServiceQuery {
                filter: self.base.filter.clone(),
                limit: limit,
                offset: self.base.offset
            }
        })
    }
    fn offset(&self, offset: usize) -> Box<dyn AccountServiceQuery> {
        Box::new(Self {
            connection: Rc::clone(&self.connection),
            base: ServiceQuery {
                filter: self.base.filter.clone(),
                limit: self.base.limit,
                offset: offset
            }
        })
    }
    fn fetch(&self) -> Vec<Account> {
        let mut result: Vec<Account> = Vec::new();

        let mut name_filter = String::new();
        let mut query = String::new();

        //Set filter
        if let Some(filter) = &self.base.filter {
            query = String::from("SELECT id,name,balance FROM accounts WHERE name LIKE :filter");
            name_filter = format!("%{}%", filter);
        } else {
            query = String::from("SELECT id,name,balance FROM accounts");
        }

        //Add limit and offset
        query = format!("{} LIMIT :offset, :limit", query);

        //Create statement
        let mut statement = self.connection.prepare(query).unwrap();
        if name_filter != "" {
            statement.bind((":filter", name_filter.as_str())).unwrap();
        }
        statement.bind((":offset", self.base.offset as i64)).unwrap();
        statement.bind((":limit", self.base.limit as i64)).unwrap();

        while let Ok(State::Row) = statement.next() {
            let name: String = statement.read::<String, _>("name").unwrap();
            let mut account = Account::new(
                statement.read::<i64, _>("id").unwrap() as usize,
                &name
            );
            account.balance = statement.read::<i64, _>("id").unwrap() as i32;

            result.push(account);
        }
        return result;
    }
    
}
///////////////////////////////////
// pub struct SqliteAccountService {
//     connection: Rc<Connection>
// }
// impl SqliteAccountService {
//     pub fn new(conn: Rc<Connection>) -> Self {

//         Self {
//             connection: conn
//         }
//     }
// }
// impl Service for SqliteAccountService {

// }
// impl AccountService<SqliteAccountQuery> for SqliteAccountService {
//     fn get_account_by_id(&self, id: u32) -> Account {
//         todo!()
//     }
    
//     fn accounts(&self) -> Box<dyn Repository<SqliteAccountQuery>>  {
//         Box::new(SqliteAccountRepository::new(Rc::clone(&self.connection)))
//     }

// }

// pub struct SqliteAccountRepository  {
//     connection: Rc<Connection>
    
// }

//  impl Repository<SqliteAccountQuery> for SqliteAccountRepository {
//     fn query(&self) -> SqliteAccountQuery {
//         SqliteAccountQuery {
//             connection: Rc::clone(&self.connection),
//             base: ServiceQuery2 { request: RequestParameters::new() }
//         }
//     }
// }

// pub struct SqliteServiceQuery {
//     base: ServiceQuery2,
//     connection: Rc<Connection>
// }
// impl SqliteServiceQuery {
//     fn filter(mut self, value: &str) -> Self {
//         self.base.request.filter = Some(String::from(value));
//         self
//     }
//     fn limit(mut self, limit: usize) -> Self {
//         self.base.request.limit = limit;
//         self
//     }
//     fn offset(mut self, offset: usize) -> Self {
//         self.base.request.offset = offset;
//         self
//     }
//     fn fetch(mut self) -> Vec<Account> {

//         let mut result: Vec<Account> = Vec::new();

//         let mut name_filter = String::new();
//         let mut query = String::new();

//         //Set filter
//         if let Some(filter) = &self.base.request.filter {
//             query = String::from("SELECT id,name,balance FROM accounts WHERE name LIKE :filter");
//             name_filter = format!("%{}%", filter);
//         } else {
//             query = String::from("SELECT id,name,balance FROM accounts");
//         }

//         //Add limit and offset
//         query = format!("{} LIMIT :offset, :limit", query);

//         //Create statement
//         let mut statement = self.connection.prepare(query).unwrap();
//         if name_filter != "" {
//             statement.bind((":filter", name_filter.as_str())).unwrap();
//         }
//         statement.bind((":offset", self.base.request.offset as i64)).unwrap();
//         statement.bind((":limit", self.base.request.limit as i64)).unwrap();
        
//         while let Ok(State::Row) = statement.next() {
//             let name: String = statement.read::<String, _>("name").unwrap();
//             let mut account = Account::new(
//                 statement.read::<i64, _>("id").unwrap() as usize,
//                 &name
//             );
//             account.balance = statement.read::<i64, _>("id").unwrap() as i32;

//             result.push(account);
//         }
//         return result;
        
//     }
// }

// pub struct SqliteAccountQuery {
//     //request: RequestParameters,
//     base: ServiceQuery2,
//     connection: Rc<Connection>
// }
// impl ServiceQuery2Trait for SqliteAccountQuery
// {
//     fn get_base(&self) -> &ServiceQuery2 {
//         return &self.base;
//     } 
// }
// impl SqliteAccountQuery {
//     pub fn filter(mut self, value: &str) -> Self {
//         self.base.request.filter = Some(String::from(value));
//         self
//     }
//     pub fn limit(mut self, limit: usize) -> Self {
//         self.base.request.limit = limit;
//         self
//     }
//     pub fn offset(mut self, offset: usize) -> Self {
//         self.base.request.offset = offset;
//         self
//     }
//     pub fn fetch(mut self) -> Vec<Account> {

//         let mut result: Vec<Account> = Vec::new();

//         let mut name_filter = String::new();
//         let mut query = String::new();

//         //Set filter
//         if let Some(filter) = &self.base.request.filter {
//             query = String::from("SELECT id,name,balance FROM accounts WHERE name LIKE :filter");
//             name_filter = format!("%{}%", filter);
//         } else {
//             query = String::from("SELECT id,name,balance FROM accounts");
//         }

//         //Add limit and offset
//         query = format!("{} LIMIT :offset, :limit", query);

//         //Create statement
//         let mut statement = self.connection.prepare(query).unwrap();
//         if name_filter != "" {
//             statement.bind((":filter", name_filter.as_str())).unwrap();
//         }
//         statement.bind((":offset", self.base.request.offset as i64)).unwrap();
//         statement.bind((":limit", self.base.request.limit as i64)).unwrap();
        
//         while let Ok(State::Row) = statement.next() {
//             let name: String = statement.read::<String, _>("name").unwrap();
//             let mut account = Account::new(
//                 statement.read::<i64, _>("id").unwrap() as usize,
//                 &name
//             );
//             account.balance = statement.read::<i64, _>("id").unwrap() as i32;

//             result.push(account);
//         }
//         return result;
        
//     }
//     fn with_id(mut self, id: usize) -> Option<Account> {
//         None
//         // let result: Vec<Account> = self.service.accounts.iter()
//         //     .filter(|account| account.get_id() == id)
//         //     .skip(self.request.offset)
//         //     .take(self.request.limit)
//         //     .cloned()
//         //     .collect();
//         // if let Some(account) = result.first() {
//         //     Some(account.clone())
//         // } else {
//         //     None
//         // }
//     }
// }
// impl SqliteAccountRepository {
//     pub fn new(conn: Rc<Connection>) -> Self {
        
//         Self {
//             connection: conn,
//         }
//     }
//     pub fn get_connection(&self) -> &Connection {
//         &self.connection
//     }
// }