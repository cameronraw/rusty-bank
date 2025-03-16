pub mod bank_account;
mod transaction;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::VecDeque};

    use bank_account::{BankAccount, HandlesTransactions};
    use chrono::{DateTime, NaiveDate, TimeZone, Utc};

    use super::*;

    fn mock_datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Utc> {
        let naive_date = NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap();
        Utc.from_utc_datetime(&naive_date)
    }

    #[test]
    fn bank_account_should_print_statement() {
        let received_statement = RefCell::from(String::new());
        let printer = |statement: String| {
            received_statement.borrow_mut().push_str(&statement);
        };
        let mocked_dates = RefCell::from(VecDeque::from([
            mock_datetime(2025, 1, 1, 12, 0, 0),
            mock_datetime(2025, 4, 2, 10, 30, 22),
            mock_datetime(2025, 3, 3, 15, 45, 52),
        ]));
        let date_getter = || {
            mocked_dates
                .borrow_mut()
                .pop_front()
                .expect("No mocked date found")
        };
        let mut bank_account = BankAccount::new(date_getter, printer);
        bank_account.deposit(100);
        bank_account.deposit(25);
        bank_account.deposit(25);
        bank_account.print_statement();
        const EXPECTED_OUTPUT: &str =
            "01-01-2025 12:00:00 | 100 \n 02-04-2025 10:30:22 | 25 \n 03-03-2025 15:45:52 | 25 \n ";
        assert_eq!(*received_statement.borrow(), EXPECTED_OUTPUT);
    }
}
