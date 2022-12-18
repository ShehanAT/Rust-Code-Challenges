fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
    // todo!();
    let mut result = 0;
    let mut all_missing_values = true;
    for (pos, e) in numbers.iter().enumerate() {
        let unwraped_num = e.unwrap_or(0); 
        if unwraped_num != 0 {
            all_missing_values = false;
            break;
        }
    }

    if !all_missing_values {
        for (pos, e) in numbers.iter().enumerate() {
            let unwrapped_num = e.unwrap_or(0); 
            result += unwrapped_num;
        }
    } 
    result
}

fn main() {
    let nn = vec![Some(1), Some(5), Some(4)];
    println!("{:}", sum_with_missing(nn));
    // println!("");
}

#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(nn), 0);
}