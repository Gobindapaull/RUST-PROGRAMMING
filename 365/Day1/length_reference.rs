fn main() {
   let s1 = String::from("Hello World");
   let length = get_length(&s1);
   println!("The length of {s1} is {length}");

}

fn get_length(s: &String) -> usize {
    return s.len();
}
