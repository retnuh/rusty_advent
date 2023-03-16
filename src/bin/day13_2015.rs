use std::collections::{HashMap, HashSet};
use std::fs;
use std::i32::MIN;
use std::io::{stdout, Write};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("inputs/2015/day13.txt").unwrap();
    let part1 = find_happiness(&input);
    let part2 = find_happiness_with_myself(&input);
    println!("Part 1: {}", part1);
    // Question asks for "change in happiness" but correct answer is total happiness
    println!("Part 2: {}", part2);
    println!("Part 2 - Part 1: {}", part2 - part1);
}

fn parse_line(line: &str) -> ((&str, &str), i32) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)\.",
        )
        .unwrap();
    }

    let c = REGEX.captures(line).unwrap();
    let mult = if c.get(2).unwrap().as_str() == "lose" {
        -1
    } else {
        1
    };
    let s = c.get(1).unwrap().as_str();
    let t = c.get(4).unwrap().as_str();
    let p = (s, t);
    let h = mult * c.get(3).unwrap().as_str().parse::<i32>().unwrap();
    (p, h)
}

fn find_happiness(input: &String) -> i32 {
    let feelings: HashMap<(&str, &str), i32> = input.lines().map(parse_line).collect();
    println!("Feelings: {:?}", feelings);
    let people: HashSet<&str> = feelings.keys().map(|x| x.0).collect();
    println!("People: {:?}", people);
    stdout().flush().unwrap();
    return calculate_buzz(feelings, people);
}

fn calculate_buzz(feelings: HashMap<(&str, &str), i32>, people: HashSet<&str>) -> i32 {
    let perms: Vec<Vec<&str>> = people
        .iter()
        .map(|&x| x)
        .permutations(people.len())
        .collect();
    // println!("Perms: {:?}", perms);
    let mut seen: HashSet<Vec<&str>> = HashSet::new();
    let mut best_buzz: i32 = MIN;
    for path in perms {
        if seen.contains(&path) {
            // println!("Seen {:?} or it's reverse, skipping", path);
            continue;
        }
        let mut rev = path.clone();
        rev.reverse();
        seen.insert(path.clone());
        seen.insert(rev);
        let mut buzz = 0;
        for pair in path.windows(2) {
            buzz += feelings.get(&(pair[0], pair[1])).unwrap();
            buzz += feelings.get(&(pair[1], pair[0])).unwrap();
        }
        buzz += feelings.get(&(path[0], path[path.len() - 1])).unwrap();
        buzz += feelings.get(&(path[path.len() - 1], path[0])).unwrap();
        if buzz > best_buzz {
            println!("\t{} for route {:?}", buzz, path);
            best_buzz = buzz;
        }
    }
    return best_buzz;
}

fn find_happiness_with_myself(input: &String) -> i32 {
    let mut feelings: HashMap<(&str, &str), i32> = input.lines().map(parse_line).collect();
    println!("Feelings: {:?}", feelings);
    let mut people: HashSet<&str> = feelings.keys().map(|x| x.0).collect();
    println!("People: {:?}", people);
    for person in &people {
        feelings.insert(("Me", *person), 0);
        feelings.insert((*person, "Me"), 0);
    }
    people.insert("Me");
    return calculate_buzz(feelings, people);
}

#[test]
fn test_find_happiness() {
    let example = r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
"
    .to_string();
    assert_eq!(find_happiness(&example), 330);
}
