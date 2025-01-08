fn main() {
    let mut i = 0;
    loop {
        println!("loop : {i}");
        if i == 5 {
            break;
        }
        i += 1;
    }

    // for loop
    for i in 0..5 {
        println!("for loop : {i}");
    }
    // for array
    let arr = [1, 2, 3, 4, 5];
    for a in arr {
        println!("array : {a}");
    }
    // length
    let n = arr.len();
    println!("arr length: {n}")

}
