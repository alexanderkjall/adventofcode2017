mod input;

fn main() {
    let mut contents = input::file_to_string("day1-input");

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