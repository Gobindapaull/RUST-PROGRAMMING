fn example() -> Result<i32, &'static str> {
    Ok(42)
    // Err("err")
}

fn main() {
    let result = example();
    let value: Option<i32> = result.ok();
    match value {
        Some(val) => println!("Got value: {}", val),
        None => println!("No value")
    }
}
