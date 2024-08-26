use std::io;

fn main() {
    println!("Enter your weight (in kg) : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f64 = input.trim().parse().unwrap();
    println!("New weight : {}", weight + 15.00);
}
