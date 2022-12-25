
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

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        todo!();
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        todo!();
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JRKUC";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);
}