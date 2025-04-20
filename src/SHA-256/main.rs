use digest::{Digest, Output};
use sha2::Sha256;

fn main() {
    let mut hasher = Sha256::new();
    let input_data = b"hello world";
    hasher.update(input_data);
    let result: Output<Sha256> = hasher.finalize();
    // Print the hash
    println!("SHA-256 hash: {:?}", result);
    println!("SHA-256 hash: {:x}", result);
}
