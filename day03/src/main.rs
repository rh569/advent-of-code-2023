use std::fs;

struct Number {
    i: usize,
    j: usize,
    len: usize,
    value: u32,
}

type Schematic = Vec<Vec<char>>;
type GearPosition = (usize, usize);

fn get_schematic(s: &String) -> Schematic {
    let mut schematic: Schematic = Vec::new();

    for l in s.lines() {
        schematic.push(l.chars().collect())
    }

    schematic
}

fn find_numbers(s: &Schematic) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    for (i, l) in s.iter().enumerate() {
        let mut j: usize = 0;

        while j < l.len() {
            let is_digit = l[j].is_ascii_digit();

            if is_digit {
                let mut len: usize = 1;
                let mut val_str: String = String::from(l[j]);

                while j + len < l.len() && l[j + len].is_ascii_digit() {
                    val_str.push(l[j + len]);
                    len += 1;
                }

                numbers.push(Number {
                    i,
                    j,
                    len,
                    value: val_str.parse::<u32>().unwrap(),
                });

                j += len;
            } else {
                j += 1;
            }
        }
    }

    numbers
}

fn find_gear_positions(s: &Schematic) -> Vec<GearPosition> {
    let mut positions: Vec<GearPosition> = Vec::new();

    for (i, l) in s.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c == '*' {
                positions.push((i, j));
            }
        }
    }

    positions
}

fn is_part_number(n: &Number, s: &Schematic) -> bool {
    let mut found_symbol = false;

    'outer: for i in n.i.checked_sub(1).unwrap_or(0)..=n.i + 1 {
        if i >= s.len() {
            continue;
        }

        for j in n.j.checked_sub(1).unwrap_or(0)..=n.j + n.len {
            if j >= s[i].len() {
                continue;
            }

            match s[i][j] {
                '0'..='9' => (),
                '.' => (),
                _ => {
                    found_symbol = true;
                    break 'outer;
                }
            }
        }
    }

    found_symbol
}

fn is_adjacent(n: &Number, p: &GearPosition) -> bool {
    (p.0 >= n.i.checked_sub(1).unwrap_or(0) && p.0 <= n.i + 1)
        && (p.1 >= n.j.checked_sub(1).unwrap_or(0) && p.1 <= n.j + n.len)
}

fn get_gear_ratio(p: &GearPosition, numbers: &Vec<Number>) -> Option<u32> {
    let adj_numbers: Vec<&Number> = numbers.iter().filter(|n| is_adjacent(n, p)).collect();

    if adj_numbers.len() != 2 {
        return None;
    }

    Some(adj_numbers.iter().map(|n| n.value).product())
}

fn part_one(input: &String) -> u32 {
    let schematic = get_schematic(input);
    let numbers = find_numbers(&schematic);

    numbers
        .iter()
        .filter(|n| is_part_number(n, &schematic))
        .map(|n| n.value)
        .sum()
}

fn part_two(input: &String) -> u32 {
    let schematic = get_schematic(input);
    let numbers = find_numbers(&schematic);
    let gear_positions = find_gear_positions(&schematic);

    gear_positions
        .iter()
        .map(|p| get_gear_ratio(p, &numbers))
        .map(|r| r.unwrap_or(0))
        .sum()
}

fn main() {
    let input = fs::read_to_string("../inputs/day03.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day03.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(4361, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day03.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(467835, part_two(&input))
}
