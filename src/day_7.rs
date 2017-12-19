extern crate regex;

use std::collections::HashMap;
use regex::Regex;

mod input;

struct Program {
    name: String,
    weight: i32,
    carry: Vec<String>,
}

impl Program {
    fn parse_to_vec(line: &str) -> Vec<String> {
        if line.len() == 0 {
            return Vec::new::<>();
        }
        //print!(" -> {0}\n", line);
        return line.split(", ").map(|x| x.to_owned()).collect::<Vec<String>>()
    }

    pub fn new(line: String, re: &Regex) -> Program {
        for cap in re.captures_iter(&line[..]) {
            //print!("{0} ", &cap[1]);
            return Program {name: (&cap[1]).to_owned(),
                weight: (&cap[2]).parse::<i32>().unwrap(),
                carry: Program::parse_to_vec(cap.get(4).map_or("", |m| m.as_str())),
            };
        }
        return Program {name: String::new(), weight: 2, carry: Vec::new::<>()};
    }
}

fn main() {
    let input = input::file_to_string("day7-input");
    let re = Regex::new(r"^(\w+) \((\d+)\)( -> (.*))?$").unwrap();

    let mut programs = HashMap::new();
    for row in input.split("\n") {
        let p = Program::new(row.to_owned(), &re);
        programs.insert(p.name.clone(), p);
    }

    let mut carried_by = HashMap::new();
    for p in programs.values() {
        for c in p.carry.clone() {
            //print!("{0} carried by {1}\n", c, p.name.clone());
            carried_by.insert(c, p.name.clone());
        }
    }

    let mut c = carried_by.keys().nth(0).unwrap().clone();
    while carried_by.contains_key(&c) {
        c = carried_by[&c].clone().to_owned();
    }

    println!("answer day7 part1 = {0}", c);
}
