// regular expression
use regex::Regex;

fn main() {
     let re = Regex::new(r"^one").unwrap();
     // new function return a result
     // unwrap to handle the result
     let message = "one two three";
     let result = re.is_match(&message);
     println!("{}", result);
}
