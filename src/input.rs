use std::io::prelude::*;
use std;

pub fn file_to_string(name: &str) -> String {
    let mut f = std::fs::File::open(name).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

pub fn str_to_i32_vec(input: String) -> Vec<i32> {
    return input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
}

pub fn str_to_i32_vec_split(input: String, split: &String) -> Vec<i32> {
    return input.split(split).map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
}
