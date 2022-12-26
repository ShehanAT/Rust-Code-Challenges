use sycamore::prelude::*;



mod vigenere {
    // *b: A string in Rust is a valid sequence of unicode characters and hence it can be represented as &[u8]. A byte is also an 8-bit integer so it is considered as a sequence of unicode bytes. 
    // Prefixing a string with 'b' converts the string to a sequence of bytes(of type u8). This is called a byte literal
    const SIZE: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const A: u8 = b'A';
    const Z: u8 = b'Z';
    const WRAP: u8 = 26; 

    use ciphers::{Vigenere, Cipher};

    fn clean_input(plaintext: &str) -> impl Iterator<Item = u8> + '_{
        plaintext.bytes().filter_map(|x| 
            // x is a byte representation of a char, not char itself 
            match x {
                A..=Z => Some(x),
                b'a'..=b'z' => Some(x - (b'a' - b'A')), // This converts lowercase char byte literals to capitalized byte literal strings
                _ => None
            }
        )
        // cleaned_iter
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let mut key_bytes = key.bytes().map(|x| x - Z).cycle(); // The purpose for x - A seems to be for encryption purposes

        let encrypted_str = clean_input(plaintext)
            .map(|x| {
                let offset = key_bytes.next().unwrap();
                ((x - A) + offset) % WRAP + A
            })
            .collect();
        
            // If you want to return an expression without the 'return' keyword then exclude the semi colon at its end
            String::from_utf8(encrypted_str).unwrap()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let mut key_bytes = key.bytes().map(|x| x - A).cycle(); // The purpose for x - A seems to be for decryption purposes

        let decrypted_str = clean_input(ciphertext)
        .map(|x|{
            let offset = key_bytes.next().unwrap();
            ((x + WRAP - A) + offset) % WRAP + A
        })
        .collect();

        String::from_utf8(decrypted_str).unwrap()
    }
}

fn main() {

    let key = "WHYRUST";
    let encodemsg = "ENCODE_THIS";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JRKUC
    ";

    println!("{}", vigenere::decrypt(&ciphertext, key));
}