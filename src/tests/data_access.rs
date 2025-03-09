use crate::{data_access::ServiceRequest, models::accounting::InMemoryAccountStore, models::accounting::AccountService};


#[test]
fn test_filter() {
    let mut store: InMemoryAccountStore = InMemoryAccountStore::new();
    
    assert_eq!(store.get_accounts().len() , 3);

    assert_eq!(store.filter("x").get_accounts().len(), 1);

}

#[test]
fn test_limit() {
    let mut store: InMemoryAccountStore = InMemoryAccountStore::new();
    
    assert_eq!(store.get_accounts().len() , 3);

    assert_eq!(store.limit(1).get_accounts().len(), 1);
    assert_eq!(store.limit(2).get_accounts().len(), 2);
}

#[test]
fn test_offset() {
    let mut store: InMemoryAccountStore = InMemoryAccountStore::new();
    
    assert_eq!(store.get_accounts().len() , 3);

    let mut result = store.offset(2).limit(1).get_accounts();
    assert_eq!(result[0].name, "Aricane")
    //assert_eq!(store.offset(2).limit(1).get_accounts().len(), 1);
    
}