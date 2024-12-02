fn main() {
    let tup = ("Hello", 2, 3.5);
    let (x, y, z) = tup;

    let (a, b, c) = (tup.0, tup.1, tup.2);

    println!("x is {x} or {a}");
    println!("y is {y} or {b}");
    println!("z is {z} or {c}");
}
