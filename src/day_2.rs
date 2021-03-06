mod input;

fn main() {
    let contents = input::file_to_string("day2-input");

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;

    let rows = contents.split("\n");
    for row in rows {
        if row.len() == 0 {
            continue;
        }
        let nums = row.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let max = nums.iter().max().unwrap();
        let min = nums.iter().min().unwrap();
        sum_part_1 += max - min;

        for n1 in nums.iter() {
            for n2 in nums.iter() {
                if n1 != n2 && n1 % n2 == 0 {
                    sum_part_2 += n1 / n2;
                }
            }
        }

    }

    println!("answer day2 part1 = {0}", sum_part_1);
    println!("answer day2 part2 = {0}", sum_part_2);

}