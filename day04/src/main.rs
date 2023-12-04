use std::fs;

struct ScratchCard {
    winning: Vec<u8>,
    numbers: Vec<u8>,
}

fn get_cards(s: &String) -> Vec<ScratchCard> {
    s.lines()
        .map(|l| l.split(": ").collect::<Vec<&str>>()[1])
        .map(|l| {
            let [winning, numbers] = l.split(" | ").collect::<Vec<&str>>()[0..=1] else {
                panic!("Expected two parts in string around ' | '")
            };
            (winning, numbers)
        })
        .map(|(winning, numbers)| {
            let _winning = winning
                .split(" ")
                .filter(|s| s.len() > 0)
                .map(|s| s.trim())
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let _numbers = numbers
                .split(" ")
                .filter(|s| s.len() > 0)
                .map(|s| s.trim())
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            ScratchCard {
                winning: _winning,
                numbers: _numbers,
            }
        })
        .collect()
}

fn get_win_count(c: &ScratchCard) -> usize {
    c.numbers.iter().filter(|n| c.winning.contains(n)).count()
}

fn get_points(c: &ScratchCard) -> u32 {
    let win_count = get_win_count(&c);

    match win_count {
        0 => 0,
        _ => 2_u32.pow((win_count - 1).try_into().unwrap()),
    }
}

fn part_one(input: &String) -> u32 {
    get_cards(input).iter().map(|c| get_points(&c)).sum()
}

fn part_two(input: &String) -> u32 {
    let card_points = get_cards(input)
        .iter()
        .map(|c| get_win_count(&c))
        .collect::<Vec<usize>>();
    let mut card_counts: Vec<u32> = card_points.iter().map(|_| 1).collect::<Vec<u32>>();

    for (i, p) in card_points.iter().enumerate() {
        let this_card_copies = card_counts[i];

        for j in i + 1..=i + *p {
            card_counts[j] += this_card_copies;
        }
    }

    card_counts.iter().sum()
}

fn main() {
    let input = fs::read_to_string("../inputs/day04.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day04.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(13, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day04.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(30, part_two(&input))
}
