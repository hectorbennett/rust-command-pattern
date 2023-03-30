use std::collections::HashMap;
use uuid::Uuid;

use crate::account::Account;

pub fn generate_uid() -> u64 {
    let id = Uuid::new_v4();
    id.to_u128_le() as u64
}

pub struct Bank {
    accounts: HashMap<u64, Account>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank {
            accounts: HashMap::new(),
        }
    }

    pub fn create_account(&mut self, name: &str) -> &mut Account {
        let uid = generate_uid();
        let account = Account::new(name, uid);
        self.accounts.insert(uid, account);
        self.get_account(uid)
    }

    pub fn get_account(&mut self, uid: u64) -> &mut Account {
        self.accounts.get_mut(&uid).unwrap()
    }
}
