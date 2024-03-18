use std::io;
use std::num::IntErrorKind;

fn factorial(input: u128) -> u128 {
    match input <= 1 {
        true => 1,
        false => input * factorial(input - 1),
    }
}

fn main() {
    println!("Enter number to get factorial:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Something went wrong");

    let number: u8 = match input.trim().parse() {
        Ok(value) => value,
        Err(error) => match error.kind() {
            IntErrorKind::InvalidDigit => panic!("Inavalid digit"),
            IntErrorKind::PosOverflow => panic!("Bigger than u8 data type"),
            IntErrorKind::Empty => panic!("Value is emty"),
            other_error => panic!("Problem parsing the value {:?}", other_error),
        },
    };

    if number > 34 {
        panic!("Factorial cannot be found for value: {input}");
    }

    println!("Factorial for {input} is {}", factorial(number as u128));
}
