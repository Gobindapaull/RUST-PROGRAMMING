struct Token {
    name: String,
    price: u64
}

fn main() {
    let token: Token = Token {
        name: String::from("ETH"),
        price: 3000
    };

    println!("name: {} , price: {}", token.name, token.price);
}
