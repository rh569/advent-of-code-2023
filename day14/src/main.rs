use std::{collections::HashMap, fs};

fn get_platform(s: &String) -> Vec<Vec<char>> {
    let mut platform: Vec<Vec<char>> = Vec::new();

    for l in s.lines() {
        platform.push(l.chars().collect())
    }

    platform
}

fn roll_north(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 1..platform.len() {
        for j in 0..platform[i].len() {
            if platform[i][j] == 'O' {
                let mut north_most_i = i;

                while north_most_i > 0 {
                    if platform[north_most_i - 1][j] == '.' {
                        north_most_i -= 1;
                    } else {
                        break;
                    }
                }

                if north_most_i < i && platform[north_most_i][j] == '.' {
                    platform[north_most_i][j] = 'O';
                    platform[i][j] = '.';
                }
            }
        }
    }

    platform
}

fn roll_south(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in (0..platform.len() - 1).rev() {
        for j in 0..platform[i].len() {
            if platform[i][j] == 'O' {
                let mut south_most_i: usize = i;

                while south_most_i < platform.len() - 1 {
                    if platform[south_most_i + 1][j] == '.' {
                        south_most_i += 1;
                    } else {
                        break;
                    }
                }

                if south_most_i > i && platform[south_most_i][j] == '.' {
                    platform[south_most_i][j] = 'O';
                    platform[i][j] = '.';
                }
            }
        }
    }

    platform
}

fn roll_west(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..platform.len() {
        for j in 1..platform[i].len() {
            if platform[i][j] == 'O' {
                let mut west_most_j: usize = j;

                while west_most_j > 0 {
                    if platform[i][west_most_j - 1] == '.' {
                        west_most_j -= 1;
                    } else {
                        break;
                    }
                }

                if west_most_j < j && platform[i][west_most_j] == '.' {
                    platform[i][west_most_j] = 'O';
                    platform[i][j] = '.';
                }
            }
        }
    }

    platform
}

fn roll_east(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..platform.len() {
        for j in (0..platform[i].len() - 1).rev() {
            if platform[i][j] == 'O' {
                let mut east_most_j: usize = j;

                while east_most_j < platform[i].len() - 1 {
                    if platform[i][east_most_j + 1] == '.' {
                        east_most_j += 1;
                    } else {
                        break;
                    }
                }

                if east_most_j > j && platform[i][east_most_j] == '.' {
                    platform[i][east_most_j] = 'O';
                    platform[i][j] = '.';
                }
            }
        }
    }

    platform
}

fn get_north_load(platform: Vec<Vec<char>>) -> u32 {
    let mut load: u32 = 0;

    for i in 0..platform.len() {
        for j in 0..platform[i].len() {
            if platform[i][j] == 'O' {
                load += (platform.len() - i) as u32
            }
        }
    }

    load
}

fn to_string(p: &Vec<Vec<char>>) -> String {
    let mut plat_str = String::from("");

    for i in 0..p.len() {
        for j in 0..p[i].len() {
            plat_str.push(p[i][j])
        }
    }

    plat_str
}

fn part_one(input: &String) -> u32 {
    let mut platform = get_platform(input);
    platform = roll_north(platform);
    get_north_load(platform)
}

fn part_two(input: &String) -> u32 {
    let mut platform = get_platform(input);
    let mut n_by_state_map: HashMap<String, i32> = HashMap::new();

    for n in 0..1000000000 {
        let plat_str = to_string(&platform);

        if n_by_state_map.contains_key(&plat_str) {
            let loop_length = n - n_by_state_map.get(&plat_str).unwrap();
            let remaining_cycles = (1000000000 - n) % loop_length;

            for _ in 0..remaining_cycles {
                platform = do_spin_cycle(platform);
            }

            return get_north_load(platform);
        } else {
            n_by_state_map.insert(plat_str, n);
        }

        platform = do_spin_cycle(platform);
    }

    get_north_load(platform)
}

fn do_spin_cycle(mut platform: Vec<Vec<char>>) -> Vec<Vec<char>> {
    platform = roll_north(platform);
    platform = roll_west(platform);
    platform = roll_south(platform);
    roll_east(platform)
}

fn main() {
    let input = fs::read_to_string("../inputs/day14.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day14.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(136, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day14.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(64, part_two(&input))
}
