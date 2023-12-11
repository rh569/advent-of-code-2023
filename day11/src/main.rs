use std::fs;

fn get_cosmic_properties(s: &String) -> (usize, Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let size = s.lines().count();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut empty_is: Vec<usize> = (0..size).collect();
    let mut empty_js: Vec<usize> = (0..size).collect();

    for (i, l) in s.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                empty_is[i] = 2 * size;
                empty_js[j] = 2 * size;
                galaxies.push((i, j));
            }
        }
    }

    (
        size,
        galaxies,
        empty_is
            .iter()
            .filter(|n| *n != &(size * 2))
            .map(|n| *n)
            .collect(),
        empty_js
            .iter()
            .filter(|n| *n != &(size * 2))
            .map(|n| *n)
            .collect(),
    )
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> u64 {
    (b.0 as u64).abs_diff(a.0 as u64) + (b.1 as u64).abs_diff(a.1 as u64)
}

fn expand(
    mut galaxies: Vec<(usize, usize)>,
    empty_is: Vec<usize>,
    empty_js: Vec<usize>,
    expansion_factor: usize,
) -> Vec<(usize, usize)> {
    for g_idx in 0..galaxies.len() {
        let mut g = galaxies[g_idx];
        let mut i_count: usize = 0;
        let mut j_count: usize = 0;

        for i in empty_is.as_slice() {
            if i < &g.0 {
                i_count += 1;
            }
        }

        for j in empty_js.as_slice() {
            if j < &g.1 {
                j_count += 1;
            }
        }

        g.0 = g.0 - i_count + expansion_factor * i_count;
        g.1 = (g.1 - j_count) + expansion_factor * j_count;
        galaxies[g_idx] = g;
    }

    galaxies
}

fn get_distance_sum(galaxies: Vec<(usize, usize)>) -> u64 {
    let mut distance_sum = 0;

    for idx_a in 0..galaxies.len() {
        for idx_b in idx_a + 1..galaxies.len() {
            distance_sum += manhattan_distance(galaxies[idx_a], galaxies[idx_b])
        }
    }

    distance_sum
}

fn part_x(input: &String, expansion_factor: usize) -> u64 {
    let (_size, galaxies, empty_is, empty_js) = get_cosmic_properties(input);
    let expanded_galaxies = expand(galaxies, empty_is, empty_js, expansion_factor);
    get_distance_sum(expanded_galaxies)
}

fn main() {
    let input = fs::read_to_string("../inputs/day11.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_x(&input, 2));
    println!("Part 2: {}", part_x(&input, 1_000_000));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day11.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(374, part_x(&input, 2))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day11.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(1030, part_x(&input, 10))
}

#[test]
fn test_part_two_two() {
    let input = fs::read_to_string("../inputs/test_day11.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(8410, part_x(&input, 100))
}
