fn main() {
    let min = i32::MIN;
    let max = i32::MAX;
    println!("{min}"); // -2147483648
    println!("{max}"); // 2147483647

    let mut u_max = u32::MAX;
    u_max += 1;
    println!("{u_max}"); // cargo run --release // 0
}
