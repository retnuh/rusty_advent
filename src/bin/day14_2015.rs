use std::cmp::min;
use std::fs;

use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("inputs/2015/day14.txt").unwrap();
    let part1 = find_fastest(&input, 2503);
    println!("Part 1: {}", part1);
    let part2 = find_best_score(&input, 2503);
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    distance: u32,
    sprint: u32,
    sleep: u32,
    clock: i32,
    current_distance: u32,
    points: u32,
}

impl Reindeer {
    fn fly_for(&self, time: u32) -> u32 {
        let block = self.sprint + self.sleep;
        let (div, rem) = time.div_rem(&block);
        return self.distance * (div * self.sprint + min(rem, self.sprint));
    }

    fn tick(&mut self) {
        if self.clock > 0 {
            self.current_distance += self.distance;
            self.clock -= 1;
            if self.clock == 0 {
                self.clock = -(self.sleep as i32)
            }
        } else if self.clock < 0 {
            self.clock += 1;
            if self.clock == 0 {
                self.clock = self.sprint as i32
            }
        } else {
            panic!("Bad state: {:?}", self)
        }
    }

    fn score(&mut self) {
        self.points += 1
    }
}

fn parse_line(line: &str) -> Reindeer {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds\.",
        )
        .unwrap();
    }

    let c = RE.captures(line).unwrap();
    let sprint = c.get(3).unwrap().as_str().parse::<u32>().unwrap();
    return Reindeer {
        name: c.get(1).unwrap().as_str().to_owned(),
        distance: c.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        sprint,
        sleep: c.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        clock: sprint as i32,
        current_distance: 0,
        points: 0,
    };
}

fn find_fastest(input: &String, time: u32) -> u32 {
    let reindeer: Vec<Reindeer> = input.lines().map(parse_line).collect();
    println!("Reindeer: {:?}", reindeer);
    let distances: Vec<(Reindeer, u32)> = reindeer
        .into_iter()
        .map(|r| {
            let t = r.fly_for(time);
            (r, t)
        })
        .collect();
    let best = distances.iter().max_by_key(|&x| x.1).unwrap();
    println!("The best - {} - has flown {}", best.0.name, best.1);
    return best.1;
}

fn find_best_score(input: &String, time: u32) -> u32 {
    let mut reindeer: Vec<Reindeer> = input.lines().map(parse_line).collect();
    println!("Reindeer: {:?}", reindeer);
    for _t in 1..=time {
        reindeer.iter_mut().for_each(|r| r.tick());
        reindeer.sort_unstable_by_key(|r| -(r.current_distance as i32));
        let lead_distance = {
            let leader = reindeer.first().unwrap();
            // println!(
            //     "Leader at time {}: {} {} {}",
            //     _t, leader.name, leader.points, leader.current_distance
            // );
            leader.current_distance
        };
        reindeer
            .iter_mut()
            .take_while(|r| r.current_distance == lead_distance)
            .for_each(|r| r.score())
    }
    reindeer.sort_unstable_by_key(|r| -(r.points as i32));
    let best = reindeer.first().unwrap();
    println!(
        "Winner Winner Reindeer dinner: {} has {} points after travelling {}",
        best.name, best.points, best.current_distance
    );
    best.points
}

#[test]
fn test_find_fastest() {
    let example = r"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"
    .to_string();
    assert_eq!(find_fastest(&example, 1000), 1120);
    assert_eq!(find_best_score(&example, 1000), 689);
}
