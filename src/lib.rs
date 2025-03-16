pub mod bank_account;
mod transaction;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::VecDeque};

    use bank_account::{BankAccount, HandlesTransactions};
    use chrono::{DateTime, NaiveDate, TimeZone, Utc};

    use super::*;

    // Helper function improves test readability
    fn mock_datetime(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> DateTime<Utc> {
        let naive_date = NaiveDate::from_ymd_opt(year, month, day)
            .expect("Error creating mock date")
            .and_hms_opt(hour, minute, second)
            .expect("Error creating mock time");
        Utc.from_utc_datetime(&naive_date)
    }

    #[test]
    fn bank_account_should_print_statement() {
        let received_statement = RefCell::from(String::new());
        let printer = |statement: String| {
            received_statement.borrow_mut().push_str(&statement);
        };
        let mocked_dates = RefCell::from(VecDeque::from([
            mock_datetime(2025, 1, 2, 12, 0, 0),
            mock_datetime(2025, 1, 20, 10, 30, 22),
            mock_datetime(2025, 2, 5, 15, 45, 52),
            mock_datetime(2025, 2, 8, 9, 12, 2),
            mock_datetime(2025, 3, 2, 22, 41, 19),
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
        bank_account.withdraw(12);
        bank_account.deposit(25);
        bank_account.deposit(20);
        bank_account.print_statement();

        const EXPECTED_OUTPUT: &str = r#"02-01-2025 12:00:00 | 100
20-01-2025 10:30:22 | 25
05-02-2025 15:45:52 | -12
08-02-2025 09:12:02 | 25
02-03-2025 22:41:19 | 20

Balance: 158"#;
        assert_eq!(*received_statement.borrow(), EXPECTED_OUTPUT);
    }
}
