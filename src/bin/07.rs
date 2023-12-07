use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Play {
    hand: String,
    points: u32,
}

struct PlayWithJoker {
    hand: String,
    points: u32,
}

fn get_char_point(c: char) -> u8 {
    match c {
        c if c.is_ascii_digit() => c as u8 - 48,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        _ => 14,
    }
}

fn get_char_point_with_joker(c: char) -> u8 {
    match c {
        c if c.is_ascii_digit() => c as u8 - 48,
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        _ => 14,
    }
}

fn get_char_map(hand: &str) -> HashMap<char, u32> {
    let mut letter_counts: HashMap<char, u32> = HashMap::new();
    hand.chars().for_each(|c| {
        *letter_counts.entry(c).or_insert(0) += 1;
    });
    letter_counts
}

fn get_hand_value(letter_counts: HashMap<char, u32>) -> u32 {
    match letter_counts.keys().len() {
        1 => 7,
        2 => {
            if letter_counts.values().any(|i| i == &4) {
                6
            } else {
                5
            }
        }
        3 => {
            if letter_counts.values().any(|i| i == &3) {
                4
            } else {
                3
            }
        }
        4 => 2,
        _ => 1,
    }
}

fn get_hand_value_with_joker(mut letter_counts: HashMap<char, u32>) -> u32 {
    let j = letter_counts.remove(&'J').unwrap_or(0);
    match j {
        0 => get_hand_value(letter_counts),
        1 => match &letter_counts.keys().len() {
            1 => 7,
            2 => {
                if letter_counts.values().any(|i| i == &4) {
                    7
                } else if letter_counts.values().any(|i| i == &3) {
                    6
                } else if letter_counts.values().any(|i| i == &2) {
                    if letter_counts.keys().len() == 2 {
                        5
                    } else {
                        4
                    }
                } else {
                    2
                }
            }
            3 => 4,
            _ => 2,
        },
        2 => match &letter_counts.keys().len() {
            1 => 7,
            2 => 6,
            _ => 4,
        },
        3 => match &letter_counts.keys().len() {
            1 => 7,
            _ => 6,
        },
        _ => 7,
    }
}

impl Play {
    fn compare_hands(&self, other: &Self) -> Ordering {
        for (self_char, other_char) in self.hand.chars().zip(other.hand.chars()) {
            let a = get_char_point(self_char);
            let b = get_char_point(other_char);
            if a != b {
                return a.cmp(&b);
            }
        }
        Ordering::Equal
    }
}

impl PlayWithJoker {
    fn compare_hands(&self, other: &Self) -> Ordering {
        for (self_char, other_char) in self.hand.chars().zip(other.hand.chars()) {
            let a = get_char_point_with_joker(self_char);
            let b = get_char_point_with_joker(other_char);
            if a != b {
                return a.cmp(&b);
            }
        }
        Ordering::Equal
    }
}

impl Eq for Play {}
impl Eq for PlayWithJoker {}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        let first_hand_value = get_hand_value(get_char_map(&self.hand));
        let sec_hand_value = get_hand_value(get_char_map(&other.hand));
        if first_hand_value != sec_hand_value {
            first_hand_value.cmp(&sec_hand_value)
        } else {
            self.compare_hands(other)
        }
    }
}

impl Ord for PlayWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        let first_hand_value = get_hand_value_with_joker(get_char_map(&self.hand));
        let sec_hand_value = get_hand_value_with_joker(get_char_map(&other.hand));
        if first_hand_value != sec_hand_value {
            (first_hand_value as u8).cmp(&(sec_hand_value as u8))
        } else {
            self.compare_hands(other)
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for PlayWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialEq for PlayWithJoker {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim_end().split('\n');
    let mut plays: Vec<Play> = lines
        .map(|line| {
            let mut a = line.split(' ');
            Play {
                hand: a.next().unwrap().to_owned(),
                points: a.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    plays.sort();
    let mut res = 0;
    for (x, play) in plays.iter().enumerate() {
        res += play.points * (x as u32 + 1);
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim_end().split('\n');
    let mut plays: Vec<PlayWithJoker> = lines
        .map(|line| {
            let mut a = line.split(' ');
            PlayWithJoker {
                hand: a.next().unwrap().to_owned(),
                points: a.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    plays.sort();
    let mut res = 0;
    for (x, play) in plays.iter().enumerate() {
        res += play.points * (x as u32 + 1);
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(6440));
        assert_eq!(result, Some(252295678));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(5905));
        assert_eq!(result, Some(250577259));
    }
}
