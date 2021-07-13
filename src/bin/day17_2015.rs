use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/2015/day17.txt").unwrap();
    let part1_out = part1(&input, 150);
    println!("Part 1: {}", part1_out);
    let part2_out = part2(&input, 150);
    println!("Part 2: {}", part2_out);
}

fn loop_containers_part1(containers: &Vec<u32>, index: usize, remaining: u32) -> u32 {
    if index == containers.len() - 1 {
        if containers[index] == remaining {
            1
        } else {
            0
        }
    } else {
        let without_me = loop_containers_part1(containers, index + 1, remaining);
        if containers[index] == remaining {
            1 + without_me
        } else if containers[index] > remaining {
            without_me
        } else {
            without_me + loop_containers_part1(containers, index + 1, remaining - containers[index])
        }
    }
}

fn part1(input: &String, amount: u32) -> u32 {
    let containers: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    loop_containers_part1(&containers, 0, amount)
}

type ContainerCount = u32;
type Capacity = u32;

fn loop_containers_part2(
    containers: &Vec<u32>,
    index: usize,
    remaining: Capacity,
    used: ContainerCount,
    combos: &mut Vec<ContainerCount>,
) {
    if index == containers.len() - 1 {
        if containers[index] == remaining {
            combos.push(used + 1)
        }
    } else {
        loop_containers_part2(containers, index + 1, remaining, used, combos);
        if containers[index] == remaining {
            combos.push(used + 1)
        } else if containers[index] < remaining {
            loop_containers_part2(
                containers,
                index + 1,
                remaining - containers[index],
                used + 1,
                combos,
            );
        }
    }
}

fn part2(input: &String, amount: u32) -> usize {
    let containers: Vec<u32> = input.lines().map(|l| l.parse::<u32>().unwrap()).collect();
    let mut combos = vec![];
    loop_containers_part2(&containers, 0, amount, 0, &mut combos);
    combos.sort_unstable();
    let min = combos.first().unwrap();
    combos.iter().take_while(|&x| x == min).count()
}

#[test]
fn test_stuff() {
    let example = "20\n15\n10\n5\n5".to_string();
    assert_eq!(part1(&example, 25), 4);
    assert_eq!(part2(&example, 25), 3);
}
