struct Token <A, B, C> {
    name: A,
    price: B,
    buy: C
}

fn main() {
    let token = Token {
        name: String::from("ETH"),
        price: 3000,
        buy: true
    };

    println!("name: {} , price: {} buy: {}", token.name, token.price, token.buy);
}
