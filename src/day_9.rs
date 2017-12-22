mod input;

fn calc(input: String) -> i32 {
    let mut score = 0;
    let mut cur_lvl = 0;
    let mut garbage = false;
    let mut cont = false;

    for i in 0..input.len() {
        if(cont) {
            cont = false;
        } else {
            match input.chars().nth(i).unwrap() {
                '{' => {
                    if !garbage {
                        cur_lvl += 1;
                    }
                }
                '}' => {
                    if !garbage {
                        score += cur_lvl;
                        cur_lvl -= 1;
                    }
                }
                '!' => {
                    if garbage {
                        cont = true;
                    }
                }
                '<' => {
                    garbage = true;
                }
                '>' => {
                    garbage = false;
                }
                _ => {}
            }
        }
    }

    return score;
}

fn main() {
    let input = input::file_to_string("day9-input");

    println!("answer day9 part1 = {0}", calc(input));
}