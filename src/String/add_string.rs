fn main() {
    let mut tok: String = String::from("ETH");
    call_coin(&mut tok);
    println!("{}", tok);
}

fn call_coin(tk: &mut String) {
    tk.push_str(" Coin");
}
