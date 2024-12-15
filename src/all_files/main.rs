use std::fs;

fn main() {
    let path = fs::read_dir("./").unwrap();
    // read_dir to read a path of a directory
    // unwrap to handle the result
    for p in path {
        println!("{}", p.unwrap().path().display());
    }
}
