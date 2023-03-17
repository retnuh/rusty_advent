use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2015/day9.txt").unwrap();
    let min_path_cost = find_path(&input, u32::MAX, |x, y| x < y);
    println!("Part 1: {}", min_path_cost);
    let max_path_cost = find_path(&input, u32::MIN, |x, y| x > y);
    println!("Part 2: {}", max_path_cost);
}

fn find_path(input: &str, worst: u32, better: impl Fn(u32, u32) -> bool) -> u32 {
    let regex = regex::Regex::new(r"(.+?) to (.+?) = (\d+)").unwrap();
    let edges: HashSet<(u32, &str, &str)> = input
        .lines()
        .map(|line| {
            let c = regex.captures(line).unwrap();
            (
                c.get(3).unwrap().as_str().parse().unwrap(),
                c.get(1).unwrap().as_str(),
                c.get(2).unwrap().as_str(),
            )
        })
        .collect();
    println!("Edges: {:?}", edges);
    let graph: HashMap<String, u32> = edges
        .iter()
        .flat_map(|&e| {
            vec![
                (format!("{}-{}", e.1, e.2), e.0),
                (format!("{}-{}", e.2, e.1), e.0),
            ]
        })
        .collect();
    let nodes: HashSet<&str> = edges.iter().flat_map(|&e| vec![e.1, e.2]).collect();
    println!("Nodes: {:?}", nodes);
    let perms: Vec<Vec<&str>> = nodes.iter().cloned().permutations(nodes.len()).collect();
    // println!("Perms: {:?}", perms);
    let mut seen: HashSet<Vec<&str>> = HashSet::new();
    let mut best_cost: u32 = worst;
    for path in perms {
        if seen.contains(&path) {
            // println!("Seen {:?} or it's reverse, skipping", path);
            continue;
        }
        let mut rev = path.clone();
        rev.reverse();
        seen.insert(path.clone());
        seen.insert(rev);
        let mut cost = 0;
        for route in path.windows(2) {
            cost += *graph.get(&format!("{}-{}", route[0], route[1])).unwrap();
        }
        if better(cost, best_cost) {
            println!("\t{} for route {:?}", cost, path);
            best_cost = cost;
        }
    }
    best_cost
}

#[test]
fn test_find_path() {
    let example = r"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141
"
    .to_string();
    assert_eq!(find_path(&example, u32::MAX, |x, y| x < y), 605);
    assert_eq!(find_path(&example, u32::MIN, |x, y| x > y), 982);
}
