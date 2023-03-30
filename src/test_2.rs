// use account::Account;
// use bank::Bank;
// // use commands::Deposit;
// // use controller::BankController;

// pub mod account;
// pub mod bank;
// // pub mod commands;
// // pub mod controller;
// // pub mod transaction;

pub trait Transaction {
    fn execute(&mut self) -> &str;
    fn rollback(&mut self) -> &str;
}

pub struct Deposit<'a> {
    account: &'a mut Account,
    amount: u32,
}

impl Deposit<'_> {
    pub fn new(account: &mut Account, amount: u32) -> Deposit {
        Deposit { account, amount }
    }
}

impl Transaction for Deposit<'_> {
    fn execute(&mut self) -> &str {
        self.account.deposit(self.amount);
        println!(
            "Deposited {} to account: {}",
            self.amount, self.account.name
        );
        "create table"
    }
    fn rollback(&mut self) -> &str {
        "drop table"
    }
}

pub struct Withdraw;
impl Transaction for Withdraw {
    fn execute(&mut self) -> &str {
        println!("Withdraw");
        "add field"
    }
    fn rollback(&mut self) -> &str {
        "remove field"
    }
}

struct BankController {
    commands: Vec<Box<dyn Transaction>>,
}

impl BankController {
    fn new() -> Self {
        Self { commands: vec![] }
    }

    fn add_Transaction(&mut self, cmd: Box<dyn Transaction>) {
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }
    fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev() // reverse iterator's direction
            .map(|cmd| cmd.rollback())
            .collect()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut controller = BankController::new();

    let hector = bank.create_account("Hector");

    controller.add_Transaction(Box::new(Deposit::new(hector, 1000)));
    controller.add_Transaction(Box::new(Withdraw));

    assert_eq!(vec!["create table", "add field"], controller.execute());
    assert_eq!(vec!["remove field", "drop table"], controller.rollback());
}

// fn main() {
//     let mut bank = Bank::new();
//     // let controller = BankController::new();

//     let hector = bank.create_account("Hector");
//     // hector.deposit(10000);
//     controller.execute(Deposit::new(hector, 10000));

//     let hayley = bank.create_account("Hayley");
//     hayley.deposit(10000);

//     let rich = bank.create_account("Rich");
//     rich.deposit(10000);

//     let elliot = bank.create_account("Elliot");
//     elliot.deposit(100000);

//     // rich.withdraw(500);
//     // hayley.deposit(500);
// }
