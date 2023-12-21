use std::{cmp::min, collections::HashMap, fs};

type RockMap = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct NoReflectionError;

fn get_rock_maps(s: &String) -> Vec<RockMap> {
    s.split("\n\n")
        .map(|block| {
            let mut map: RockMap = Vec::new();

            for l in block.lines() {
                let row = l.chars().collect::<Vec<char>>();
                map.push(row);
            }

            map
        })
        .collect::<Vec<RockMap>>()
}

fn row_eq(a: &Vec<char>, b: &Vec<char>) -> bool {
    debug_assert_eq!(a.len(), b.len());

    for j in 0..a.len() {
        if a[j] != b[j] {
            return false;
        }
    }

    true
}

fn col_eq(m: &RockMap, j_a: usize, j_b: usize) -> bool {
    debug_assert!(j_a != j_b);
    debug_assert!(j_a < m[0].len());
    debug_assert!(j_b < m[0].len());

    for i in 0..m.len() {
        if m[i][j_a] != m[i][j_b] {
            return false;
        }
    }

    true
}

fn get_vertical_reflections(rock_map: &RockMap) -> Vec<usize> {
    let mut reflections: Vec<usize> = Vec::new();

    for check_j in 1..rock_map[0].len() {
        let mut is_mirror = true;
        let reflection_size = min(check_j, rock_map[0].len() - check_j);

        for offset in 1..=reflection_size {
            if !col_eq(rock_map, check_j - offset, check_j + offset - 1) {
                is_mirror = false;
                break;
            }
        }

        if is_mirror {
            reflections.push(check_j)
        }
    }

    reflections
}

fn get_horizontal_reflections(rock_map: &RockMap) -> Vec<usize> {
    let mut reflections: Vec<usize> = Vec::new();
    for check_i in 1..rock_map.len() {
        let mut is_mirror = true;
        let reflection_size = min(check_i, rock_map.len() - check_i);

        for offset in 1..=reflection_size {
            if !row_eq(&rock_map[check_i - offset], &rock_map[check_i + offset - 1]) {
                is_mirror = false;
                break;
            }
        }

        if is_mirror {
            reflections.push(check_i)
        }
    }
    reflections
}

/**
 * Finds the col or row index where a line of reflection occurs
 * Returns the index for cols, or 100 times the index for rows
 *
 * Treats vertical reflection lines with higher precedence
 */
fn find_either_reflection(rock_map: &RockMap) -> Result<u32, NoReflectionError> {
    let vert_refs = get_vertical_reflections(rock_map);

    if vert_refs.len() == 1 {
        return Ok(vert_refs[0] as u32);
    }

    let horz_refs = get_horizontal_reflections(rock_map);

    if horz_refs.len() == 1 {
        return Ok((100 * horz_refs[0]) as u32);
    }

    Err(NoReflectionError)
}

fn find_all_reflections(rock_map: &RockMap) -> (Vec<u32>, Vec<u32>) {
    let vert_refs = get_vertical_reflections(rock_map)
        .iter()
        .map(|r| *r as u32)
        .collect();
    let horz_refs = get_horizontal_reflections(rock_map)
        .iter()
        .map(|r| *r as u32)
        .map(|r| r * 100)
        .collect();

    (vert_refs, horz_refs)
}

fn flip(c: char) -> char {
    match c {
        '.' => '#',
        '#' => '.',
        _ => panic!(),
    }
}

fn get_smudge_possibilities(source_map: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut possible_maps: Vec<Vec<Vec<char>>> = Vec::new();

    for i in 0..source_map.len() {
        for j in 0..source_map[i].len() {
            let mut new_map = source_map.clone();
            new_map[i][j] = flip(new_map[i][j]);
            possible_maps.push(new_map);
        }
    }

    possible_maps
}

fn part_one(input: &String) -> u32 {
    let rock_maps: Vec<Vec<Vec<char>>> = get_rock_maps(input);
    rock_maps
        .iter()
        .map(|m| find_either_reflection(m).unwrap())
        .sum()
}

fn part_two(input: &String) -> u32 {
    let rock_maps: Vec<Vec<Vec<char>>> = get_rock_maps(input);
    let mut sum: u32 = 0;

    for i in 0..rock_maps.len() {
        let possible_maps: Vec<Vec<Vec<char>>> = get_smudge_possibilities(&rock_maps[i]);
        let mut reflections: HashMap<u32, u32> = HashMap::new();

        for m in possible_maps {
            let (vert_refs, horz_refs) = find_all_reflections(&m);

            for v in vert_refs.iter() {
                if reflections.contains_key(v) {
                    reflections.insert(*v, reflections.get(v).unwrap() + 1);
                } else {
                    reflections.insert(*v, 1);
                }
            }

            for h in horz_refs.iter() {
                if reflections.contains_key(h) {
                    reflections.insert(*h, reflections.get(h).unwrap() + 1);
                } else {
                    reflections.insert(*h, 1);
                }
            }
        }

        sum += get_new(reflections)
    }

    sum
}

fn get_new(reflections: HashMap<u32, u32>) -> u32 {
    for (k, v) in reflections.iter() {
        if *v == 2 {
            // expect 2 entries because an opposite smudge would also cause this line of reflection
            return *k;
        }
    }

    panic!("No new lines of reflection found")
}

fn main() {
    let input = fs::read_to_string("../inputs/day13.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day13.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(405, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day13.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(400, part_two(&input))
}
