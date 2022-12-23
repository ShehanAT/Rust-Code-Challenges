use std::string::ParseError;
use chrono::format::ParseErrorKind::TooShort;
use chrono::NaiveDate;
use chrono::Duration;

// Parses a string that represents a date. When a date
// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let parse_from_str = NaiveDate::parse_from_str;

    let format_1 = "%Y/%b/%d";
    let format_2 = "%Y-%m-%d";
    
    let format_3 = "%d.%b.%Y";
    let format_4 = "%b.%d.%Y";

    let format_1_fail = false;
    let format_2_fail = false;
    let format_3_fail = false;
    let format_4_fail = false;
    let mut parsing_status = String::new();

    // for i in 1..5 {
    //     let conversion_result = parse_from_str(text, &format!("format_{i}"));
    //     println!("{:}", conversion_result.unwrap());
    //     match conversion_result.unwrap() {
    //         ParseError(Invalid) => return Err(ParseError(Invalid))
    //     }
    //     if(NaiveDate::default() != conversion_result.unwrap_or(NaiveDate::default())) {
    //         // parsing process suceeded 
    //         return conversion_result.unwrap().pred_opt()
    //     }
    // }
    let default = NaiveDate::default();
    let conversion_result_1 = parse_from_str(text, format_1).unwrap_or(default);
    let conversion_result_2 = parse_from_str(text, format_2).unwrap_or(default);
    let conversion_result_3 = parse_from_str(text, format_3).unwrap_or(default);
    let conversion_result_4 = parse_from_str(text, format_4).unwrap_or(default);

    let conversion_results_arr = [conversion_result_1, conversion_result_2, conversion_result_3, conversion_result_4];

    let failed_date = NaiveDate::from_ymd_opt(1970, 01, 01);

    // println!("{:}", conversion_result_1);
    // println!("{:}", conversion_result_2);
    // println!("{:}", conversion_result_3);
    // println!("{:}", conversion_result_4);
    // println!("{:}", conversion_result_5);

    for i in conversion_results_arr.iter() {
        println!("result: {:} duration: {:}", i.to_string(), *i - failed_date.unwrap());
        if *i - failed_date.unwrap() != Duration::zero(){
            return Some(*i)
        }
    }



    // if(conversion_result_1 != ParseError(Invalid)) {
    //     return converison_result_1;
    // } else if (conversion_result_2 != ParseError(Invalid)) {
    //     return converison_result_2;
    // } else if (conversion_result_3 != ParseError(Invalid)) {
    //     return converison_result_3;
    // } else if (conversion_result_4 != ParseError(Invalid)) {
    //     return converison_result_4;
    // }

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