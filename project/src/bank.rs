use crate::account::Account;
use crate::transaction::Transaction;

pub struct Bank {
    pub name: String,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

impl Bank {
    pub fn new(name: String) -> Self {
        Self { name, accounts: Vec::new(), transactions: Vec::new() }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn print(&self) {
        println!("Bank: {}", self.name);
        println!("Accounts:");
        for i in 0..self.accounts.len() {
            println!("  [{}]", i);
            self.accounts[i].print();
        }
        println!("Transactions:");
        for i in 0..self.transactions.len() {
            print!("  [{}] ", i);
            self.transactions[i].print();
        }
    }

    pub fn find_account_index(&self, number: u32) -> i32 {
        for i in 0..self.accounts.len() {
            if self.accounts[i].number == number {
                return i as i32;
            }
        }
        -1
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: i32) -> bool {
        let mut tx = Transaction::new(from, to, amount);

        let from_idx = self.find_account_index(from);
        let to_idx = self.find_account_index(to);

        if from_idx == -1 || to_idx == -1 {
            self.transactions.push(tx);
            return false;
        }

        let fi = from_idx as usize;
        let ti = to_idx as usize;

        if fi == ti {
            // same account: just check balance
            if self.accounts[fi].balance >= amount {
                tx.approve();
                self.transactions.push(tx);
                return true;
            } else {
                self.transactions.push(tx);
                return false;
            }
        }

        // Borrow two distinct mutable references safely using split_at_mut
        if fi < ti {
            let (left, right) = self.accounts.split_at_mut(ti);
            let from_acc = &mut left[fi];
            let to_acc = &mut right[0];

            if from_acc.balance >= amount {
                from_acc.withdraw(amount);
                to_acc.deposit(amount);
                tx.approve();
                self.transactions.push(tx);
                return true;
            } else {
                self.transactions.push(tx);
                return false;
            }
        } else {
            let (left, right) = self.accounts.split_at_mut(fi);
            let from_acc = &mut right[0];
            let to_acc = &mut left[ti];

            if from_acc.balance >= amount {
                from_acc.withdraw(amount);
                to_acc.deposit(amount);
                tx.approve();
                self.transactions.push(tx);
                return true;
            } else {
                self.transactions.push(tx);
                return false;
            }
        }
    }
}
