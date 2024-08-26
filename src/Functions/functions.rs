fn main() {
    println!("main function");

    another_fn();

    another_fn2(5, '💰');

    let t = ten();
    println!("{t}");
}

fn another_fn() {
    println!("another function");
}

fn another_fn2(x: u8, y: char) {
    println!("x:{} y: {}", x, y);
}

fn ten() -> u8 {
    10
}
