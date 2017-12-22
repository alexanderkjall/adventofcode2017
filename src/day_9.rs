mod input;

fn calc(input: String) -> (i32, i32) {
    let mut score = 0;
    let mut cur_lvl = 0;
    let mut garbage = false;
    let mut cont = false;
    let mut garbage_count = 0;

    for c in input.chars() {
        if cont {
            cont = false;
        } else {
            match c {
                '{' => {
                    if !garbage {
                        cur_lvl += 1;
                    } else {
                        garbage_count += 1;
                    }
                }
                '}' => {
                    if !garbage {
                        score += cur_lvl;
                        cur_lvl -= 1;
                    } else {
                        garbage_count += 1;
                    }
                }
                '!' => {
                    if garbage {
                        cont = true;
                    }
                }
                '<' => {
                    if !garbage {
                        garbage = true;
                    } else {
                        garbage_count += 1;
                    }
                }
                '>' => {
                    if garbage {
                        garbage = false;
                    }
                }
                _ => {
                    if garbage {
                        garbage_count += 1;
                    }
                }
            }
        }
    }

    return (score, garbage_count);
}

fn main() {
    let input = input::file_to_string("day9-input");

    let (score, garbage_count) = calc(input);
    println!("answer day9 part1 = {0}", score);
    println!("answer day9 part2 = {0}", garbage_count);
}