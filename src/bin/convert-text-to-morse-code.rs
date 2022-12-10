/*
A trait defines functionality a particular type has and can share with other types. We can use traits 
to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type 
can be any type that has certain behavior.

Traits are similar to interfaces in Java, although there are some differences
*/

fn to_morse_code() -> Message {
    
}

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

type Message = Vec<Letter>;
type Letter = Vec<Pulse>;

#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        };
        print!(" ");
    };
    println!();
}

fn main() {
    let greeting = "Hello, world"
        .to_string()
        .to_morse_code();

    print_morse_code(&greeting);
}