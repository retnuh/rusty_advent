#![feature(default_free_fn)]

use std::cmp::{max, min};
use std::fs;
use std::ops::{Add, Mul};

use lazy_static::lazy_static;
use num::Integer;
use regex::Regex;
use std::default::default;

fn main() {
    let input = fs::read_to_string("inputs/2015/day15.txt").unwrap();
    let part1_out = part1(&input, false);
    println!("Part 1: {}", part1_out);
    let part2_out = part1(&input, true);
    println!("Part 2: {}", part2_out);
}

#[derive(Debug, Default, Copy, Clone)]
struct Properties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Properties {
    fn score(&self) -> i32 {
        max(0, self.capacity) * max(0, self.durability) * max(0, self.flavor) * max(0, self.texture)
    }
}

impl Add for Properties {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            capacity: self.capacity + other.capacity,
            durability: self.durability + other.durability,
            flavor: self.flavor + other.flavor,
            texture: self.texture + other.texture,
            calories: self.calories + other.calories,
        }
    }
}

impl Mul<i32> for Properties {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

fn parse_line(line: &str) -> (String, Properties) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)",
        )
        .unwrap();
    }
    let c = RE.captures(line).unwrap();
    let p = Properties {
        capacity: c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        durability: c.get(3).unwrap().as_str().parse::<i32>().unwrap(),
        flavor: c.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        texture: c.get(5).unwrap().as_str().parse::<i32>().unwrap(),
        calories: c.get(6).unwrap().as_str().parse::<i32>().unwrap(),
    };
    (c.get(1).unwrap().as_str().to_owned(), p)
}

fn loop_ingredients(
    ingredients: &Vec<(String, Properties)>,
    index: usize,
    max_count: i32,
    current: &Properties,
    restricted: bool,
) -> Properties {
    if index == ingredients.len() - 1 {
        let just_me = (ingredients[index].1 * max_count);
        let with_me = *current + just_me;
        // println!(
        //     "\tLast: {} {} {} {} {:?} {:?}",
        //     max_count,
        //     ingredients[index].0,
        //     just_me.score(),
        //     with_me.score(),
        //     just_me,
        //     with_me,
        // );
        with_me
    } else {
        let mut best = *current;
        for count in 0..=max_count {
            let with_me = *current + (ingredients[index].1 * count);
            let best_this_count = loop_ingredients(
                ingredients,
                index + 1,
                max_count - count,
                &with_me,
                restricted,
            );
            // println!(
            //     "Current: {}, with_me: {}, best_this_count: {}, {}, {}",
            //     current.score(),
            //     with_me.score(),
            //     best_this_count.score(),
            //     count,
            //     ingredients[index].0
            // );
            if (!restricted || best_this_count.calories == 500)
                && best_this_count.score() > best.score()
            {
                println!(
                    "New best: {} {} {} {}",
                    count,
                    ingredients[index].0,
                    best_this_count.score(),
                    best.score()
                );
                best = best_this_count;
            }
        }
        best
    }
}

fn part1(input: &String, restricted: bool) -> i32 {
    let ingredients: Vec<(String, Properties)> = input.lines().map(parse_line).collect();
    println!("Ingredients: {:?}", ingredients);
    let default: Properties = default();
    let best = loop_ingredients(&ingredients, 0, 100, &default, restricted);
    return best.score();
}

#[test]
fn test_stuff() {
    let example = r"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
"
    .to_string();
    assert_eq!(part1(&example, false), 62842880);
    assert_eq!(part1(&example, true), 57600000);
}
