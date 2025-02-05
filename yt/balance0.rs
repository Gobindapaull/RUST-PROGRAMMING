use ethers::types::U256;

fn main() {
    let balance = U256::from(1234);

    // balance > 0
    if balance > U256::from(0) {
        println!("Balance is greater than zero");
    }    
}

// [dependencies]
// ethers = "2.0"
