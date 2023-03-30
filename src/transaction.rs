pub trait Transaction {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}
