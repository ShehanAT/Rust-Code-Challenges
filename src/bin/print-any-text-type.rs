fn info<T: AsRef<str>>(text: &T) {
    // The trait std::convert::AsRef is used to do a reference-to-reference conversion
    println!("{}", text.as_ref());
}

fn info_v2<T: std::fmt::Display>(text: &T) {
    // The trait std::convert::AsRef is used to do a reference-to-reference conversion
    println!("{}", text);
}

fn main() {
    let a = "?";
    let b = "?".to_string();

    info(&a);
    info(&b);
    info_v2(&a);
}

#[test]
fn str() {
    let input = "Rust";
    info_v2(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info_v2(&input);
}

#[test]
fn path() {
    use std::path::Path;
    let input = Path::new("/tpm/rust").file_name()?.to_str()?;
    info(&input);
}