use std::collections::HashMap;

trait Command {
    fn execute(&mut self);
}

struct Deposit {
    account: Account,
    amount: u32,
}
impl Deposit {
    fn new(account: Account, amount: u32) -> Deposit {
        Deposit { account, amount }
    }
}
impl Command for Deposit {
    fn execute(&mut self) {
        self.account.deposit(self.amount);
    }
}

struct Withdraw {
    account: Account,
    amount: u32,
}
impl Withdraw {
    fn new(account: Account, amount: u32) -> Withdraw {
        Withdraw { account, amount }
    }
}
impl Command for Withdraw {
    fn execute(&mut self) {
        self.account.withdraw(self.amount);
    }
}

struct BankController {
    commands: Vec<Box<dyn Command>>,
}

impl BankController {
    fn new() -> BankController {
        BankController { commands: vec![] }
    }

    fn add_command(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }

    fn execute(&mut self) {
        for cmd in self.commands.iter_mut() {
            cmd.execute();
        }
    }
}

#[derive(Clone)]
pub struct Account {
    pub name: String,
    pub balance: i32,
}

impl Account {
    pub fn new(name: &str) -> Account {
        Account {
            name: name.into(),
            balance: 0,
        }
    }

    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount as i32;
    }

    pub fn withdraw(&mut self, amount: u32) {
        self.balance -= amount as i32;
    }
}

pub fn test_3() {
    // let bank = Bank::new();
    let mut controller = BankController::new();

    let hector = Account::new("hector");
    let hayley = Account::new("hayley");

    // deposit
    controller.add_command(Box::new(Deposit::new(hector, 10000)));
    // controller.add_command(Box::new(Withdraw::new(hector, 500)));
    // controller.add_command(Box::new(Deposit::new(hayley, 10000)));

    // withdraw
    // controller.add_command(Box::new(Withdraw::new(hector, 10000)));

    controller.execute();
}
