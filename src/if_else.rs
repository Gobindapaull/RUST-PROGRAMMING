
fn main() {
    test_if();
    // test_loop();
}

fn test_if() {
    let price_buy_sell = 1500u16;
    println!("Enter the ETH price: ");
    let user_input_price: &mut String = &mut String::from("");
    std::io::stdin().read_line(user_input_price).unwrap();
    let price:u16 = user_input_price.replace("\n", "").parse::<u16>().unwrap();
    if price > price_buy_sell {
        println!("SELL ETH");
    }
    else if price == price_buy_sell {
        println!("HOLD AND WAIT ..");
    }
    else {
        println!("BUY ETH");
    }

}

// #[warn(dead_code)]
// fn test_loop() {
//     loop {
//         println!("search for new liquidity pair ...")
//     }
// }
