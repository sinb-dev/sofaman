use crate::data::in_memory::InMemoryServiceLayer;
use crate::{data::sqlite::SqliteServiceLayer, data_access::DataContext};
use crate::accounting::data;
use crate::accounting::models;

fn get_sqlite_context() -> DataContext {
    DataContext {
        services: Box::new(SqliteServiceLayer::new(":memory:"))
    }
}
fn get_in_memory_context() -> DataContext {
    DataContext {
        services: Box::new(InMemoryServiceLayer::new())
    }
}

#[test]
fn sqlite_fetch() {
    let context = get_sqlite_context();
    let service = context.services.accounting();
    
    assert_eq!(service.query().fetch().len(), 4);
}

#[test]
fn sqlite_filter() {
    let context = get_sqlite_context();
    let service = context.services.accounting();
  
    assert_eq!(service.query().fetch().len(), 4);

    assert_eq!(service.query().filter("i").fetch().len(), 3);
}
#[test]
fn sqlite_limit() {
    let context = get_sqlite_context();
    let service = context.services.accounting();
  
    assert_eq!(service.query().fetch().len(), 4);

    assert_eq!(service.query().limit(1).fetch().len(), 1);
    assert_eq!(service.query().limit(2).fetch().len(), 2);
}
#[test]
fn sqlite_offset() {
    let context = get_sqlite_context();
    let service = context.services.accounting();
    
    assert_eq!(service.query().fetch().len() , 4);

    let result = service.query().offset(2).limit(1).fetch();
    assert_eq!(result[0].name, "Charlie")
}

#[test]
fn test_filter() {
    let context = get_in_memory_context();
    let service = context.services.accounting();
  
    assert_eq!(service.query().fetch().len(), 3);

    assert_eq!(service.query().filter("x").fetch().len(), 1);

}

#[test]
fn test_limit() {
    let context = get_in_memory_context();
    let service = context.services.accounting();
    
    assert_eq!(service.query().fetch().len() , 3);

    assert_eq!(service.query().limit(1).fetch().len(), 1);
    assert_eq!(service.query().limit(2).fetch().len(), 2);
}

#[test]
fn test_offset() {
    let context = get_in_memory_context();
    let service = context.services.accounting();
    
    assert_eq!(service.query().fetch().len() , 3);

    let mut result = service.query().offset(2).limit(1).fetch();
    assert_eq!(result[0].name, "Aricane")
}

#[test]
fn test_by_id() {
    let context = get_in_memory_context();
    let service = context.services.accounting();
    
    let oracins_account = service.get_account_by_id(2).unwrap();
    assert_eq!(oracins_account.name, "oracin")
}