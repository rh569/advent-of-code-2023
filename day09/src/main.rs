use std::fs;

fn get_histories(s: &String) -> Vec<Vec<i32>> {
    let mut histories = Vec::new();

    for l in s.lines() {
        histories.push(
            l.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    }

    histories
}

fn get_diffs(v: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = Vec::new();

    for i in 1..v.len() {
        diffs.push(v[i] - v[i - 1])
    }

    diffs
}

fn part_one(input: &String) -> i32 {
    let histories = get_histories(input);
    let mut next_values: Vec<i32> = Vec::new();

    for h in histories {
        let sequence: Vec<i32> = h.clone();
        let mut sequences: Vec<Vec<i32>> = Vec::from([sequence]);

        loop {
            let next_sequence: Vec<i32> = get_diffs(&sequences[sequences.len() - 1]);

            if next_sequence.iter().all(|n| *n == 0) {
                break;
            }

            sequences.push(next_sequence);
        }

        next_values.push(
            sequences.iter().map(|s| s[s.len() - 1]).sum(), //
        )
    }

    next_values.iter().sum()
}

fn part_two(input: &String) -> i32 {
    let histories = get_histories(input);
    let mut next_values: Vec<i32> = Vec::new();

    for h in histories {
        let sequence: Vec<i32> = h.clone();
        let mut sequences: Vec<Vec<i32>> = Vec::from([sequence]);

        loop {
            let next_sequence: Vec<i32> = get_diffs(&sequences[sequences.len() - 1]);

            if next_sequence.iter().all(|n| *n == 0) {
                break;
            }

            sequences.push(next_sequence);
        }

        let alternate = sequences
            .iter()
            .enumerate()
            .map(|(i, s)| if i % 2 == 0 { s[0] } else { -1 * s[0] })
            .collect::<Vec<i32>>();

        next_values.push(alternate.iter().sum());
    }

    next_values.iter().sum()
}

fn main() {
    let input = fs::read_to_string("../inputs/day09.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day09.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(114, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day09.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(2, part_two(&input))
}
