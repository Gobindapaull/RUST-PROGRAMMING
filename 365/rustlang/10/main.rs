fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 32, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {result}");
}
