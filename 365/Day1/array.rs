fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, -5];
    let b: [&str; 2] = ["Hello", "World"];
    let c: [i32; 5] = [3; 5];

    println!("{}", a[3]);
    println!("{}", a.len());

    println!("{}", b[0]);
    println!("{}", b.len());

    println!("{}", c[2]);
    println!("{}", c.len());
}
