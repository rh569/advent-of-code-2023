use std::fs;

struct Race(u32, u32);
struct BigRace(u64, u64);

fn get_races(input: &str) -> Vec<Race> {
    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();

    for l in input.lines() {
        let parts = l.split(" ").collect::<Vec<&str>>().clone();
        let part_type = parts[0];
        let values = parts[1..]
            .iter()
            .map(|p| p.trim())
            .filter(|p| p.len() > 0)
            .map(|p| p.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if part_type.contains("Time") {
            times = values;
        } else {
            distances = values;
        }
    }

    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Race(*t, *d))
        .collect::<Vec<Race>>()
}

fn get_big_race(input: &str) -> BigRace {
    let mut time = 0;
    let mut distance = 0;

    for l in input.lines() {
        let parts = l.split(" ").collect::<Vec<&str>>().clone();
        let part_type = parts[0];
        let value: String = parts[1..]
            .iter()
            .map(|p| p.trim())
            .filter(|p| p.len() > 0)
            .map(|s| s.chars())
            .flatten()
            .collect();

        if part_type.contains("Time") {
            time = value.parse::<u64>().unwrap_or(0);
        } else {
            distance = value.parse::<u64>().unwrap_or(0);
        }
    }

    BigRace(time, distance)
}

fn part_one(input: &String) -> u32 {
    let races = get_races(input);

    races
        .iter()
        .map(|race| {
            let mid = race.0 / 2;
            let count_further_distances: u32 = (1..=mid)
                .map(|ct| ct * (race.0 - ct))
                .filter(|d| *d > race.1)
                .count()
                .try_into()
                .unwrap();
            if race.0 % 2 == 0 {
                count_further_distances * 2 - 1
            } else {
                count_further_distances * 2
            }
        })
        .product()
}

fn part_two(input: &String) -> u64 {
    let race = get_big_race(input);

    let mid = race.0 / 2;
    let count_further_distances: u64 = (1..=mid)
        .map(|ct| ct * (race.0 - ct))
        .filter(|d| *d > race.1)
        .count()
        .try_into()
        .unwrap();
    if race.0 % 2 == 0 {
        count_further_distances * 2 - 1
    } else {
        count_further_distances * 2
    }
}

fn main() {
    let input = fs::read_to_string("../inputs/day06.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day06.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(288, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day06.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(71503, part_two(&input))
}
