use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2015/day8.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();
    let full = data.iter().fold(0, |t, s: &&str| t + (*s).len());
    let rendered = data
        .iter()
        .fold(0, |t, s: &&str| t + part1_render_count(*s));
    let encoded = data
        .iter()
        .fold(0, |t, s: &&str| t + part2_encode_count(*s));
    println!("Part 1: {}", full - rendered);
    println!("Part 2: {}", encoded - full);
}

fn part1_render_count(s: &str) -> usize {
    let cs: Vec<char> = s.chars().collect();
    let mut t = 0;
    let mut i: usize = 0;
    while i < cs.len() {
        match cs[i] {
            '\\' => match cs[i + 1] {
                '"' | '\\' => i += 2,
                'x' => i += 4,
                _ => panic!("Unknown escape: {}{} in {}", cs[i], cs[i + 1], s),
            },
            _ => i += 1,
        }
        t += 1
    }
    t - 2
}

fn part2_encode_count(s: &str) -> usize {
    let mut t = 2;
    for c in s.chars() {
        let size = match c {
            '"' | '\\' => 2,
            _ => 1,
        };
        t += size
    }
    t
}

#[test]
fn test_part1() {
    let input = fs::read_to_string("inputs/2015/day8.txt").unwrap();
    let data: Vec<&str> = input.lines().collect();
    assert_eq!(part2_encode_count("\"\""), 4);
    assert_eq!(data[0].len(), 16);
    assert_eq!(part1_render_count(data[1]), 2);
    assert_eq!(data[1].len(), 4);
}

#[test]
fn test_part2() {
    assert_eq!(part2_encode_count("\"\""), 6);
    assert_eq!(part2_encode_count("\"abc\""), 9);
    assert_eq!(part2_encode_count("\"aaa\\\"aaa\""), 16);
    assert_eq!(part2_encode_count("\"\\x27\""), 11);
}
