use std::fs;
use std::str::FromStr;

use regex::Regex;

trait Light {
    fn on(&mut self);
    fn off(&mut self);
    fn toggle(&mut self);

    fn apply(&mut self, op: &str) {
        match op {
            "turn on" => self.on(),
            "turn off" => self.off(),
            "toggle" => self.toggle(),
            _ => panic!("Unknown operation: '{}'", op),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Part1Light(u32);

#[derive(Debug, Copy, Clone, Default)]
struct Part2Light(u32);

impl Light for Part2Light {
    fn on(&mut self) {
        self.0 += 1;
    }

    fn off(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }

    fn toggle(&mut self) {
        self.0 += 2;
    }
}

impl Light for Part1Light {
    fn on(&mut self) {
        self.0 = 1;
    }

    fn off(&mut self) {
        self.0 = 0;
    }

    fn toggle(&mut self) {
        self.0 ^= 1;
    }
}

#[test]
fn test_part1_light() {
    let mut l = Part1Light(0);
    l.toggle();
    assert_eq!(1, l.0);
    l.toggle();
    assert_eq!(0, l.0);
    l.on();
    assert_eq!(1, l.0);
    l.off();
    assert_eq!(0, l.0);
}

#[test]
fn test_part2_light() {
    let mut l = Part2Light(1);
    l.off();
    assert_eq!(0, l.0);
    l.off();
    assert_eq!(0, l.0);
    l.off();
    assert_eq!(0, l.0);
    l.toggle();
    assert_eq!(2, l.0);
    l.toggle();
    assert_eq!(4, l.0);
    l.on();
    assert_eq!(5, l.0);
    l.off();
    assert_eq!(4, l.0);
}

fn main() {
    let input = fs::read_to_string("inputs/2015/day6.txt").unwrap();
    let parser: Regex =
        Regex::new(r"(turn (?:on|off)|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut part1_grid = vec![Part1Light(0); 1_000_000];
    let mut part2_grid = vec![Part2Light(0); 1_000_000];
    for line in input.lines() {
        let captures = parser.captures(line).unwrap();
        let start_x: usize = usize::from_str(captures.get(2).unwrap().as_str()).unwrap();
        let start_y: usize = usize::from_str(captures.get(3).unwrap().as_str()).unwrap();
        let end_x: usize = usize::from_str(captures.get(4).unwrap().as_str()).unwrap();
        let end_y: usize = usize::from_str(captures.get(5).unwrap().as_str()).unwrap();
        let op = captures.get(1).unwrap().as_str();
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                &mut part1_grid[y * 1000 + x].apply(op);
                &mut part2_grid[y * 1000 + x].apply(op);
            }
        }
    }
    let mut part1_on = 0;
    let mut part2_brightness = 0;
    for y in 0..1000 {
        for x in 0..1000 {
            part1_on += part1_grid[y * 1000 + x].0;
            part2_brightness += part2_grid[y * 1000 + x].0;
        }
    }
    println!("Part1 on: {}", part1_on);
    println!("Part2 brightness: {}", part2_brightness);
}
