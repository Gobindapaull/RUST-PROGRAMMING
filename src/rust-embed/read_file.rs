use rust_embed::Embed;

fn main() {
    let file = Asset::get("hello.txt").unwrap();
    let data = file.data.to_vec(); // to_vec method to get the vector
    let content = String::from_utf8(data).unwrap();
    // convert this vector of bytes to a String
    println!("{}", content);
}

#[derive(Embed)]
#[folder = "test"]

struct Asset;

// [dependencies]
// rust-embed="8.4.0"
