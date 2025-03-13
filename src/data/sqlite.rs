use sqlite::{Connection, State};
use std::rc::Rc;
use crate::data_access::{DataContext, Repository, RequestParameters, Service, ServiceManager, ServiceQuery, ServiceQuery2, ServiceQuery2Trait};
use crate::accounting::data::AccountService;
use crate::accounting::models::Account;
pub struct SqliteServiceManager {
    connection: Rc<Connection>
}
impl SqliteServiceManager {
    pub fn new() -> Self {
        let connection = sqlite::open(":memory:").unwrap();

        let query = "
            CREATE TABLE accounts (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, balance REAL);
            INSERT INTO accounts (name, balance) VALUES ('Alice', 42),
                ('Bob', 69),
                ('Charlie', 42),
                ('Dennis', 69);
        ";
        connection.execute(query).unwrap();
        Self {
            connection: Rc::new(connection)
        }
    }
    // pub fn account_service(&self) -> SqliteAccountService{
    //      SqliteAccountService::new(Rc::clone(&self.connection))
    // }
}
impl ServiceManager for SqliteServiceManager {
    fn accounts_service(&self) -> Box<dyn AccountService<dyn ServiceQuery2Trait>> {
        Box::new(SqliteAccountService::new(Rc::clone(&self.connection)))
    }
}

impl DataContext {
    /*pub fn account_service(&self) -> impl AccountService<Account> {
        self.service_manager.accounts_service()
    }*/
}

pub struct SqliteAccountService {
    connection: Rc<Connection>
}
impl SqliteAccountService {
    pub fn new(conn: Rc<Connection>) -> Self {

        Self {
            connection: conn
        }
    }
}
impl Service for SqliteAccountService {

}
impl AccountService<SqliteAccountQuery> for SqliteAccountService {
    fn get_account_by_id(&self, id: u32) -> Account {
        todo!()
    }
    
    fn accounts(&self) -> Box<dyn Repository<SqliteAccountQuery>>  {
        Box::new(SqliteAccountRepository::new(Rc::clone(&self.connection)))
    }

}

pub struct SqliteAccountRepository  {
    connection: Rc<Connection>
    
}

 impl Repository<SqliteAccountQuery> for SqliteAccountRepository {
    fn query(&self) -> SqliteAccountQuery {
        SqliteAccountQuery {
            connection: Rc::clone(&self.connection),
            base: ServiceQuery2 { request: RequestParameters::new() }
        }
    }
}

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

pub struct SqliteAccountQuery {
    //request: RequestParameters,
    base: ServiceQuery2,
    connection: Rc<Connection>
}
impl ServiceQuery2Trait for SqliteAccountQuery
{
    fn get_base(&self) -> &ServiceQuery2 {
        return &self.base;
    } 
}
impl SqliteAccountQuery {
    pub fn filter(mut self, value: &str) -> Self {
        self.base.request.filter = Some(String::from(value));
        self
    }
    pub fn limit(mut self, limit: usize) -> Self {
        self.base.request.limit = limit;
        self
    }
    pub fn offset(mut self, offset: usize) -> Self {
        self.base.request.offset = offset;
        self
    }
    pub fn fetch(mut self) -> Vec<Account> {

        let mut result: Vec<Account> = Vec::new();

        let mut name_filter = String::new();
        let mut query = String::new();

        //Set filter
        if let Some(filter) = &self.base.request.filter {
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
        statement.bind((":offset", self.base.request.offset as i64)).unwrap();
        statement.bind((":limit", self.base.request.limit as i64)).unwrap();
        
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
    fn with_id(mut self, id: usize) -> Option<Account> {
        None
        // let result: Vec<Account> = self.service.accounts.iter()
        //     .filter(|account| account.get_id() == id)
        //     .skip(self.request.offset)
        //     .take(self.request.limit)
        //     .cloned()
        //     .collect();
        // if let Some(account) = result.first() {
        //     Some(account.clone())
        // } else {
        //     None
        // }
    }
}
impl SqliteAccountRepository {
    pub fn new(conn: Rc<Connection>) -> Self {
        
        Self {
            connection: conn,
        }
    }
    pub fn get_connection(&self) -> &Connection {
        &self.connection
    }
}