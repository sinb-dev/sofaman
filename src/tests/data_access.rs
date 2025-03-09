use crate::{data_access::ServiceQuery, models::accounting::{AccountService, InMemoryAccountStore, InMemoryAccountStoreQuery}};


#[test]
fn test_filter() {
    let store: InMemoryAccountStore = InMemoryAccountStore::new();
    
    assert_eq!(store.query().fetch().len() , 3);

    assert_eq!(store.query().filter("x").fetch().len(), 1);

}

#[test]
fn test_limit() {
    let store: InMemoryAccountStore = InMemoryAccountStore::new();
    
    assert_eq!(store.query().fetch().len() , 3);

    assert_eq!(store.query().limit(1).fetch().len(), 1);
    assert_eq!(store.query().limit(2).fetch().len(), 2);
}

#[test]
fn test_offset() {
    let store: InMemoryAccountStore = InMemoryAccountStore::new();
    
    assert_eq!(store.query().fetch().len() , 3);

    let mut result = store.query().offset(2).limit(1).fetch();
    assert_eq!(result[0].name, "Aricane")
}

#[test]
fn test_by_id() {
    let store: InMemoryAccountStore = InMemoryAccountStore::new();
    
    let oracins_account = store.query().with_id(2).unwrap();
    assert_eq!(oracins_account.name, "oracin")
}