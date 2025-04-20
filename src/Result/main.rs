fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10.0, 2.3) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e)
    }
}
