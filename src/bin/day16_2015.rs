#![feature(default_free_fn)]

use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("inputs/2015/day16.txt").unwrap();
    let part1_analysis: HashMap<&str, i32> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .cloned()
    .collect();
    let part1_out = part1(&input, &part1_analysis);
    println!("Part 1 Sue {}", part1_out);
    let part2_out = part2(&input, &part1_analysis);
    println!("Part 2 Sue {}", part2_out);
}

fn parse_line(line: &str) -> (&str, HashMap<&str, i32>) {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)$",).unwrap();
    }
    let c = RE.captures(line).unwrap();
    let mut m = HashMap::new();
    m.insert(
        c.get(2).unwrap().as_str(),
        c.get(3).unwrap().as_str().parse::<i32>().unwrap(),
    );
    m.insert(
        c.get(4).unwrap().as_str(),
        c.get(5).unwrap().as_str().parse::<i32>().unwrap(),
    );
    m.insert(
        c.get(6).unwrap().as_str(),
        c.get(7).unwrap().as_str().parse::<i32>().unwrap(),
    );
    (c.get(1).unwrap().as_str(), m)
}

fn part1<'a>(input: &'a str, analysis: &HashMap<&str, i32>) -> &'a str {
    let sues: Vec<(&str, HashMap<&str, i32>)> = input.lines().map(parse_line).collect();
    let sue = sues
        .iter()
        .find(|&(_, m)| {
            m.iter()
                .fold(true, |c, (&k, v)| c && analysis.get(k).unwrap() == v)
        })
        .unwrap();
    sue.0
}

fn part2<'a>(input: &'a str, analysis: &HashMap<&str, i32>) -> &'a str {
    let sues: Vec<(&str, HashMap<&str, i32>)> = input.lines().map(parse_line).collect();
    let sue = sues
        .iter()
        .find(|&(_, m)| {
            m.iter().fold(true, |c, (&k, v)| {
                let av = analysis.get(k).unwrap();
                c && match k {
                    "cats" | "trees" => v > av,
                    "pomeranians" | "goldfish" => v < av,
                    _ => av == v,
                }
            })
        })
        .unwrap();
    sue.0
}

#[test]
fn test_stuff() {}
