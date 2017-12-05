#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Spiral {
    curr: Point,
    layer: i32,
    leg: i32,
}

impl Spiral {
    pub fn new() -> Spiral {
        Spiral {
            curr: Point {
                x: 0,
                y: 0
            },
            layer: 1,
            leg: 0,
        }
    }
}

// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Spiral {
    type Item = Point;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Point> {

        match self.leg {
            0 => {
                self.curr.x += 1;
                if self.curr.x == self.layer {
                    self.leg += 1;
                }
            }
            1 => {
                self.curr.y += 1;
                if self.curr.y == self.layer {
                    self.leg += 1;
                }
            }
            2 => {
                self.curr.x -= 1;
                if -self.curr.x == self.layer {
                    self.leg += 1;
                }
            }
            3 => {
                self.curr.y -= 1;
                if -self.curr.y == self.layer {
                    self.leg = 0;
                    self.layer += 1;
                }
            }
            _ => {
                panic!("wrong leg");
            }
        }

        Some(self.curr)
    }
}

fn point_at_pos_in_spiral(pos: i32) -> Point {
    let mut sp = Spiral::new();

    for _ in 0..(pos - 2) {
        sp.next();
    }

    return sp.next().unwrap();
}

fn main() {
    let p = point_at_pos_in_spiral(312051);

    println!("answer day3 part1 = {0}", p.x.abs() + p.y.abs());
}
