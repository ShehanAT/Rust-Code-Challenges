
fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by(|a, b| a.as_ref().to_lowercase().cmp(&b.as_ref().to_lowercase()));
}

fn sort_usernames_v2<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}


fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted: {:?}", &users);

    let mut users2 = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    println!("unsorted sort_by_cached_key: {:?}", users2);
    sort_usernames_v2(&mut users2);
    println!("sorted sort_by_cached_key: {:?}", users2);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}

