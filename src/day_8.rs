extern crate regex;

use std::collections::HashMap;
use regex::Regex;

mod input;

struct Instruction {
    register: String,
    operator: String,
    value: i32,
    cond_reg: String,
    cond_op: String,
    cond_val: i32
}

impl Instruction {
    pub fn new(line: String, re: &Regex) -> Instruction {
        for cap in re.captures_iter(&line[..]) {
            return Instruction {register: (&cap[1]).to_owned(),
                operator: (&cap[2]).to_owned(),
                value: (&cap[3]).parse::<i32>().unwrap(),
                cond_reg: (&cap[4]).to_owned(),
                cond_op: (&cap[5]).to_owned(),
                cond_val: (&cap[6]).parse::<i32>().unwrap(),
            };
        }
        return Instruction {register: "".to_owned(),
            operator: "".to_owned(),
            value: 0,
            cond_reg: "".to_owned(),
            cond_op: "".to_owned(),
            cond_val: 0,
        };
    }

}

fn main() {
    let input = input::file_to_string("day8-input");

    let re = Regex::new(r"^(\w+) (\w+) (.+) if (\w+) (.+) (.+)$").unwrap();
    let mut instructions = Vec::new();
    for row in input.split("\n") {
        let p = Instruction::new(row.to_owned(), &re);
        instructions.push(p);
    }

    let mut registers = HashMap::new();
    for i in &instructions {
        registers.insert(i.register.clone(), 0);
    }

    for i in &instructions {
        match i.cond_op.as_ref() {
            "<" => {
                if registers[&i.cond_reg] < i.cond_val {
                    let mut k = registers[&i.register];
                    if i.operator == "inc" {
                        k += i.value;
                    } else {
                        k -= i.value;
                    }
                    registers.insert(i.register.clone(), k);
                }
            }
            "<=" => {
                if registers[&i.cond_reg] <= i.cond_val {
                    let mut k = registers[&i.register];
                    if i.operator == "inc" {
                        k += i.value;
                    } else {
                        k -= i.value;
                    }
                    registers.insert(i.register.clone(), k);
                }
            }
            "==" => {
                if registers[&i.cond_reg] == i.cond_val {
                    let mut k = registers[&i.register];
                    if i.operator == "inc" {
                        k += i.value;
                    } else {
                        k -= i.value;
                    }
                    registers.insert(i.register.clone(), k);
                }
            }
            ">" => {
                if registers[&i.cond_reg] > i.cond_val {
                    let mut k = registers[&i.register];
                    if i.operator == "inc" {
                        k += i.value;
                    } else {
                        k -= i.value;
                    }
                    registers.insert(i.register.clone(), k);
                }
            }
            ">=" => {
                if registers[&i.cond_reg] >= i.cond_val {
                    let mut k = registers[&i.register];
                    if i.operator == "inc" {
                        k += i.value;
                    } else {
                        k -= i.value;
                    }
                    registers.insert(i.register.clone(), k);
                }
            }
            "!=" => {
                if registers[&i.cond_reg] != i.cond_val {
                    let mut k = registers[&i.register];
                    if i.operator == "inc" {
                        k += i.value;
                    } else {
                        k -= i.value;
                    }
                    registers.insert(i.register.clone(), k);
                }
            }
            _ => {
            }
        }
    }

    println!("answer day8 part1 = {0}", registers.values().max().unwrap());
}
