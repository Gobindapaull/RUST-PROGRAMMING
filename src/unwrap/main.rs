fn main() {
    let x: Option<i32> = Some(3);
    let x_v = x.unwrap();
    println!("{x_v}");
}
