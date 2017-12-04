use std::io::prelude::*;

fn main() {
    let mut f = std::fs::File::open("day1-input").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.pop();

    let mut sum = 0;

    const RADIX: u32 = 10;
    for (i, c) in contents.chars().enumerate() {
        let next_c;
        if i + 1 != contents.len() {
            next_c = contents.chars().nth(i + 1).unwrap();
        } else {
            next_c = contents.chars().nth(0).unwrap();
        }

        if c == next_c {
            sum += c.to_digit(RADIX).unwrap();
        }

    }
    println!("answer day1 part1 = {0}", sum);
}