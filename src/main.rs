use std::io;

fn main() {
    println!("Enter Fahrenheit temperature");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Please enter Fahrenheit temperature");
    let fahrenheit: f32 = input.trim().parse().expect("Provided temperature must be a number");

    let celsius: f32 = (fahrenheit - 32.0) * 5.0/9.0;

    println!("Celsius: {:.1}", celsius);
}