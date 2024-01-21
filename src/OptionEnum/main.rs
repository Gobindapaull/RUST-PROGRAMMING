pub mod optiontest;

fn main() {
    println!("Option Enum test");

    let result = optiontest::option_test();
    println!("{}", result.unwrap());

    let result1 = optiontest::option_test_string();
    println!("{}", result1.unwrap());

    let result2 = optiontest::option_test_char();
    println!("{}", result2.unwrap().to_string());

}
