use crate::{accounting::data::AccountService, data::{sqlite::{SqliteAccountRepository, SqliteAccountService, SqliteServiceManager}, InMemory::{InMemoryAccountRepository, InMemoryAccountService}}, data_access::{ServiceManager, ServiceQuery}};
use crate::data_access::Repository;
use crate::accounting::data;
use crate::accounting::models;

#[test]
fn sqlite_fetch() {
    let service_manager = SqliteServiceManager::new();
    let service = service_manager.accounts_service();

    let repo = service.accounts();
    let hmm = repo.query();
    assert_eq!(repo.query().fetch().len(), 4);
}

#[test]
fn sqlite_filter() {
    let service_manager = SqliteServiceManager::new();
    let service = service_manager.accounts_service();

    let repo = service.accounts();
  
    assert_eq!(repo.query().fetch().len(), 4);

    assert_eq!(repo.query().filter("i").fetch().len(), 3);
}
#[test]
fn sqlite_limit() {
    let service_manager = SqliteServiceManager::new();
    let service = service_manager.accounts_service();

    let repo = service.accounts();
  
    assert_eq!(repo.query().fetch().len(), 4);

    assert_eq!(repo.query().limit(1).fetch().len(), 1);
    assert_eq!(repo.query().limit(2).fetch().len(), 2);
}
#[test]
fn sqlite_offset() {
    let service_manager = SqliteServiceManager::new();
    let service = service_manager.accounts_service();
    let repo = service.accounts();
    
    assert_eq!(repo.query().fetch().len() , 4);

    let result = repo.query().offset(2).limit(1).fetch();
    assert_eq!(result[0].name, "Charlie")
}

#[test]
fn test_filter() {
    let repo: InMemoryAccountRepository = InMemoryAccountRepository::new();
  
    assert_eq!(repo.query().fetch().len(), 3);

    assert_eq!(repo.query().filter("x").fetch().len(), 1);

}

#[test]
fn test_limit() {
    let repo: InMemoryAccountRepository = InMemoryAccountRepository::new();
    
    assert_eq!(repo.query().fetch().len() , 3);

    assert_eq!(repo.query().limit(1).fetch().len(), 1);
    assert_eq!(repo.query().limit(2).fetch().len(), 2);
}

#[test]
fn test_offset() {
    let repo: InMemoryAccountRepository = InMemoryAccountRepository::new();
    
    assert_eq!(repo.query().fetch().len() , 3);

    let mut result = repo.query().offset(2).limit(1).fetch();
    assert_eq!(result[0].name, "Aricane")
}

#[test]
fn test_by_id() {
    let repo: InMemoryAccountRepository = InMemoryAccountRepository::new();
    
    let oracins_account = repo.query().with_id(2).unwrap();
    assert_eq!(oracins_account.name, "oracin")
}