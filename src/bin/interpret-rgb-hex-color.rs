use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    // TODO: implement trait 
    fn r(&self) -> u8 { self.r }
    
    fn g(&self) -> u8 { self.g }
    
    fn b(&self) -> u8 { self.b }
}

#[derive(Debug, PartialEq)]
enum ColorError {
    HashMissing,
    RedColorOutOfBounds(ParseIntError),
    GreenColorOutOfBounds(ParseIntError),
    BlueColorOutOfBounds(ParseIntError),
}

impl FromStr for Rgb {
    // TODO: implement trait 
    type Err = ColorError;

    fn from_str(string: &str) -> Result<Self, <Self as FromStr>::Err> { 
        
        if string.find("#") == None { return Err(ColorError::HashMissing) }

        let trimmed_string = string.replace("#", "");
        println!("{:}", trimmed_string);
        let r_code = &trimmed_string[0..2];
        let g_code = &trimmed_string[2..4];
        let b_code = &trimmed_string[4..6];

        let r_num = u8::from_str_radix(r_code, 16).or_else(|err| Err(ColorError::RedColorOutOfBounds(err)));
        let g_num = u8::from_str_radix(g_code, 16).or_else(|err| Err(ColorError::GreenColorOutOfBounds(err)));
        let b_num = u8::from_str_radix(b_code, 16).or_else(|err| Err(ColorError::BlueColorOutOfBounds(err)));

        Ok(Rgb { r: r_num.unwrap(), g: g_num.unwrap(), b: b_num.unwrap() })
    }
}


impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {

}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        // println!("{:}", hex);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}


#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}


#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}

