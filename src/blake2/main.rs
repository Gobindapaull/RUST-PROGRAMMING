use digest::Digest;
use blake2::Blake2b;

use generic_array::{GenericArray, typenum::U64};

fn main() {
    let mut hasher = Blake2b::new();
    let input_data = b"hello world";
    hasher.update(input_data);

    let result: GenericArray<u8, U64> = hasher.finalize();
    println!("Blake2b Hash in bytes: {:?}", result);
    println!("Blake2b Hash: {:x}", result);
}
