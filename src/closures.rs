fn main() {
    println!("Closures in rust");
    test_closure();
}

fn test_closure() {
    let text = |x: i8, y: i8| {
        println!("x and y are {} {}", x, y);
        let z = x + y;
        println!("result is {}", z);
        println!("✅");
    };
    text(-3, 5);
}
