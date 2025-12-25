mod customer;
mod account;
mod transaction;
mod bank;

use customer::Customer;
use account::Account;
use bank::Bank;

fn main() {
    // create two customers
    let cust1 = Customer::new(1, String::from("Alice"), 30);
    let cust2 = Customer::new(2, String::from("Bob"), 40);

    // create two accounts with different owners (clone owners when we still need them)
    let acc1 = Account::new(1001, 500, cust1.clone());
    let acc2 = Account::new(1002, 200, cust2.clone());

    // create a bank and add accounts (taking ownership)
    let mut bank = Bank::new(String::from("My Bank"));
    bank.add_account(acc1);
    bank.add_account(acc2);

    println!("Initial state:");
    bank.print();

    // perform a valid transfer
    println!("\nAttempting valid transfer of 100 from 1001 to 1002");
    let ok = bank.transfer(1001, 1002, 100);
    println!("Transfer success: {}", ok);
    bank.print();

    // perform an invalid transfer (too large)
    println!("\nAttempting invalid transfer of 10000 from 1001 to 1002");
    let ok2 = bank.transfer(1001, 1002, 10000);
    println!("Transfer success: {}", ok2);
    bank.print();

    // change ownership of account 1002 to cust1
    println!("\nChanging owner of account 1002 to customer 1 (Alice)");
    let idx = bank.find_account_index(1002);
    if idx != -1 {
        bank.accounts[idx as usize].change_owner(cust1.clone());
    }
    bank.print();
}
