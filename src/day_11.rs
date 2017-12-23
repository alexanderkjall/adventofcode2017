mod input;

struct Point {
    x:i32,
    y:i32,
    z:i32
}

fn cube_distance(a:&Point, b:&Point) -> i32 {
    return ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) / 2
}

fn calc_end(input:&String) -> (Point, i32) {
    let mut start = Point { x:0, y:0, z:0 };

    let mut max = 0;

    for st in input.split(",") {
        match st {
            "n" => {
                start.x += 1;
                start.z -= 1;
            }
            "nw" => {
                start.y += 1;
                start.z -= 1;
            }
            "ne" => {
                start.y -= 1;
                start.x += 1;
            }
            "sw" => {
                start.y += 1;
                start.x -= 1;
            }
            "se" => {
                start.y -= 1;
                start.z += 1;
            }
            "s" => {
                start.x -= 1;
                start.z += 1;
            }
            _ => {
                println!("no match! {0}", st);
            }
        }
        let dist = cube_distance(&start, &Point { x:0, y:0, z:0 });
        if(dist > max) {
            max = dist;
        }

    }

    return (start, max);
}

fn main() {
    let input = input::file_to_string("day11-input");

    let (start, max) = calc_end(&input);

    println!("answer day11 part1 = {0}", cube_distance(&start, &Point { x:0, y:0, z:0 }));
    println!("answer day11 part2 = {0}", max);
}