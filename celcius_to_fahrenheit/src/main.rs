use std::io;
fn main() {
    let mut input = String::new();
    println!("Please enter the temperature in Celsius: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let temp: i32=input.trim().parse().expect("Please type a number!");
    println!("The temperature in Celsius is: {}", temp);
    println!("The temperature in Fahrenheit is: {}", temp * 9 / 5 + 32);
}
