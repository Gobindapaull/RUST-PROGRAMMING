fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{guess}");

    // Floating point types
    // IEEE-754 standard
    let _x: f32 = 2.0;
    let _y: f64 = 3.0;

    // Numeric Operations

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    println!("{quotient}"); // 1.7608695652173911

    let truncated = -5 / 3;
    println!("{truncated}"); // -1

    // remainder
    let remainder = 43 % 5;
    println!("{remainder}"); // 3

    // booleans
    let _t = true;
    let _f: bool = false;

    // The Character Type
    // Rust’s char type is four bytes in size
    let _dollar: char = '💰';
    println!("{_dollar}");

    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length

    let _tuple: (i32, f64, u8) = (-123, 4.99, 11);
    // destructuring 
    let (_a, _b, _c) = _tuple;
    println!("{_c} {_a} {_b}");
    println!("{}", _tuple.2); // 11

    // The Array Type
    let _arr = [1, 2, 3, 4, 5];
    let _element1 = _arr[0];
    let _element2 = _arr[1];
    println!("{_element1} {_element2}");

    let mut _index = String::new();
    println!("{_index}");

}
