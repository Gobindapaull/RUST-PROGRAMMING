
fn main() {
    let v1 = vec![1,2,3,4,5];
    let v2 = v1.iter().scan(0, |a, b| {
        *a += b;
        Some(*a)
    }).collect::<Vec<_>>();
    // scan method takes two arguments
    // collect method with turbo fish syntax to convert this iterator back to a vector
    println!("{:?}", v2); // [1, 3, 6, 10, 15]
}
