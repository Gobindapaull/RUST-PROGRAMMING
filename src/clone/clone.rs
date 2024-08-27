fn main() {
    let mut s = String::from("Rust");
    let s_clone = s.clone();
    s.push_str(" programming");
    println!("{}", s);
    println!("{s_clone}");
}
