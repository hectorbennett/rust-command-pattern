use crate::transaction::Transaction;

pub struct BankController {}

impl BankController {
    pub fn new() -> BankController {
        BankController {}
    }

    pub fn execute(&mut self, transaction: &mut dyn Transaction) -> &str {
        transaction.execute()
    }
}
