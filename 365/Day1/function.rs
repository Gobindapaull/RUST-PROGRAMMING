fn main() {
    test( 1000000000.0);
    let gas = gas_limit();
    println!("{}", gas);
}

fn test(price: f64) {
    println!("{}", price / 1e9); // wei to GWEI
}

fn gas_limit() -> u32 {
    return 21000;
}
