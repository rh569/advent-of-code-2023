use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

fn parse_map(input: &str) -> (Vec<usize>, HashMap<&str, [&str; 2]>) {
    let [ins_part, net_part] = input.split("\n\n").collect::<Vec<&str>>()[0..=1] else {
        panic!("Not two map parts")
    };

    let instructions: Vec<usize> = ins_part
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();

    let mut network: HashMap<&str, [&str; 2]> = HashMap::new();

    for l in net_part.lines() {
        network.insert(&l[0..3], [&l[7..10], &l[12..15]]);
    }

    (instructions, network)
}

fn gcd(a: u64, b: u64) -> u64 {
    let s = min(a, b);
    let l = max(a, b);

    let r = l % s;

    if r == 0 {
        s
    } else {
        gcd(s, r)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm_many(mut loop_lengths: Vec<u64>) -> u64 {
    loop_lengths.sort();

    for i in 0..loop_lengths.len() - 1 {
        loop_lengths[i + 1] = lcm(loop_lengths[i], loop_lengths[i + 1]);
    }

    loop_lengths[loop_lengths.len() - 1]
}

fn part_one(input: &String) -> u32 {
    let (instructions, network) = parse_map(input);
    let mut current_node = "AAA";
    let mut ins_idx = 0;
    let mut steps = 0;

    while current_node != "ZZZ" {
        current_node = network.get(current_node).unwrap()[instructions[ins_idx]];
        ins_idx = (ins_idx + 1) % instructions.len();
        steps += 1;
    }

    steps
}

fn part_two(input: &String) -> u64 {
    let (instructions, network) = parse_map(input);

    let mut starting_nodes: Vec<&str> = network
        .keys()
        .filter(|s| s.chars().collect::<Vec<char>>()[2] == 'A')
        .map(|s| *s)
        .collect();

    starting_nodes.sort();

    let mut loop_lengths: Vec<u64> = Vec::new();

    for i in 0..starting_nodes.len() {
        let mut ins_idx = 0;
        let mut steps = 0;
        let mut node = starting_nodes[i];

        while node.chars().collect::<Vec<char>>()[2] != 'Z' {
            node = network.get(node).unwrap()[instructions[ins_idx]];
            ins_idx = (ins_idx + 1) % instructions.len();
            steps += 1;
        }

        loop_lengths.push(steps);
    }

    lcm_many(loop_lengths)
}

fn main() {
    let input = fs::read_to_string("../inputs/day08.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day08.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(6, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day08_2.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(6, part_two(&input))
}

#[test]
fn test_gcd() {
    assert_eq!(1, gcd(8, 9))
}

#[test]
fn test_lcm() {
    assert_eq!(504, lcm_many(Vec::from([8, 9, 21])))
}
