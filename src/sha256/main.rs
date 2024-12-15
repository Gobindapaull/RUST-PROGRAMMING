// cargo add sha2
// use sha2 crate to compute sha256 hash
use sha2::Digest;

fn main() {
    let mut hasher = sha2::Sha256::new();
    let data = b"hello world";
    hasher.update(data); // update method to update data to the hasher
    let result = hasher.finalize();
    println!("{:x}", result); // print the hashed value in hexadecimal format
    // b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
}
