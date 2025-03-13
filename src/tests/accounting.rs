use crate::accounting::models::Account;

#[test]
fn test_balance() {

    let mut account = Account::new(1,"test");

    assert_eq!(account.get_balance(), 0);

    let _ = account.deposit("Salary", 1000);
    assert_eq!(account.get_balance(), 1000);

    _ = account.deposit("Bonus", 500);
    assert_eq!(account.get_balance(), 1500);

    _ = account.withdraw("Rent", 550);
    assert_eq!(account.get_balance(), 950);

}
#[test]
fn test_withdraw() {
    let mut account = Account::new(1,"test");

    assert_eq!(account.get_balance(), 0);
    match account.withdraw("Too much", 1000) {
        Err(msg) => assert_eq!(msg, "Balance too low"),
        Ok(_) => assert!(false, "Should not be able to withdraw 1000. The account balance is 0")
    }

    let _ = account.deposit("Salary", 1000);
    account.close_account();
    match account.withdraw("Bonus", 500) {
        Ok(_) => assert!(false, "Account closed, should not be able to deposit"),
        Err(msg) => assert_eq!(msg, "Cannot withdraw: Account is closed")
    }
}
#[test]
fn test_deposit() {
    let mut account = Account::new(1,"test");

    assert_eq!(account.get_balance(), 0);
    match account.deposit("Salary", 1000) {
        Ok(_) => assert_eq!(account.get_balance(), 1000),
        Err(_) => assert!(false, "Failed to deposit")
    }

    account.close_account();
    match account.deposit("Bonus", 500) {
        Ok(_) => assert!(false, "Account closed, should not be able to deposit"),
        Err(msg) => assert_eq!(msg, "Cannot deposit: Account is closed")
    }


}