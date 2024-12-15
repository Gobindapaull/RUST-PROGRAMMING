// How to create temporary Files or Directories

// [dependencies]
//  tempfile = "3.12.0"

use tempfile::Builder;
use std::{io::Write, thread, time::Duration};

fn main() {
    let folder = Builder::new().prefix("temp").tempdir_in("./").unwrap(); // to create a new builder of tempfile 
    // unwrap to handle the result
    let path = folder.path().display().to_string();
    // path method on folder to get the path
    let mut file = Builder::new().prefix("hello").suffix(".txt").tempfile_in(path).unwrap();
    write!(file, "Hello world!").unwrap();
    // write! macro to write some content in the temporary file
    // Write trait from std::io
    thread::sleep(Duration::from_secs(10));
}
