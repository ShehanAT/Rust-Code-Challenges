use sycamore::prelude::*;

pub(crate) const SIZE: usize = 192;

#[derive(Clone, Copy)]
pub(crate) struct DictWrap(pub(crate) [char; SIZE]);

#[derive(Clone, Copy)]
pub(crate) struct VigMatrixWrap(pub(crate) [[char; SIZE]; SIZE]);


// Creates and returns a new Vigenere Matrix
impl VigMatrixWrap {
    pub(crate) fn new() -> VigMatrixWrap {
        let mut mat: VigMatrixWrap = VigMatrixWrap([[' '; SIZE]; SIZE]);
        let binding = DictWrap::new().0;
        let mut acc = binding.iter().cycle();

        for r in 0..mat.0.len() {
            for c in 0..mat.0.len() {
                mat.0[r][c] = *acc.next().unwrap();
            }
            acc.next();
        }
        return mat;
    }
}


// Creates and returns a new dictionary for the Vigenere Matrix
impl DictWrap {
    pub(crate) fn new() -> DictWrap {
        // Every ASCII character that !is_control().
        let mut dict = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ"##.to_string();
        // Add carriage return to support in web textarea
        dict.push('\n');
        dict.push('\r');
        let mut dict_char_arr = [' '; SIZE];
        for (idx, ch) in dict.chars().enumerate() {
            dict_char_arr[idx] = ch;
        }

        return DictWrap(dict_char_arr);
    }

    pub(crate) fn get_string(&self) -> String {
        let mut s = String::new();
        for ch in self.0 {
            s.push(ch);
        }
        s
    }
}

// mod vigenere {
//     use ciphers::{Vigenere, Cipher};

//     pub fn encrypt(plaintext: &str, key: &str) -> String {
//        let vigenere = Vigenere::new(key);
 
//        return vigenere.encipher(plaintext).unwrap();
//     }

//     pub fn decrypt(ciphertext: &str, key: &str) -> String {
//         let vigenere = Vigenere::new(key);
//         println!("ciphertext: {}", ciphertext);
//         return vigenere.decipher(ciphertext).unwrap();
//     }
// }

// V2
mod vigenere {
    use crate::VigMatrixWrap;

    #[derive(Debug)]
    pub(crate) enum ErrorCode {
        InvalidChar(char),
        InvalidIndex(usize),
    }


    // Completes the key if the size is not the same as the message 
    fn complete_key(key: &str, msg_size: usize) -> String {
        // The .cycle() method Repeats an iterator endlessly. Instead of stopping at None, the iterator will instead start again, from the beginning. After iterating again, it will start at the beginning again
        let mut key_chars = key.chars().cycle();
        let mut new_key = "".to_string();
        for _ in 0..msg_size {
            new_key.push(key_chars.next().unwrap());
        }
        new_key
    }

    // Returns the matching character in the Vigenere matrix, depending on the header (ch_m) and column (ch_k) characters provided
    fn vig_matcher(m: &VigMatrixWrap, ch_m: char, ch_k: char) -> Result<char, ErrorCode> {
        let idx_c = idx_finder(ch_m, &m)?;
        let idx_r = idx_finder(ch_k, &m)?;

        Ok(m.0[idx_r][idx_c])
    }

    fn idx_finder(ch: char, m: &VigMatrixWrap) -> Result<usize, ErrorCode> {
        for (idx, chi) in m.0[0].iter().enumerate() {
            if ch == *chi {
                return Ok(idx)
            }
        }

        Err(ErrorCode::InvalidChar(ch))
    }

    pub fn encrypt(plaintext: &str, key: &str, vig_mat: VigMatrixWrap) -> String {
        let msg_size = plaintext.chars().count();
        let key_size = key.chars().count();

        // Initializations 
        let mut encrypted_msg = "".to_string();

        let mut key_e = key.to_string();
        if msg_size != key_size {
            key_e = complete_key(key, msg_size);
        }

        // Convert to char vectors 
        let key_chars: Vec<_> = key_e.to_string().chars().collect();
        let msg_chars: Vec<_> = plaintext.to_string().chars().collect();

        // Encrypt message 
        for i in 0..msg_size {
            encrypted_msg.push(vig_matcher(&vig_mat, msg_chars[i], key_chars[i])?);
        }
        

        encrypted_msg
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        todo!();
    }
}

fn main() {
    let mat_signal = create_signal(cx, VigMatrixWrap::new());
    let key = "WHYRUST";
    let encodemsg = "ENCODE_THIS";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JRKUC";
    let encoded_msg = vigenere::encrypt(encodemsg, key, mat_signal.get().as_ref().clone())
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("Encoded msg: {}", encoded_msg);
    println!("{}", plaintext);
}