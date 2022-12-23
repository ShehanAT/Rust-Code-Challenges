use std::string::ParseError;
use chrono::format::ParseErrorKind::TooShort;
use chrono::NaiveDate;
use chrono::Duration;

fn is_year(field: &str) -> bool {
    field.len() == 4 && field.chars().all(|x| x.is_ascii_digit())
}

fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let text = text.trim();

    // check that there are numbers 
    if !text.bytes().any(|x| x.is_ascii_digit()) {
        return None;
    }

    // Allow any known delimiter 
   let fields: Vec<_> = text.split(['/', '-', ' ', '.'].as_slice()).collect(); // .as_slice() extracts a slice containing the entire vector

    let mut year = None;
    let mut month = None;
    let mut day = None;

    for field in fields.iter() {
        if field.len() < 3 { // skip any fields less than 3, which means day fields will be skipped
            continue;
        }

        let m = match &field.to_lowercase()[..3] { // [..3] removes all characters past the third index position and returns chars [0-3]
            "jan" => 1,
            "feb" => 2, 
            "mar" => 3,
            "apr" => 4,
            "may" => 5, 
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "oct" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => continue,
        };

        month = Some(m)
    }

    for field in fields.iter() {
        if is_year(field) {
            year = field.parse::<i32>().ok(); // attempt to convert string type field into u32, will silently return Err() if not doable
            continue;
        }

        if month.is_some() {
            day = field.parse::<u32>().ok(); // Since month usual preceeds day, attempt to convert field to u2, assuming that its a valid day
        } else {
            month = field.parse::<u32>().ok(); // If only year and month is provided skip the day field 
        }
    }

    match (year, month, day) {
        (Some(year), Some(month), Some(day)) => NaiveDate::from_ymd_opt(year, month, day),
        (Some(year), Some(month), None) => NaiveDate::from_ymd_opt(year, month, 1),
        _ => None
    }
}

// By Sean A 
// fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
//     let parse_from_str = NaiveDate::parse_from_str;

//     let format_1 = "%Y/%b/%d";
//     let format_2 = "%Y-%m-%d";    
//     let format_3 = "%d.%b.%Y";
//     let format_4 = "%b.%d.%Y";
//     let format_5 = "%Y %b %d";
//     let format_6 = "%Y/%B/%d";


//     let default = NaiveDate::default();
//     let conversion_result_1 = parse_from_str(text, format_1).unwrap_or(default);
//     let conversion_result_2 = parse_from_str(text, format_2).unwrap_or(default);
//     let conversion_result_3 = parse_from_str(text, format_3).unwrap_or(default);
//     let conversion_result_4 = parse_from_str(text, format_4).unwrap_or(default);
//     let conversion_result_5 = parse_from_str(text, format_5).unwrap_or(default);
//     let conversion_result_6 = parse_from_str(text, format_6).unwrap_or(default);

//     let conversion_results_arr = [conversion_result_1, conversion_result_2, conversion_result_3, conversion_result_4, conversion_result_5, conversion_result_6];

//     let failed_date = NaiveDate::from_ymd_opt(1970, 01, 01);

//     for i in conversion_results_arr.iter() {
//         // println!("result: {:} duration: {:}", i.to_string(), *i - failed_date.unwrap());
//         if *i - failed_date.unwrap() != Duration::zero(){
//             return Some(*i)
//         }
//     }

//     return None;
// }

fn main() {
    let dates = [
        "2002 Feb 02",
        "2010-12-11",
        "1999/March/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
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