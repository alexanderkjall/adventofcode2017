use std::collections::HashMap;

mod input;

fn max_pos(nums: &Vec<i32>) -> usize {
    let mut max:i32 = 0;
    let mut pos:usize = 0;

    for (i, item) in nums.iter().enumerate() {
        if *item > max {
            max = *item;
            pos = i;
        }
    }

    return pos;
}

fn redistribute(nums: &Vec<i32>) -> Vec<i32> {
    let mut new_nums = nums.clone();

    let max_pos = max_pos(&new_nums);
    let max = nums[max_pos];
    new_nums[max_pos] = 0;

    for i in 0..max {
        let p = (max_pos + i as usize + 1) % new_nums.len();
        new_nums[p] += 1;
    }

    return new_nums;
}

fn main() {
    let contents = input::file_to_string("day6-input");

    let mut nums = input::str_to_i32_vec(contents);

    let mut old_state = HashMap::new();

    let mut steps = 0;
    while !old_state.contains_key(&nums) {
        old_state.insert(nums.clone(), steps);
        nums = redistribute(&nums);
        steps += 1;
    }

    println!("answer day6 part1 = {0}", steps);
    println!("answer day6 part2 = {0}", steps - old_state.get(&nums).unwrap());

}