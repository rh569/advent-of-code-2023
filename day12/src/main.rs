use std::fs;

struct ConditionRecord {
    code: Vec<u8>,
    record: Vec<char>,
    unknowns: Vec<usize>,
}

fn get_possible_records(record: &Vec<char>, unknowns: &Vec<usize>) -> Vec<Vec<char>> {
    let mut possible_records: Vec<Vec<char>> = Vec::from([record.clone()]);

    for i in 0..unknowns.len() {
        let u = unknowns[i];
        let mut new_possible_records: Vec<Vec<char>> = Vec::new();

        for r in possible_records.iter() {
            for c in ['#', '.'] {
                let mut new_record = r.clone();
                new_record[u] = c;
                new_possible_records.push(new_record);
            }
        }

        possible_records = new_possible_records
    }

    possible_records
}

fn is_valid_record(r: &Vec<char>, code: &Vec<u8>) -> bool {
    let damaged_count = r.iter().filter(|c| **c == '#').count() as u8;
    let code_sum = code.iter().map(|n| *n).sum();

    if damaged_count != code_sum {
        return false;
    }

    let mut code_pos: usize = 0;
    let mut record_pos: usize = 0;

    while record_pos < r.len() {
        let c = r[record_pos];

        if c == '.' {
            // nothing here, continue
            record_pos += 1;
        } else {
            // start of a damaged sequence
            let length = code[code_pos] as usize;

            for offset in 1..length {
                if record_pos + offset >= r.len() {
                    // reached the end of the record with more damaged springs expected
                    return false;
                }

                if r[record_pos + offset] != '#' {
                    return false;
                }
            }

            record_pos += length;
            code_pos += 1;

            // also check next spring is not damaged if not last
            if record_pos < r.len() {
                if r[record_pos] == '#' {
                    return false;
                }
            }
        }
    }

    // only valid if we've also checked every value in the code
    code_pos == code.len()
}

fn get_spring_records(input: &str) -> Vec<ConditionRecord> {
    input
        .lines()
        .map(|l| {
            let [record_str, code_str] = l.split(" ").collect::<Vec<&str>>()[0..=1] else {
                panic!("Line did not have two parts: {}", l);
            };

            (
                record_str.chars().collect::<Vec<char>>(),
                code_str
                    .split(",")
                    .map(|s| s.parse::<u8>().unwrap())
                    .collect(),
            )
        })
        .map(|(record, code)| {
            let mut unknowns: Vec<usize> = Vec::new();
            for i in 0..record.len() {
                if record[i] == '?' {
                    unknowns.push(i);
                }
            }

            ConditionRecord {
                code,
                record,
                unknowns,
            }
        })
        .collect()
}

fn part_one(input: &String) -> u32 {
    let records = get_spring_records(input);
    let mut permutation_counts: Vec<u32> = Vec::new();

    for i in 0..records.len() {
        permutation_counts.push({
            get_possible_records(&records[i].record, &records[i].unknowns)
                .iter()
                .filter(|r| is_valid_record(*r, &records[i].code))
                .count() as u32
        })
    }

    permutation_counts.iter().sum()
}

fn part_two(input: &String) -> u32 {
    let _ = get_spring_records(input);

    0
}

fn main() {
    let input = fs::read_to_string("../inputs/day12.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day12.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(21, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day12.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(1, part_two(&input))
}
