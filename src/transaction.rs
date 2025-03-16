use chrono::{DateTime, Utc};

pub enum Transaction {
    Deposit(usize, DateTime<Utc>),
    Withdrawal(usize, DateTime<Utc>),
}
