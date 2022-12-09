const list = vec![1.0, 4.0, 5.0];
assert_eq!(median(list), None);

fn median(list: Vec<f32>) -> Option<f32> {
    if list.is_empty(){
        return None;
    }

    // if alloc::slice 
    //     pub fn sort(&mut self)


    list.sort();
    let mid = list.len() / 2;
    let med = if list.len() % 2 == 0 {
        // even number 
        (list[mid - 1] + list[mid]) / 2.0;
    }else{
        // odd number
        list[mid];
    };

    Some(med);
}
