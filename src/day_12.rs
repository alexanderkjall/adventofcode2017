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

fn find_group_size(pipes:&Vec<Pipe>, start:String) -> HashSet<String> {
    let mut to_investigate:Vec<String> = Vec::new();
    let mut investigated:HashSet<String> = HashSet::new();

    to_investigate.push(start);

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

    return investigated;
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

    let investigated = find_group_size(&pipes, "0".to_owned());

    println!("answer day12 part1 = {0}", investigated.len());

    let mut all_names:HashSet<String> = HashSet::new();
    for p in &pipes {
        all_names.insert(p.from.clone());
    }

    let mut groups = 0;
    while all_names.len() > 0 {
        let names = find_group_size(&pipes, all_names.iter().nth(0).unwrap().clone());
        for n in names {
            all_names.remove(&n);
        }
        groups += 1;
    }

    println!("answer day12 part2 = {0}", groups);
}