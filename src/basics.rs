fn main() {
    println!("Hello, world!");
    test_fun();
}

fn test_fun() {
    let x: u8 = 5;
    let y: f32 = 2.0; 
    let string_type: &str = "money"; // double quote
    let char_type: char = 'a'; // single quote
    let tuple_type = ("rust", 1000000);
    let array_type = [5, 9, 11, 27, 54, 99];
    let new_array = &array_type[1..4];

    println!("{}", x);
    println!("{}", y);
    println!("{}", string_type);
    println!("{}", char_type);
    println!("{:?}", tuple_type);
    println!("{:?}", array_type);
    println!("{:?}", new_array);
}
