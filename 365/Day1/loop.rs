fn main() {
    let mut a = 0;
    loop {
        a += 1;
        if a == 5 {
            println!("a is {a}");
            continue;
        }
        println!("{}", a);
        if a == 15 {
            break;
        }
    }
}
