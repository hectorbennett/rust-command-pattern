use crate::account::Account;

pub struct Deposit<'a> {
    account: &'a mut Account,
    amount: u32,
}

impl Deposit<'_> {
    pub fn new(account: &mut Account, amount: u32) -> Deposit {
        Deposit { account, amount }
    }

    pub fn execute(&mut self) {
        self.account.deposit(self.amount);
        println!(
            "Deposited {} to account: {}",
            self.amount, self.account.name
        );
    }
}

pub struct Withdraw {
    account: Account,
    amount: u32,
}

impl Withdraw {
    pub fn execute(&mut self) {
        self.account.withdraw(self.amount);
        println!(
            "Withdraw {} from account: {}",
            self.amount, self.account.name
        );
    }
}

pub struct Transfer {
    from_account: Account,
    to_account: Account,
    amount: u32,
}

impl Transfer {
    pub fn execute(&mut self) {
        self.from_account.withdraw(self.amount);
        self.to_account.withdraw(self.amount);
        println!(
            "Transfer {} from account: {} to account: {}",
            self.amount, self.from_account.name, self.to_account.name
        );
    }
}
