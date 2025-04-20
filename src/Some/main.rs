fn main() {
    let name: Option<&str> = Some("John Doe");
    let no_name: Option<&str> = None;
    
    match name {
        Some(name) => println!("Name : {}", name),
        None => println!("{:?}", no_name)
    }
}
