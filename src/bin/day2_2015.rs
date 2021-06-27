use std::fs;
use std::str::FromStr;

fn main() {
    let input = fs::read_to_string("inputs/2015/day2.txt").unwrap();
    println!("Paper: {}", input.lines().fold(0, |t, f| t + needed(f)));
    println!("Ribbon: {}", input.lines().fold(0, |t, f| t + ribbon(f)));
}

fn needed(dimstr: &str) -> u32 {
    let dims: Vec<u32> = dimstr.split('x')
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<u32>>();
    let sides = [dims[0] * dims[1], dims[1] * dims[2], dims[0] * dims[2]];
    let smallest: &u32 = sides.iter().min().unwrap();
    return sides.iter().fold(*smallest, |t, i| t + 2 * i);
}

fn ribbon(dimstr: &str) -> u32 {
    let dims: Vec<u32> = dimstr.split('x')
        .map(|s| u32::from_str(s).unwrap())
        .collect::<Vec<u32>>();
    let perimeters = [2 * (dims[0] + dims[1]), 2 * (dims[1] + dims[2]), 2 * (dims[0] + dims[2])];
    let smallest: &u32 = perimeters.iter().min().unwrap();
    return dims.iter().product::<u32>() + *smallest;
}

#[test]
fn test_needed() {
    assert_eq!(58, needed("2x3x4"));
    assert_eq!(43, needed("1x1x10"));
}

#[test]
fn test_ribbon() {
    assert_eq!(34, ribbon("2x3x4"));
    assert_eq!(34, ribbon("2x4x3"));
    assert_eq!(34, ribbon("4x3x2"));
    assert_eq!(14, ribbon("1x1x10"));
    assert_eq!(14, ribbon("1x10x1"));
    assert_eq!(14, ribbon("10x1x1"));
}
