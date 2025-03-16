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
