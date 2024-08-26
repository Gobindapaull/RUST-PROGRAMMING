fn main() {
    let n: i32 = 5;
    let res: (i32, i32) = square(n);
    println!("{} {}", res.1, res.0);
    println!("{:?}", res);
}

fn square(x: i32) -> (i32, i32) {
    return (x, x*x);
}
