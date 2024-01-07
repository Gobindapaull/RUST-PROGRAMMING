pub mod helper;

fn main() {
    println!("Hello, world!");
    test_fun();
    let store_name = helper::get_full_name("crypto", "millionaire");
    println!("main: {}", store_name);

    let add_sum = helper::helper_child::sum(99);
    println!("sum is {}", add_sum);
}

#[allow(dead_code)]
fn test_fun() {
    println!("waiting ... ");
}

