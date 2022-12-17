// Formula for F to C: C = (F - 32) * 5/9
// Formula for C to F: F = C x 9/5 + 32
#![allow(unused)]
use std::any::type_name;

enum Scale {
    Celsius,
    Fahrenheit,
}

struct Temperature {
    degrees: f32, 
    scale: Scale,
}

impl Temperature {
    fn new(degrees: f32) -> Self {
        Temperature { 
            degrees, 
            scale: Scale::Celsius,
        }
    }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    fn to_celsius(&self) -> f32 {
        match self.scale {
            Scale::Celsius => self.degrees,
            Scale::Fahrenheit => {
                let degrees_in_f = self.degrees;
                let degrees_in_c: f32 = (degrees_in_f - 32.0) * 5.0/9.0;
                degrees_in_c
            }
        }   
    }

    fn to_fahrenheit(&self) -> f32 {
        match self.scale {
            Scale::Celsius => {
                let degrees_in_c = self.degrees;
                let degrees_in_f = (degrees_in_c * 9.0/5.0) + 32.0;
                // self.scale = Scale::Fahrenheit;
                degrees_in_f
                
            },
            Scale::Fahrenheit => self.degrees,
        }
    }
}

fn main() {
    let temp = Temperature::new(20.0);

    println!("Fun Factor: 20C is an integer in celsius and fahrenheit");
    println!("          {:.1}C = {:.1}F", temp.to_celsius(), temp.to_fahrenheit());
}

#[test]
fn one_degree() {
    let cold = Temperature::new(1.0);
    assert!((cold.to_fahrenheit() - 33.8) < 0.01);
    println!("{:}", (cold.to_fahrenheit()));
    assert!((cold.to_fahrenheit() - 32.8) == 1.0);
}