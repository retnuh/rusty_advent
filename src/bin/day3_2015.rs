use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2015/day3.txt").unwrap();
    println!("Houses: {}", houses(&input));
    println!("Robo Houses: {}", robo_houses(&input));
    // println!("Ribbon: {}", input.lines().fold(0, |t, f| t + ribbon(f)));
}

fn houses(dirs: &String) -> usize {
    let mut seen = HashSet::new();
    let mut pos = (0, 0);
    seen.insert(pos.clone());
    for d in dirs.chars() {
        match d {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => panic!("unexpected char: {}",d)
        };
        seen.insert(pos.clone());
    }
    return seen.len();
}

#[test]
fn test_houses() {
    assert_eq!(2, houses(&">".to_string()));
    assert_eq!(4, houses(&"^>v<".to_string()));
    assert_eq!(2, houses(&"^v^v^v^v^v".to_string()));
}

fn robo_houses(dirs: &String) -> usize {
    let mut seen = HashSet::new();
    let mut santa = (0, 0);
    let mut robo = (0, 0);
    let mut santas_turn = true;
    seen.insert(santa.clone());
    for d in dirs.chars() {
        let mut pos = if santas_turn { &mut santa } else { &mut robo };
        match d {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => panic!("unexpected char: {}",d)
        };
        seen.insert(pos.clone());
        santas_turn = !santas_turn;
    }
    return seen.len();
}

#[test]
fn test_robo_houses() {
    assert_eq!(3, robo_houses(&"^v".to_string()));
    assert_eq!(3, robo_houses(&"^>v<".to_string()));
    assert_eq!(11, robo_houses(&"^v^v^v^v^v".to_string()));
}
