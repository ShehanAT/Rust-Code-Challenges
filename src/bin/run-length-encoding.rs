mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut counter = 0;
        let mut prev = None;
        let mut encoding_arr = String::with_capacity(text.len());
        let mut char_list = text.chars();

        while let Some(c) = char_list.next() {
            if prev == None {
                prev = Some(c)
            }
                        
            if(!c.eq(&prev.unwrap()) || counter == 9){
                encoding_arr.push_str(&format!("{}{}", counter, prev.unwrap()));
                counter = 0;
            }

            counter = counter + 1;
            prev = Some(c);
        }
        encoding_arr.push_str(&format!("{}{}", counter, prev.unwrap()));
        encoding_arr        
    }

    pub fn decode(text: &str) -> String {
        let mut decoding_arr = vec![];
        let mut char_list = text.chars();
        let n = 2;
        let mut current_block = String::new();
        for (i, c) in char_list.enumerate() {
            if (i + 1) % n != 0 {
                current_block.push(c);
            }else {
                current_block.push(c);
                decoding_arr.push(current_block);
                current_block = "".to_string();
            }
        }
        
        let mut decoded_str = String::with_capacity(text.len());

        for block in decoding_arr.iter() {
            let occurances = block.chars().nth(0).unwrap().to_digit(10).unwrap();
            let letter = block.chars().nth(1).unwrap();
            
            let decoded_block = (0..occurances).map(|_| letter).collect::<String>();
            decoded_str.push_str(&decoded_block);
        }

        decoded_str
    }

}


fn main() {
    use run_length_encoding::*;

    println!("{:}", decode("1a1b1c"));
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}



