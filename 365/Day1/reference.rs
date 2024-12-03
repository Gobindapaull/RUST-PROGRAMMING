fn main() {
    let s1 = String::from("Hello");
    let s2 = &s1[1..3];
    println!("{:?}", s2); // "el"

    let a1 = [1, 2, 3, 4, 5];
    let a2 = &a1[1..4];
    println!("{:?}", a2); // [2, 3, 4]

}
