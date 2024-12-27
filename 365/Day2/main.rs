#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}
fn main() {
    let lang = "rust";
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);
    println!("hello {lang}");

    let lang2 = Lang {
        language: "rust".to_string(),
        version: "1.83".to_string(),
    };
    println!("{:?}", lang2);
    println!("{:#?}", lang2);
}


// cargo fmt
// cargo run
