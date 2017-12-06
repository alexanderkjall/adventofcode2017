use std::io::prelude::*;
use std;

pub fn file_to_string(name: &str) -> String {
    let mut f = std::fs::File::open(name).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}