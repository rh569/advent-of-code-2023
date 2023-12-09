use std::{cmp::min, collections::VecDeque, fs};

struct MapSection {
    dest_start: u64,
    src_start: u64,
    src_end: u64,
}

struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<(u64, u64)>,
    maps: Vec<Vec<MapSection>>,
}

fn parse_almanac(s: &String) -> Almanac {
    let mut parts = s.split("\n\n").collect::<VecDeque<&str>>();
    let seeds_part = parts.pop_front().unwrap();

    let seeds = seeds_part
        .split("seeds: ")
        .flat_map(|s| s.split(" "))
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();

    for (i, v) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            continue;
        }
        seed_ranges.push((seeds[i - 1], *v))
    }

    let mut maps: Vec<Vec<MapSection>> = Vec::new();

    for block_str in parts {
        let mut map: Vec<MapSection> = Vec::new();
        for l in block_str.lines() {
            if l.ends_with(":") {
                continue;
            }
            let nums = l
                .split(" ")
                .map(|s: &str| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            let ds = nums[0];
            let ss = nums[1];
            let r = nums[2];

            map.push(MapSection {
                dest_start: ds,
                src_start: ss,
                src_end: ss + r,
            })
        }

        maps.push(map)
    }

    Almanac {
        seeds,
        seed_ranges,
        maps,
    }
}

fn get_location_for_seed(s: u64, maps: &Vec<Vec<MapSection>>) -> u64 {
    let mut val = s;

    for m in maps {
        for ms in m {
            if val >= ms.src_start && val < ms.src_end {
                val = ms.dest_start + (val - ms.src_start);
                break;
            }
        }
    }

    val
}

fn derive_locations(a: &Almanac) -> Vec<u64> {
    a.seeds
        .iter()
        .map(|s| get_location_for_seed(*s, &a.maps))
        .collect::<Vec<u64>>()
}

fn derive_minimum_location(a: &Almanac) -> u64 {
    let mut min_loc: u64 = u64::MAX;

    for sr in a.seed_ranges.as_slice() {
        for s in sr.0..=sr.0 + sr.1 {
            min_loc = min(min_loc, get_location_for_seed(s, &a.maps));
        }
    }

    min_loc
}

fn part_one(input: &String) -> u64 {
    let almanac = parse_almanac(input);

    *derive_locations(&almanac).iter().min().unwrap_or(&0)
}

fn part_two(input: &String) -> u64 {
    let almanac = parse_almanac(input);

    derive_minimum_location(&almanac)
}

fn main() {
    let input = fs::read_to_string("../inputs/day05.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day05.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(35, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day05.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(46, part_two(&input))
}
