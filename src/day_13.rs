mod input;

struct Layer {
    depth: i32,
    range: i32,
    pos: i32,
    dir_down: bool,
}

impl Layer {
    fn new(depth:i32, range:i32) -> Layer {
        return Layer { depth: depth, range: range, pos: 0, dir_down: true };
    }

    fn step_time(self:&mut Layer, time:i32) -> i32 {
        let ret:i32 = match 0 == self.pos && time == self.depth {
            true => {
                self.depth * self.range
            }
            false => {
                0
            }
        };

        if self.dir_down {
            self.pos += 1;
            if self.pos == self.range - 1 {
                self.dir_down = false;
            }
        } else {
            self.pos -= 1;
            if self.pos == 0 {
                self.dir_down = true;
            }
        }

        return ret;
    }
}

fn calc_score(firewall:&mut Vec<Layer>) -> i32 {
    let max_time = firewall.iter().map(|x|x.depth + x.range).max().unwrap();
    let mut score = 0;
    for step in 0..max_time {
        for l in firewall.iter_mut() {
            score += l.step_time(step);
        }
    }

    return score;
}

fn main() {
    let input = input::file_to_string("day13-input");

    let mut firewall:Vec<Layer> = Vec::new();
    for l in input.split("\n") {
        let dr = l.split(": ").collect::<Vec<&str>>();
        firewall.push(Layer::new(dr[0].parse::<i32>().unwrap(), dr[1].parse::<i32>().unwrap()));
    }

    let score = calc_score(&mut firewall);

    println!("answer day13 part1 = {0}", score);
}