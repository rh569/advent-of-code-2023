use std::cmp;
use std::fs;

// (r, g, b)
#[derive(Debug)]
struct Grab(u32, u32, u32);

struct Game {
    id: u32,
    grabs: Vec<Grab>,
}

fn gvec_to_grab(gvec: Vec<Vec<&str>>) -> Grab {
    let mut grab = Grab(0, 0, 0);

    for c in gvec {
        match c[1] {
            "red" => grab.0 = c[0].parse::<u32>().unwrap(),
            "green" => grab.1 = c[0].parse::<u32>().unwrap(),
            "blue" => grab.2 = c[0].parse::<u32>().unwrap(),
            _ => panic!(),
        }
    }

    grab
}

fn get_games(s: &String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for l in s.lines() {
        let colon_idx = l.find(':').unwrap();
        let id = l[5..colon_idx].parse::<u32>().unwrap();

        let grabs: Vec<Grab> = l[colon_idx + 2..]
            .split("; ")
            .map(|x| {
                gvec_to_grab(
                    x.split(", ")
                        .map(|y| y.split(" ").collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>(),
                )
            })
            .collect();

        games.push(Game { id, grabs })
    }

    games
}

fn is_allowed(gs: &Vec<Grab>, r_max: u32, g_max: u32, b_max: u32) -> bool {
    let mut allowed = true;

    for grab in gs {
        if grab.0 > r_max || grab.1 > g_max || grab.2 > b_max {
            allowed = false;
        }
    }

    allowed
}

fn get_cube_minimums(gs: &Vec<Grab>) -> Grab {
    let mut max_cubes = Grab(0, 0, 0);

    for grab in gs {
        max_cubes.0 = cmp::max(max_cubes.0, grab.0);
        max_cubes.1 = cmp::max(max_cubes.1, grab.1);
        max_cubes.2 = cmp::max(max_cubes.2, grab.2);
    }

    max_cubes
}

fn part_one(input: &String) -> u32 {
    let games = get_games(input);
    const REDS: u32 = 12;
    const GREENS: u32 = 13;
    const BLUES: u32 = 14;

    games
        .iter()
        .filter(|g| is_allowed(&g.grabs, REDS, GREENS, BLUES))
        .map(|g| g.id)
        .sum()
}

fn part_two(input: &String) -> u32 {
    let games = get_games(input);

    games
        .iter()
        .map(|g: &Game| get_cube_minimums(&g.grabs))
        .map(|gr| gr.0 * gr.1 * gr.2)
        .sum()
}

fn main() {
    let input = fs::read_to_string("../inputs/day02.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day02.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(8, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day02.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(2286, part_two(&input))
}
