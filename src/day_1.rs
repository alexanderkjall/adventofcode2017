use std::io::prelude::*;

fn main() {
    let mut f = std::fs::File::open("day1-input").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.pop();

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;

    const RADIX: u32 = 10;
    for (i, c) in contents.chars().enumerate() {
        let next_c_1;
        let next_c_2 = contents.chars().nth((i + (contents.len() / 2)) % contents.len()).unwrap();
        if i + 1 != contents.len() {
            next_c_1 = contents.chars().nth(i + 1).unwrap();
        } else {
            next_c_1 = contents.chars().nth(0).unwrap();
        }

        if c == next_c_1 {
            sum_part_1 += c.to_digit(RADIX).unwrap();
        }
        if c == next_c_2 {
            sum_part_2 += c.to_digit(RADIX).unwrap();
        }

    }
    println!("answer day1 part1 = {0}", sum_part_1);
    println!("answer day1 part2 = {0}", sum_part_2);
}