use chrono::{DateTime, Utc};

use crate::transaction::Transaction;

pub struct BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> DateTime<Utc>,
    TPrinter: Fn(String),
{
    transactions: Vec<Transaction>,
    date_getter: TDate,
    printer: TPrinter,
}

impl<TDate, TPrinter> BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> DateTime<Utc>,
    TPrinter: Fn(String),
{
    pub fn print_statement(&self) {
        let (mut statement, balance): (String, isize) = self.transactions.iter().fold(
            (String::new(), 0),
            |(mut statement, mut balance), transaction| {
                statement.push_str(&format!("{}", transaction));
                transaction.apply_amount(&mut balance);
                (statement, balance)
            },
        );
        statement.push_str(&format!("\nBalance: {}", balance));
        (self.printer)(statement);
    }
}

impl<TDate, TPrinter> BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> DateTime<Utc>,
    TPrinter: Fn(String),
{
}

impl<TDate, TPrinter> BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> DateTime<Utc>,
    TPrinter: Fn(String),
{
    pub fn new(date_getter: TDate, printer: TPrinter) -> Self {
        Self {
            transactions: vec![],
            date_getter,
            printer,
        }
    }
}

pub trait HandlesTransactions {
    fn deposit(&mut self, amount: usize);
    fn withdraw(&mut self, amount: usize);
}

impl<TDate, TPrinter> HandlesTransactions for BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> DateTime<Utc>,
    TPrinter: Fn(String),
{
    fn deposit(&mut self, amount: usize) {
        self.transactions
            .push(Transaction::Deposit(amount, (self.date_getter)()));
    }

    fn withdraw(&mut self, amount: usize) {
        self.transactions
            .push(Transaction::Withdrawal(amount, (self.date_getter)()));
    }
}

#[cfg(test)]
pub mod bank_account_should {
    use crate::transaction::Transaction::{self, Deposit, Withdrawal};
    use chrono::{NaiveDate, TimeZone, Utc};

    use super::{BankAccount, HandlesTransactions};

    impl PartialEq for Transaction {
        fn eq(&self, other: &Self) -> bool {
            match (self, other) {
                (Self::Deposit(l0, l1), Self::Deposit(r0, r1)) => l0 == r0 && l1 == r1,
                (Self::Withdrawal(l0, l1), Self::Withdrawal(r0, r1)) => l0 == r0 && l1 == r1,
                _ => false,
            }
        }
    }

    const NAIVE_DATE: chrono::NaiveDateTime = NaiveDate::from_ymd_opt(2025, 1, 1)
        .unwrap()
        .and_hms_opt(12, 0, 0)
        .unwrap();

    #[test]
    pub fn add_deposits_to_transactions() {
        let date_getter = || Utc.from_utc_datetime(&NAIVE_DATE);
        let printer = |_statement: String| {};

        let mut bank_account = BankAccount::new(date_getter, printer);
        bank_account.deposit(100);
        assert!(bank_account
            .transactions
            .contains(&Deposit(100, Utc.from_utc_datetime(&NAIVE_DATE))));
    }

    #[test]
    pub fn add_withdrawals_to_transactions() {
        let date_getter = || Utc.from_utc_datetime(&NAIVE_DATE);
        let printer = |_statement: String| {};

        let mut bank_account = BankAccount::new(date_getter, printer);
        bank_account.withdraw(100);
        assert!(bank_account
            .transactions
            .contains(&Withdrawal(100, Utc.from_utc_datetime(&NAIVE_DATE))));
    }
}
