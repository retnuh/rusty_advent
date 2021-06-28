use md5;
use std::u32::MAX;

fn main() {
    let sekrit = "yzbqklnj";
    println!("Answer 5: {}", find_next_int(1, sekrit, "00000"));
    println!("Answer 6: {}", find_next_int(1, sekrit, "000000"));
}

fn find_next_int(start: u32, prefix: &str, pattern: &str) -> u32 {
    for i in start..MAX {
        let digest = md5::compute(format!("{}{}", prefix, i).as_bytes());
        let string = format!("{:x}", digest);
        if string.starts_with(pattern) {
            return i;
        }
    }
    return 0;
}

#[test]
fn test_stuff() {
    assert_eq!(609043, find_next_int(609043, "abcdef", "00000"));
    assert_eq!(609043, find_next_int(609000, "abcdef", "00000"));
    assert_eq!(1048970, find_next_int(1048970, "pqrstuv", "00000"));
    assert_eq!(1048970, find_next_int(1048000, "pqrstuv", "00000"));
}
