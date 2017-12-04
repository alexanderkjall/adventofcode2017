use std::io::prelude::*;

fn main() {
    let mut f = std::fs::File::open("day2-input").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut sum_part_1 = 0;

    let rows = contents.split("\n");
    for row in rows {
        if row.len() == 0 {
            continue;
        }
        let max = row.split_whitespace().map(|x| x.parse::<i32>().unwrap()).max().unwrap();
        let min = row.split_whitespace().map(|x| x.parse::<i32>().unwrap()).min().unwrap();
        sum_part_1 += max - min;
    }

    println!("answer day2 part1 = {0}", sum_part_1);

}