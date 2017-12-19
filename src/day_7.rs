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

fn calc_weight(p: &String, programs: &HashMap<String, Program>, lvl: i32) -> i32 {
    let mut w = 0;

    let mut weights = HashMap::new();
    let mut weight_nums = HashMap::new();
    let mut weight_names = HashMap::new();
    for child in &programs[p].carry {
        let cw = calc_weight(child, &programs, lvl + 1);
/*
        for _ in 0..lvl {
            print!("    ");
        }
        println!("{0} has weight {1}", child, cw);
*/
        weights.insert(child.clone(), cw);
        let k = weight_nums.get(&cw).unwrap_or(&0) + 1;
        weight_nums.insert(cw, k);
        weight_names.insert(cw, child);
        w += cw;
    }
    if weight_nums.len() > 1 {
        let mut correct_val = 0;
        let mut wrong_val = 0;
        for (val, count) in weight_nums.clone() {
            if count != 1 {
                correct_val = val;
            } else {
                wrong_val = val;
            }
        }
        for (val, count) in weight_nums {
            let prog = &programs[weight_names.get(&val).unwrap().clone()];
            if count != 1 {
            } else {
                println!("answer day7 part2 = {0}", prog.weight - (wrong_val - correct_val));
            }
        }
    }
    w += programs[p].weight;

    return w;
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
            carried_by.insert(c, p.name.clone());
        }
    }

    let mut c = carried_by.keys().nth(0).unwrap().clone();
    while carried_by.contains_key(&c) {
        c = carried_by[&c].clone().to_owned();
    }

    for child in &programs[&c].carry {
        calc_weight(child, &programs, 1);
    }

    println!("answer day7 part1 = {0}", c);
}
