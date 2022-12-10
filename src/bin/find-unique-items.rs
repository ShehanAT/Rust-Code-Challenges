

fn unique(list: Vec<i32>) -> Vec<i32> {
    let mut output_list = list;
    output_list.sort();
    output_list.dedup();
    output_list
}



fn main() {
    let answer = unique(vec![1, 2, 5, 7, 9, 11]);

    println!("median([1,2,5,7,9,11]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output: Vec<i32> = vec![];
    let actual_output = unique(input);
    assert_eq!(expected_output, actual_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 5, 2, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    // the mut keyword can be used to create mutable variables, meaning the value assigned to them can be changed more than once
    let mut input = vec![1, 5, 5, 2, 2];
    input.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}