use std::iter::FromIterator;
use std::usize::MAX;

const MY_INPUT: &str = "vzbxkghb";

fn main() {
    let s = next_password(MY_INPUT);
    println!("Part 1: {}", s);
    println!("Part 1: {}", next_password(s.as_str()));
}

fn next_password(password: &str) -> String {
    let mut valid: bool = false;
    let mut chars: Vec<char> = password.chars().collect();
    let end = chars.len() - 1;

    while !valid {
        increment(&mut chars, end);
        valid = is_valid(&chars);
    }
    return String::from_iter(chars.iter());
}

fn is_valid(s: &Vec<char>) -> bool {
    let mut run_of_three = false;
    let mut pairs = 0;
    let mut last_pair_end: usize = MAX;
    for i in 0..s.len() {
        if s[i] == 'i' || s[i] == 'l' || s[i] == 'o' {
            return false;
        }
        if i >= s.len() - 1 {
            break;
        }
        if s[i + 1] == s[i] && i != last_pair_end {
            pairs += 1;
            last_pair_end = i + 1;
        }
        if i >= s.len() - 2 {
            continue;
        }
        if s[i + 1] as u8 == s[i] as u8 + 1 && s[i + 2] as u8 == s[i + 1] as u8 + 1 {
            run_of_three = true;
        }
    }
    run_of_three && pairs >= 2
}

fn increment(s: &mut Vec<char>, pos: usize) {
    match s[pos] {
        'z' => {
            s[pos] = 'a';
            increment(s, pos - 1)
        }
        'h' | 'k' | 'n' => s[pos] = (s[pos] as u8 + 2) as char,
        _ => s[pos] = (s[pos] as u8 + 1) as char,
    }
}

#[test]
fn test_stuff() {
    assert_eq!(next_password("abcdefgh").as_str(), "abcdffaa");
    assert_eq!(next_password("ghijklmn").as_str(), "ghjaabcc");
}
