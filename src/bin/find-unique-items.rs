
fn unique(list: Vec<i32>) -> Vec<i32> {

}



fn main() {
    let answer = unique(vec![1.0, 2.0, 5.0, 7.0, 9.0, 11.0]);

    println!("median([1,2,5,7,9,11]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output: Vec<i32> = vec![];
    let actual_output = unique(input);
    assert_eq!(expected_output, actual_output);
}