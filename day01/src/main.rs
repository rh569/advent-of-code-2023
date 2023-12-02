use std::fs;

fn part_one() -> u32 {
    let input = fs::read_to_string("../inputs/day01.txt")
        .expect("Should have been able to read the input file");
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut digits: Vec<u32> = Vec::new();

        for c in line.chars() {
            match c {
                '0'..='9' => digits.push(c.to_digit(10).unwrap()),
                _ => (),
            }
        }

        let mut point = digits[0].to_string();
        point.push_str(&digits[digits.len() - 1].to_string());

        sum += point.parse::<u32>().unwrap()
    }

    sum
}

fn part_two(input: String) -> u32 {
    let numeric_digits: [&str; 10] = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    let written_digits: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut digits: Vec<u32> = Vec::new();

        let mut slice_start: usize = 0;

        while slice_start < line.len() {
            let mut slice_end: usize = slice_start + 1;

            'decode: loop {
                let part = &line[slice_start..slice_end];

                if part.len() == 1 {
                    for n in numeric_digits {
                        if n == part {
                            digits.push(n.parse::<u32>().unwrap());
                            slice_start += 1;
                            break 'decode;
                        }
                    }
                } else {
                    for (i, n) in written_digits.iter().enumerate() {
                        if &part == n {
                            digits.push(u32::try_from(i).unwrap() + 1);
                            slice_start += 1;
                            break 'decode;
                        }
                    }
                }

                if slice_end < line.len() {
                    slice_end += 1;
                } else {
                    // done
                    slice_start += 1;
                    break 'decode;
                }
            }
        }

        let mut point = digits[0].to_string();
        point.push_str(&digits[digits.len() - 1].to_string());

        sum += point.parse::<u32>().unwrap()
    }

    sum
}

fn main() {
    println!("{:?}", part_one());
    println!(
        "{:?}",
        part_two(
            fs::read_to_string("../inputs/day01.txt")
                .expect("Should have been able to read the input file"),
        )
    )
}

#[test]
fn test_part_two() {
    let s = String::from(
        "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
eightbcmnpnmq7hcqd1kkbjmtnlcjsixoneightvrh",
    );
    assert_eq!(281 + 88, part_two(s));
}
