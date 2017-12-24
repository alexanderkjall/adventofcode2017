extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

mod input;

struct Pipe {
    from: String,
    to: String
}

fn connections(pipes:&Vec<Pipe>, name:&String) -> Vec<String> {
    let mut to_ret:Vec<String> = Vec::new();

    for p in pipes {
        if p.from == *name {
            to_ret.push(p.to.clone())
        }
    }
    return to_ret;
}

fn main() {
    let input = input::file_to_string("day12-input");

    let re = Regex::new(r"^(\d+) <-> (.*)$").unwrap();

    let mut pipes:Vec<Pipe> = Vec::new();
    for row in input.split("\n") {
        for cap in re.captures_iter(&row[..]) {
            for to in cap[2].split(", ") {
                pipes.push(Pipe{ from: (&cap[1]).to_owned(), to: to.to_owned() });
            }
        }
    }

    let mut to_investigate:Vec<String> = Vec::new();
    let mut investigated:HashSet<String> = HashSet::new();

    to_investigate.push("0".to_owned());

    while to_investigate.len() > 0 {
        let investigate = to_investigate.pop().unwrap();

        if !investigated.contains(&investigate) {
            for name in connections(&pipes, &investigate) {
                if !investigated.contains(&name) {
                    to_investigate.push(name.clone());
                }
            }
        }
        investigated.insert(investigate);
    }

    println!("answer day12 part1 = {0}", investigated.len());
}