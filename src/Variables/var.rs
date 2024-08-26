fn main() {
    // immutable variable
    let x = 5;
    println!("The value of x is : {x}");

    // mutable variable
    let mut y = 2;
    println!("The value y is : {y}");
    y = 3;
    println!("The new y value is : {y}");

    // constant
    const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let p = 4;
    let p = p + 1;
    println!("The value of p is : {p}");

    let spaces = "      ";
    let spaces = spaces.len();
    println!("{spaces}");

}
