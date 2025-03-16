use chrono::{DateTime, Utc};

use crate::transaction::Transaction;

type TransDate = DateTime<Utc>;

pub struct BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> TransDate,
    TPrinter: Fn(String),
{
    transactions: Vec<Transaction>,
    date_getter: TDate,
    printer: TPrinter,
}

impl<TDate, TPrinter> BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> TransDate,
    TPrinter: Fn(String),
{
    fn format_date_time(date_time: &DateTime<Utc>) -> String {
        date_time.format("%d-%m-%Y %H:%M:%S").to_string()
    }

    pub fn print_statement(&self) {
        let statement = self.transactions.iter().fold(String::new(), |mut acc, t| {
            match t {
                Transaction::Deposit(deposit, date_time) => acc.push_str(&format!(
                    "{} | {} \n ",
                    Self::format_date_time(date_time),
                    deposit
                )),
                Transaction::Withdrawal(withdrawal, date_time) => acc.push_str(&format!(
                    "{} | -{} \n ",
                    Self::format_date_time(date_time),
                    withdrawal
                )),
            }
            acc
        });
        (self.printer)(statement);
    }
}

impl<TDate, TPrinter> BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> TransDate,
    TPrinter: Fn(String),
{
}

impl<TDate, TPrinter> BankAccount<TDate, TPrinter>
where
    TDate: Fn() -> TransDate,
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
    TDate: Fn() -> TransDate,
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
