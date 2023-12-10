use std::{cmp::min, fs};

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn opposite(d: &Direction) -> &Direction {
    match d {
        Direction::North => &Direction::South,
        Direction::East => &Direction::West,
        Direction::South => &Direction::North,
        Direction::West => &Direction::East,
    }
}

static ALL_OFFSETS: [(i8, i8); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(PartialEq)]
enum Tile {
    Pipe(Direction, Direction),
    Start,
    Empty,
}

type TileMap = Vec<Vec<Tile>>;

fn get_tiles(input: &str) -> (TileMap, (usize, usize)) {
    let mut tiles: TileMap = Vec::new();
    let mut start: (usize, usize) = (0, 0);

    for (i, line) in input.lines().enumerate() {
        let mut tile_line: Vec<Tile> = Vec::new();

        for (j, tile_char) in line.chars().enumerate() {
            tile_line.push(match tile_char {
                '|' => Tile::Pipe(Direction::North, Direction::South),
                '-' => Tile::Pipe(Direction::East, Direction::West),
                'L' => Tile::Pipe(Direction::North, Direction::East),
                'J' => Tile::Pipe(Direction::North, Direction::West),
                '7' => Tile::Pipe(Direction::South, Direction::West),
                'F' => Tile::Pipe(Direction::South, Direction::East),
                '.' => Tile::Empty,
                'S' => {
                    start = (i, j);
                    Tile::Start
                }
                _ => panic!("Unrecognised tile char: {}", tile_char),
            })
        }

        tiles.push(tile_line)
    }

    assert_ne!(start, (0, 0));
    (tiles, start)
}

fn _print_tiles(t: &TileMap, name: &str) {
    let mut tiles = String::from("~~~~~~~~~~\n");

    for i in 0..t.len() {
        for j in 0..t[i].len() {
            tiles.push(match t[i][j] {
                Tile::Pipe(Direction::North, Direction::South) => '|',
                Tile::Pipe(Direction::East, Direction::West) => '-',
                Tile::Pipe(Direction::North, Direction::East) => 'L',
                Tile::Pipe(Direction::North, Direction::West) => 'J',
                Tile::Pipe(Direction::South, Direction::West) => '7',
                Tile::Pipe(Direction::South, Direction::East) => 'F',
                Tile::Empty => '.',
                Tile::Start => 'S',
                _ => panic!("Unrecognised tile at ({},{})", i, j),
            })
        }

        tiles.push_str("\n")
    }
    tiles.push_str("~~~~~~~~~~\n");

    let mut path = String::from("../outputs/");
    path.push_str(name);

    let _ = fs::write(path, tiles).expect("Should have been able to write the output file");
}

/**
 * Applies the given offset to the from position or None if out of bounds
 * Assumes a perfectly square bound
 */
fn apply_offset(from: (usize, usize), by: (i8, i8), limit: usize) -> Option<(usize, usize)> {
    let new_i = (from.0 as i32) + (by.0 as i32);
    let new_j = (from.1 as i32) + (by.1 as i32);

    if new_i >= 0 && new_i < limit as i32 && new_j >= 0 && new_j < limit as i32 {
        Some((new_i as usize, new_j as usize))
    } else {
        None
    }
}

fn get_direction_to_neighbour(
    head_pos: (usize, usize),
    neighbour_pos: (usize, usize),
) -> Direction {
    match (head_pos, neighbour_pos) {
        ((h_i, h_j), (n_i, n_j)) if h_i == n_i => {
            if n_j > h_j {
                Direction::East
            } else {
                Direction::West
            }
        }
        ((h_i, h_j), (n_i, n_j)) if h_j == n_j => {
            if n_i < h_i {
                Direction::North
            } else {
                Direction::South
            }
        }
        _ => panic!(
            "Unhandled relative positions: {:?} -> {:?}",
            head_pos, neighbour_pos
        ),
    }
}

fn tiles_connect(
    head_tile: &Tile,
    neighbour_tile: &Tile,
    head_pos: (usize, usize),
    neighbour_pos: (usize, usize),
) -> bool {
    let neighbour_direction = get_direction_to_neighbour(head_pos, neighbour_pos);

    match (head_tile, neighbour_tile) {
        (&Tile::Start, &Tile::Pipe(ref n_0, ref n_1)) => {
            return neighbour_direction == *opposite(n_0) || neighbour_direction == *opposite(n_1);
        }
        (&Tile::Pipe(ref h_0, ref h_1), &Tile::Start) => {
            return neighbour_direction == *h_0 || neighbour_direction == *h_1;
        }
        (_, &Tile::Empty) => false,
        (&Tile::Pipe(ref h_0, ref h_1), &Tile::Pipe(ref n_0, ref n_1)) => {
            return (*h_0 == neighbour_direction || *h_1 == neighbour_direction)
                && (neighbour_direction == *opposite(n_0)
                    || neighbour_direction == *opposite(n_1));
        }
        _ => panic!("Unsupported tiles comparison"),
    }
}

fn get_complete_path(tiles: &TileMap, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut paths: Vec<(Vec<(usize, usize)>, Option<(i8, i8)>)> =
        Vec::from([(Vec::from([start]), None)]);
    let mut complete_path: Vec<(usize, usize)> = Vec::new();

    while !paths.is_empty() && complete_path.is_empty() {
        let (path, some_last_offset) = paths.pop().unwrap();
        let head_pos = path[path.len() - 1];
        let head_tile = &tiles[head_pos.0][head_pos.1];

        for offset in ALL_OFFSETS {
            // Don't go back the way we came
            if some_last_offset.is_some() {
                let last_offest = some_last_offset.unwrap();
                if offset == (last_offest.0 * -1, last_offest.1 * -1) {
                    continue;
                }
            }

            let some_neighbour_pos = apply_offset(head_pos, offset, tiles.len());

            if some_neighbour_pos.is_some() {
                let neighbour_pos = some_neighbour_pos.unwrap();
                let neighbour_tile = &tiles[neighbour_pos.0][neighbour_pos.1];

                if tiles_connect(head_tile, neighbour_tile, head_pos, neighbour_pos) {
                    if neighbour_tile == &Tile::Start {
                        complete_path = path.clone();
                        break;
                    }

                    let mut new_path = path.clone();
                    new_path.push(neighbour_pos);
                    paths.push((new_path, Some(offset)));
                }
            }
        }
    }

    assert!(!complete_path.is_empty());
    complete_path
}

/**
 * Walks in a set direction (West) to the edge of the map
 * keeping track of how many North and South terminuses are seen
 * if the number of complete pairs of N,S is odd, then enclosed
 */
fn is_enclosed(pos: (usize, usize), tiles: &TileMap) -> bool {
    let mut north_count = 0;
    let mut south_count = 0;

    for j in 0..pos.1 {
        match &tiles[pos.0][j] {
            Tile::Pipe(d_0, d_1) => {
                match d_0 {
                    &Direction::North => north_count += 1,
                    &Direction::South => south_count += 1,
                    _ => (),
                }

                match d_1 {
                    &Direction::North => north_count += 1,
                    &Direction::South => south_count += 1,
                    _ => (),
                }
            }
            _ => (),
        }
    }

    min(north_count, south_count) % 2 == 1
}

fn part_one(input: &String) -> u32 {
    let (tiles, start) = get_tiles(input);
    let complete_path = get_complete_path(&tiles, start);
    complete_path.len() as u32 / 2
}

fn part_two(input: &String) -> u32 {
    let (mut tiles, start) = get_tiles(input);
    let complete_path = get_complete_path(&tiles, start);

    // Clean up the unused pipe pieces
    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            if !complete_path.contains(&(i, j)) {
                tiles[i][j] = Tile::Empty
            }
        }
    }

    let mut enclosed_count = 0;

    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            if !complete_path.contains(&(i, j)) {
                if is_enclosed((i, j), &tiles) {
                    enclosed_count += 1;
                }
            }
        }
    }

    enclosed_count
}

fn main() {
    let input = fs::read_to_string("../inputs/day10.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one_one() {
    let input = fs::read_to_string("../inputs/test_day10_1.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(4, part_one(&input))
}

#[test]
fn test_part_one_two() {
    let input = fs::read_to_string("../inputs/test_day10_2.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(8, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day10_3.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(10, part_two(&input))
}
