fn main() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }

    let res: Result<u32, String> = Ok(10);
    match res {
        Ok(val) => println!("{val}"),
        Err(msg) => println!("{msg}")
    }
}
