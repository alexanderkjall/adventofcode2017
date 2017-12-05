use std::io::prelude::*;

fn main() {
    let mut f = std::fs::File::open("day4-input").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut sum_part_1 = 0;

    let rows = contents.split("\n");

    for row in rows {
        if row.len() == 0 {
            continue;
        }
        let words = row.split_whitespace().collect::<Vec<&str>>();
        let mut unique_words = std::collections::HashSet::new();

        for w in words.iter() {
            unique_words.insert(w);
        }

        if words.len() == unique_words.len() {
            sum_part_1 += 1;
        }
    }
    println!("answer day4 part1 = {0}", sum_part_1);
}
