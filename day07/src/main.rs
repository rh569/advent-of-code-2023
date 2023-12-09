use std::{cmp::Ordering, collections::HashMap, fs};

#[derive(PartialOrd, PartialEq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

impl HandType {
    fn from_cards(cards: &Vec<u8>, use_jokers: bool) -> HandType {
        let mut card_counts: HashMap<u8, u8> = HashMap::new();

        for c in cards.iter() {
            if card_counts.contains_key(c) {
                card_counts.insert(*c, card_counts.get(c).unwrap() + 1);
            } else {
                card_counts.insert(*c, 1);
            }
        }

        let mut sorted_counts = card_counts.values().map(|n: &u8| *n).collect::<Vec<u8>>();
        sorted_counts.sort_unstable_by(|a, b| b.cmp(a));

        // If using jokers, find how many there were and increase the highest other count, removing the joker count
        if use_jokers {
            let joker_count = *card_counts.get(&1).unwrap_or(&0);

            if joker_count > 0 {
                let joker_idx: usize = sorted_counts
                    .iter()
                    .position(|c: &u8| *c == joker_count)
                    .unwrap();

                // only make changes if not FiveOAK of jokers
                if joker_count < 5 {
                    // remove first incase jokers are most prevalent
                    sorted_counts.remove(joker_idx);
                    sorted_counts[0] += joker_count;
                }
            }
        }

        match sorted_counts.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if sorted_counts[0] == 3 {
                    assert_eq!(1, sorted_counts[1]);
                    HandType::ThreeOAK
                } else {
                    assert_eq!(2, sorted_counts[0]);
                    HandType::TwoPair
                }
            }
            2 => {
                if sorted_counts[0] == 4 {
                    HandType::FourOAK
                } else {
                    assert_eq!(3, sorted_counts[0]);
                    HandType::FullHouse
                }
            }
            1 => {
                assert_eq!(5, sorted_counts[0]);
                HandType::FiveOAK
            }
            _ => panic!("Unhandled counts of cards"),
        }
    }
}

struct Hand {
    bid: u32,
    cards: Vec<u8>,
    hand_type: HandType,
}

impl Hand {
    fn from_str(s: &str, use_jokers: bool) -> Hand {
        let [cards_str, bid_str] = s.split(' ').collect::<Vec<&str>>()[0..=1] else {
            panic!("Hand string not expected: {:?}", { s })
        };

        let cards = cards_str
            .chars()
            .map(|c| from_char(c, use_jokers))
            .collect();

        Hand {
            bid: bid_str.parse().unwrap(),
            hand_type: HandType::from_cards(&cards, use_jokers),
            cards,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type > other.hand_type {
            Some(Ordering::Greater)
        } else if self.hand_type < other.hand_type {
            Some(Ordering::Less)
        } else {
            for (i, c) in self.cards.iter().enumerate() {
                if *c == other.cards[i] {
                    continue;
                }

                return c.partial_cmp(&other.cards[i]);
            }

            Some(Ordering::Equal)
        }
    }
}

fn from_char(c: char, use_jokers: bool) -> u8 {
    match c {
        '2'..='9' => c.to_digit(10).unwrap().try_into().unwrap(),
        'T' => 10,
        'J' => {
            if use_jokers {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Not a valid card char, given: {}", c),
    }
}

fn get_hands(input: &str, use_jokers: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|l| Hand::from_str(l, use_jokers))
        .collect::<Vec<Hand>>()
}

fn sum_hand_values(mut hands: Vec<Hand>) -> u32 {
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum()
}

fn part_one(input: &String) -> u32 {
    let hands: Vec<Hand> = get_hands(input, false);
    sum_hand_values(hands)
}

fn part_two(input: &String) -> u32 {
    let hands: Vec<Hand> = get_hands(input, true);
    sum_hand_values(hands)
}

fn main() {
    let input = fs::read_to_string("../inputs/day07.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two(&input));
}

#[test]
fn test_part_one() {
    let input = fs::read_to_string("../inputs/test_day07.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(6440, part_one(&input))
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("../inputs/test_day07.txt")
        .expect("Should have been able to read the input file");

    assert_eq!(5905, part_two(&input))
}
