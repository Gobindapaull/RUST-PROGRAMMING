// Generates a 12-character alphanumeric password

use rand::{thread_rng, Rng};

fn main() {
    let password: String = (0..12).map(|_| thread_rng().sample(rand::distributions::Alphanumeric) as char).collect();
    println!("🔐 Password: {}", password); // 🔐 Password: TxBSTWEmz3Nh
}

// rand = "0.8"
