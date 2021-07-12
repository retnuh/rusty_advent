const MY_INPUT: &str = "3113322113";

fn main() {
    let mut s = MY_INPUT.to_string();
    for _ in 0..40 {
        s = look_and_say(&s)
    }
    println!("Part 1: {}", s.len());
    for _ in 0..10 {
        s = look_and_say(&s)
    }
    println!("Part 2: {}", s.len())
}

fn look_and_say(look: &str) -> String {
    let mut say = String::new();
    let chars: Vec<char> = look.chars().collect();
    let mut prev = chars[0];
    let mut count = 1;
    for i in 1..chars.len() {
        if chars[i] == prev {
            count += 1
        } else {
            say.push_str(format!("{}{}", count, prev).as_str());
            prev = chars[i];
            count = 1;
        }
    }
    say.push_str(format!("{}{}", count, prev).as_str());
    say
}

#[test]
fn test_look_and_say() {
    assert_eq!(look_and_say("1").as_str(), "11");
    assert_eq!(look_and_say("11").as_str(), "21");
    assert_eq!(look_and_say("21").as_str(), "1211");
    assert_eq!(look_and_say("1211").as_str(), "111221");
    assert_eq!(look_and_say("111221").as_str(), "312211");
}
