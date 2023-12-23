use std::fs;

struct HashmapStep {
    label: Vec<char>,
    op: char,
    focal_length: u32,
}

struct Lens {
    label: Vec<char>,
    focal_length: u32,
}

impl HashmapStep {
    fn from_str(s: &str) -> HashmapStep {
        let mut label: Vec<char> = Vec::new();
        let mut op: char = '*';
        let mut focal_length = 0;

        for c in s.chars() {
            match c {
                'a'..='z' => label.push(c),
                '1'..='9' => focal_length = c.to_digit(10).expect("should be digit"),
                '-' | '=' => op = c,
                _ => panic!("unexpected char {}", c),
            }
        }

        HashmapStep {
            label,
            op,
            focal_length,
        }
    }
}

fn get_steps(s: &String) -> Vec<String> {
    s.split(',')
        .map(|st| String::from(st))
        .collect::<Vec<String>>()
}

fn h_a_s_h(chars: Vec<char>) -> u32 {
    let mut val = 0;

    for c in chars.iter() {
        val += *c as u32;
        val *= 17;
        val = val % 256;
    }

    val
}

fn initialize_lpf(mut lens_boxes: Vec<Vec<Lens>>, steps: Vec<HashmapStep>) -> Vec<Vec<Lens>> {
    for step in steps.iter() {
        let box_idx = h_a_s_h(step.label.clone()) as usize;

        if step.op == '-' {
            for l_i in 0..lens_boxes[box_idx].len() {
                if step.label == lens_boxes[box_idx][l_i].label {
                    lens_boxes[box_idx].remove(l_i);
                    break;
                }
            }
        }

        if step.op == '=' {
            let mut found_lens = false;

            for l_i in 0..lens_boxes[box_idx].len() {
                if step.label == lens_boxes[box_idx][l_i].label {
                    // replace
                    found_lens = true;
                    lens_boxes[box_idx][l_i] = Lens {
                        label: step.label.clone(),
                        focal_length: step.focal_length,
                    };
                    break;
                }
            }

            if !found_lens {
                // insert
                lens_boxes[box_idx].push(Lens {
                    label: step.label.clone(),
                    focal_length: step.focal_length,
                });
            }
        }
    }

    lens_boxes
}

fn get_focusing_power(lens_boxes: Vec<Vec<Lens>>) -> u32 {
    lens_boxes
        .iter()
        .enumerate()
        .map(|(box_idx, _box)| {
            _box.iter().enumerate().fold(0, |acc, (l_i, l)| {
                acc + (box_idx + 1) as u32 * (l_i + 1) as u32 * l.focal_length
            })
        })
        .sum()
}

fn part_one(input: &String) -> u32 {
    let steps = get_steps(input);

    steps
        .iter()
        .map(|s| h_a_s_h(s.chars().collect::<Vec<char>>()))
        .sum()
}

fn part_two(input: &String) -> u32 {
    let steps = get_steps(input);
    let hashmap_steps = steps
        .iter()
        .map(|s| HashmapStep::from_str(s))
        .collect::<Vec<HashmapStep>>();

    let mut lens_boxes: Vec<Vec<Lens>> = Vec::new();
    for _ in 0..256 {
        lens_boxes.push(Vec::new())
    }

    lens_boxes = initialize_lpf(lens_boxes, hashmap_steps);
    get_focusing_power(lens_boxes)
}

fn main() {
    let input = fs::read_to_string("../inputs/day15.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day15.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(1320, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day15.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(145, part_two(&input))
}

#[test]
fn test_h_a_s_h() {
    assert_eq!(52, h_a_s_h(Vec::from(['H', 'A', 'S', 'H'])))
}

#[test]
fn test_h_a_s_h_label() {
    assert_eq!(0, h_a_s_h(Vec::from(['r', 'n'])))
}
