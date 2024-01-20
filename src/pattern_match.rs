
pub fn test_match() {
    let price: u16 = 1000;

    match price {
        1000 => {
            println!("buy crypto");
        }
        _ => {
            println!("sell crypto");
        }
    }
}
