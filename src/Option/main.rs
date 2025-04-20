fn get_username(user_id: u32) -> Option<String> {
    if user_id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn main() {
    let name = get_username(1);

    match name {
        Some(username) => println!("Username is {}", username),
        None => println!("User not found")
    }
}
