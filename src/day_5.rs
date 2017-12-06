mod input;

fn jump_around(mut instruction: Vec<i32>) -> i32 {
    let mut pos: i32 = 0;
    let mut step: i32 = 0;
    while pos < instruction.len() as i32 {
        let p = instruction[pos as usize].clone();
        instruction[pos as usize] = instruction[pos as usize] + 1;
        pos = pos + p;
        step += 1;
    }
    return step;
}

fn jump_around_type_2(mut instruction: Vec<i32>) -> i32 {
    let mut pos: i32 = 0;
    let mut step: i32 = 0;
    while pos < instruction.len() as i32 {
        let p = instruction[pos as usize].clone();
        if p > 2 {
            instruction[pos as usize] -= 1;
        } else {
            instruction[pos as usize] += 1;
        }
        pos = pos + p;
        step += 1;
    }
    return step;
}

fn main() {
    let contents = input::file_to_string("day5-input");

    let rows = contents.split("\n");

    let mut instruction: Vec<i32> = vec![];

    for row in rows {
        if row.len() == 0 {
            continue;
        }
        instruction.push(row.parse::<i32>().unwrap())
    }

    println!("answer day5 part1 = {0}", jump_around(instruction.clone()));
    println!("answer day5 part2 = {0}", jump_around_type_2(instruction));
}
