use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Clone)]
struct BeamFront(usize, usize, char);

fn get_contraption(s: &String) -> Vec<Vec<char>> {
    let mut contraption: Vec<Vec<char>> = Vec::new();

    for l in s.lines() {
        contraption.push(l.chars().collect::<Vec<char>>())
    }

    contraption
}

fn get_new_beam_fronts(beam_front: &BeamFront, tile: char, size: usize) -> Vec<BeamFront> {
    let mut new_beam_fronts: Vec<BeamFront> = Vec::new();

    let directions = match (tile, beam_front.2) {
        ('.', b) => Vec::from([b]),

        ('\\', '^') => Vec::from(['<']),
        ('\\', '>') => Vec::from(['v']),
        ('\\', 'v') => Vec::from(['>']),
        ('\\', '<') => Vec::from(['^']),

        ('/', '^') => Vec::from(['>']),
        ('/', '>') => Vec::from(['^']),
        ('/', 'v') => Vec::from(['<']),
        ('/', '<') => Vec::from(['v']),

        ('|', '^') => Vec::from(['^']),
        ('|', '>') => Vec::from(['^', 'v']),
        ('|', 'v') => Vec::from(['v']),
        ('|', '<') => Vec::from(['^', 'v']),

        ('-', '^') => Vec::from(['>', '<']),
        ('-', '>') => Vec::from(['>']),
        ('-', 'v') => Vec::from(['>', '<']),
        ('-', '<') => Vec::from(['<']),

        _ => panic!("unexpected interaction"),
    };

    for d in directions {
        match d {
            '^' => {
                if beam_front.0 > 0 {
                    new_beam_fronts.push(BeamFront(beam_front.0 - 1, beam_front.1, d));
                }
            }
            '>' => {
                if beam_front.1 < size - 1 {
                    new_beam_fronts.push(BeamFront(beam_front.0, beam_front.1 + 1, d));
                }
            }
            'v' => {
                if beam_front.0 < size - 1 {
                    new_beam_fronts.push(BeamFront(beam_front.0 + 1, beam_front.1, d));
                }
            }
            '<' => {
                if beam_front.1 > 0 {
                    new_beam_fronts.push(BeamFront(beam_front.0, beam_front.1 - 1, d));
                }
            }
            _ => panic!("unexpected direction"),
        }
    }

    new_beam_fronts
}

fn get_energised_count(beam_fronts_seen: HashSet<BeamFront>) -> u32 {
    let energised_tiles: HashSet<String> = beam_fronts_seen
        .iter()
        .map(|bf| bf.0.to_string() + "," + &bf.1.to_string())
        .collect::<HashSet<String>>();

    energised_tiles.len() as u32
}

fn get_energised_count_for_initial(
    contraption: &Vec<Vec<char>>,
    initial_beam_front: BeamFront,
) -> u32 {
    let mut beam_fronts: Vec<BeamFront> = Vec::new();
    let mut beam_fronts_seen: HashSet<BeamFront> = HashSet::new();

    beam_fronts_seen.insert(initial_beam_front.clone());
    beam_fronts.push(initial_beam_front.clone());

    while beam_fronts.len() > 0 {
        let current_beam_front: BeamFront = beam_fronts.pop().unwrap();
        let new_beam_fronts: Vec<BeamFront> = get_new_beam_fronts(
            &current_beam_front,
            contraption[current_beam_front.0][current_beam_front.1],
            contraption.len(),
        );

        for i in 0..new_beam_fronts.len() {
            if beam_fronts_seen.contains(&new_beam_fronts[i]) {
                // loop detected
                continue;
            }

            beam_fronts_seen.insert(new_beam_fronts[i].clone());
            beam_fronts.push(new_beam_fronts[i].clone());
        }
    }

    get_energised_count(beam_fronts_seen)
}

fn part_one(input: &String) -> u32 {
    let contraption = get_contraption(input);
    let initial_beam_front = BeamFront(0, 0, '>');

    get_energised_count_for_initial(&contraption, initial_beam_front)
}

fn part_two(input: &String) -> u32 {
    let contraption = get_contraption(input);
    let size = contraption.len();

    let directions = Vec::from(['^', '>', 'v', '<']);
    let initial_beam_fronts = directions
        .iter()
        .flat_map(|d| match d {
            '^' => (0..size)
                .map(|j| BeamFront(size - 1, j, *d))
                .collect::<Vec<BeamFront>>(),
            '>' => (0..size)
                .map(|i| BeamFront(i, 0, *d))
                .collect::<Vec<BeamFront>>(),
            'v' => (0..size)
                .map(|j| BeamFront(0, j, *d))
                .collect::<Vec<BeamFront>>(),
            '<' => (0..size)
                .map(|i| BeamFront(i, size - 1, *d))
                .collect::<Vec<BeamFront>>(),
            _ => panic!("unexpected direction"),
        })
        .collect::<Vec<BeamFront>>();

    initial_beam_fronts
        .iter()
        .map(|ibf| get_energised_count_for_initial(&contraption, ibf.clone()))
        .max()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("../inputs/day16.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day16.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(46, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day16.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(51, part_two(&input))
}
