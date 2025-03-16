use std::{
    fmt::Display,
    ops::{AddAssign, SubAssign},
};

use chrono::{DateTime, Utc};

pub enum Transaction {
    Deposit(usize, DateTime<Utc>),
    Withdrawal(usize, DateTime<Utc>),
}

impl Transaction {
    pub fn apply_amount(&self, balance: &mut isize) {
        match self {
            Transaction::Deposit(amount, _) => balance.add_assign(*amount as isize),
            Transaction::Withdrawal(amount, _) => balance.sub_assign(*amount as isize),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (date_time, signed_amount) = match self {
            Transaction::Deposit(amount, date_time) => (date_time, format!("+{}", amount)),
            Transaction::Withdrawal(amount, date_time) => (date_time, format!("-{}", amount)),
        };
        writeln!(
            f,
            "{} | {}",
            date_time.format("%d-%m-%Y %H:%M:%S"),
            signed_amount
        )
    }
}

#[cfg(test)]
pub mod transaction_should {
    use super::Transaction::{Deposit, Withdrawal};
    use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};

    const MOCK_DATE: NaiveDateTime = NaiveDate::from_ymd_opt(2025, 1, 1)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();

    #[test]
    pub fn apply_deposits_to_balance() {
        let mut balance = 0;
        let transaction = Deposit(100, Utc.from_utc_datetime(&MOCK_DATE));
        transaction.apply_amount(&mut balance);
        assert_eq!(balance, 100);
    }

    #[test]
    pub fn apply_withdrawals_to_balance() {
        let mut balance = 200;
        let transaction = Withdrawal(100, Utc.from_utc_datetime(&MOCK_DATE));
        transaction.apply_amount(&mut balance);
        assert_eq!(balance, 100);
    }
    #[test]
    pub fn be_automatically_formatted_deposit_when_displayed_as_string() {
        let deposit = Deposit(100, Utc.from_utc_datetime(&MOCK_DATE));
        assert_eq!(
            String::from("01-01-2025 12:00:00 | +100\n"),
            format!("{}", deposit)
        );
        let withdrawal = Withdrawal(100, Utc.from_utc_datetime(&MOCK_DATE));
        assert_eq!(
            String::from("01-01-2025 12:00:00 | -100\n"),
            format!("{}", withdrawal)
        )
    }
    #[test]
    pub fn be_automatically_formatted_withdrawal_when_displayed_as_string() {
        let withdrawal = Withdrawal(100, Utc.from_utc_datetime(&MOCK_DATE));
        assert_eq!(
            String::from("01-01-2025 12:00:00 | -100\n"),
            format!("{}", withdrawal)
        )
    }
}
