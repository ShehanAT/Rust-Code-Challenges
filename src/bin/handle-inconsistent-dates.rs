use std::string::ParseError;

use chrono::NaiveDate;

// Parses a string that represents a date. When a date
// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let parse_from_str = NaiveDate::parse_from_str;

    let format_1 = "%Y-%m-%d";
    let format_2 = "%Y/%m/%d";
    let format_3 = "%d.%m.%Y";
    let format_4 = "%m.%d.%Y";

    let format_1_fail = false;
    let format_2_fail = false;
    let format_3_fail = false;
    let format_4_fail = false;
    let mut parsing_status = String::new();

    for i in 1..5 {
        let conversion_result = parse_from_str(text, &format!("format_{i}"));
        if(NaiveDate::default() != conversion_result.unwrap_or(NaiveDate::default())) {
            // parsing process suceeded 
            return conversion_result.unwrap().pred_opt()
        }
    }
    return None;
}

fn main() {
    let dates = [
        "2010-12-11", 
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    // for d in dates.iter() {
    //     println!("{} -> {:?}", d, flexible_date_parse(d));
    // }
    println!("{} -> {:?}", dates[0], flexible_date_parse(dates[0]));
}

#[test]
fn ymd_hyphen() {
    assert_eq!(flexible_date_parse("2010-12-11"), Some(NaiveDate::from_ymd(2010, 12, 11)))
}

#[test]
fn ymd_slash() {
    assert_eq!(flexible_date_parse("1999/Mar/02"), Some(NaiveDate::from_ymd(1999, 3, 2)))
}

#[test]
fn dmy_dot() {
    assert_eq!(flexible_date_parse("01.Mar.2021"), Some(NaiveDate::from_ymd(2021, 3, 1)))
}

#[test]
fn mdy_dot() {
    assert_eq!(flexible_date_parse("Apr.05.2021"), Some(NaiveDate::from_ymd(2021, 4, 5)))
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}