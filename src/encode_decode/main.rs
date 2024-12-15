// base64 to encode and decode data
// cargo add base64
use base64::prelude::*;

fn main() {
    let data = "Hello world";
    let encoded_data = BASE64_STANDARD.encode(data);
    println!("{}", encoded_data); // SGVsbG8gd29ybGQ=

    let decoded_data = BASE64_STANDARD.decode(encoded_data).unwrap();

    let result = String::from_utf8(decoded_data).unwrap();
    println!("{}", result); // Hello world
}
