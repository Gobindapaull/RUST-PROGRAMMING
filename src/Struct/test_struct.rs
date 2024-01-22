
pub struct Crypto {
    name: String,
    price: u16
}

pub fn crypto_info() -> Crypto {
    let p1 = Crypto{
        name: "BNB".to_string(),
        price: 400
    };
    return p1;
}
