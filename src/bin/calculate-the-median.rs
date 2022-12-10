fn median(mut b: Vec<f32>) -> Option<f32> {
        if b.is_empty(){
            return None;
        }


        b.sort_by(|x, y| x.partial_cmp(y).unwrap());

        let n_elements = b.len();
        let mid = n_elements / 2;


        let med = if n_elements % 2 == 0 {
            // even number 
            (b[mid] + b[mid - 1]) / 2.0
        } else {
            // odd number
            b[mid]
        };

        Some(med)
}


fn main() {
    let answer = median(vec![1.0, 2.0, 5.0, 7.0, 9.0, 11.0]);

    println!("median([1,2,5,7,9,11]) = {:?}", answer);
}


#[test] 
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 4.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

