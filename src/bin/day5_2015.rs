use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2015/day5.txt").unwrap();
    println!("Nice: {}", input.lines().map(|l| nice(l.trim())).sum::<u32>());
    println!("Moar Nice: {}", input.lines().map(|l| moar_nice(l.trim())).sum::<u32>());
}

fn nice(s: &str) -> u32 {
    let mut vowels = 0;
    let mut double = false;
    let mut prev = '\0';
    for c in s.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
            'b' if prev == 'a' => return 0,
            'd' if prev == 'c' => return 0,
            'q' if prev == 'p' => return 0,
            'y' if prev == 'x' => return 0,
            _ => {}
        }
        if c == prev {
            double = true;
        }
        prev = c;
    }
    if vowels >= 3 && double {
        return 1;
    }
    return 0;
}

#[test]
fn test_nice() {
    assert_eq!(1, nice("ugknbfddgicrmopn"));
    assert_eq!(1, nice("aaa"));
    assert_eq!(0, nice("jchzalrnumimnmhp"));
    assert_eq!(0, nice("haegwjzuvuyypxyu"));
    assert_eq!(0, nice("dvszwmarrgswjxmb"));
}

fn double_pair(chars: &Vec<char>) -> bool {
    let mut pairs: HashMap<&[char], usize> = HashMap::new();
    for (i, w) in chars.windows(2).enumerate() {
        match pairs.get(w) {
            Some(x) => return *x != (i - 1),
            None => {
                pairs.insert(w, i);
                ()
            }
        }
    }
    return false;
}

fn repeat_after_gap(chars: &Vec<char>) -> bool {
    for w in chars.windows(3) {
        if w[0] == w[2] {
            return true;
        }
    }
    return false;
}

fn moar_nice(s: &str) -> u32 {
    let chars: Vec<char> = s.chars().collect();
    let moar_nice = repeat_after_gap(&chars) && double_pair(&chars);
    return if moar_nice { 1 } else { 0 };
    // return moar_nice.into();
}

#[test]
fn test_moar_nice() {
    assert_eq!(1, moar_nice("qjhvhtzxzqqjkmpb"));
    assert_eq!(1, moar_nice("xxyxx"));
    assert_eq!(0, moar_nice("uurcxstgmygtbstg"));
    assert_eq!(0, moar_nice("ieodomkazucvgmuy"));
    assert_eq!(0, moar_nice("aaa"));
    assert_eq!(0, moar_nice(""));
    assert_eq!(1, moar_nice("abxyxab"));
    assert_eq!(1, moar_nice("abcabxyx"));
}
