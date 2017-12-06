mod input;

fn main() {
    let contents = input::file_to_string("day4-input");

    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;

    let rows = contents.split("\n");

    for row in rows {
        if row.len() == 0 {
            continue;
        }
        let words = row.split_whitespace().collect::<Vec<&str>>();
        let mut unique_words = std::collections::HashSet::new();
        let mut unique_sorted_words = std::collections::HashSet::new();

        for w in words.iter() {
            unique_words.insert(w);
            let mut chars: Vec<char> = w.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));
            unique_sorted_words.insert(chars);
        }

        if words.len() == unique_words.len() {
            sum_part_1 += 1;
        }

        if words.len() == unique_sorted_words.len() {
            sum_part_2 += 1;
        }
    }
    println!("answer day4 part1 = {0}", sum_part_1);
    println!("answer day4 part2 = {0}", sum_part_2);
}
