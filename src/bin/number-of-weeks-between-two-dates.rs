use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

fn weeks_between(a: &str, b: &str) -> i32 {
    let first_date = NaiveDate::parse_from_str(a, "%Y-%m-%d").unwrap();
    let second_date = NaiveDate::parse_from_str(b, "%Y-%m-%d").unwrap();

    let offset = second_date - first_date;
    offset.num_weeks() as i32
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("Number of weeks between dates: {:}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}