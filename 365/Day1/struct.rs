fn main() {
    let coin = Coin {
        name: String::from("BNB"),
        price: 600,
        buy: true
    };

    println!("name: {}, price: {}, buy: {}", coin.name, coin.price, coin.buy);
}

struct Coin {
    name: String,
    price: u64,
    buy: bool
}
