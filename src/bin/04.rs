use std::{cmp::min, collections::HashSet};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim_end().split('\n');
    Some(
        lines
            .map(|line| {
                let mut split = line.split(':').nth(1).unwrap().split('|');
                let winning_numbers: HashSet<&str> = HashSet::from_iter(
                    split
                        .next()
                        .unwrap()
                        .trim()
                        .split(' ')
                        .filter(|e| !e.is_empty())
                        .collect::<Vec<&str>>(),
                );
                let other_numbers: HashSet<&str> = HashSet::from_iter(
                    split
                        .next()
                        .unwrap()
                        .trim()
                        .split(' ')
                        .filter(|e| !e.is_empty())
                        .collect::<Vec<&str>>(),
                );
                compute_points(winning_numbers.intersection(&other_numbers).count() as u32)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim_end().split('\n');
    let wins: Vec<u32> = lines
        .map(|line| {
            let mut split = line.split(':').nth(1).unwrap().split('|');
            let winning_numbers: HashSet<&str> = HashSet::from_iter(
                split
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|e| !e.is_empty())
                    .collect::<Vec<&str>>(),
            );
            let other_numbers: HashSet<&str> = HashSet::from_iter(
                split
                    .next()
                    .unwrap()
                    .trim()
                    .split(' ')
                    .filter(|e| !e.is_empty())
                    .collect::<Vec<&str>>(),
            );
            winning_numbers.intersection(&other_numbers).count() as u32
        })
        .collect();
    let mut scratchpad_wins = vec![0; wins.len()];
    for x in 0..wins.len() {
        let current = wins[x];
        let next = scratchpad_wins[x] + 1;
        scratchpad_wins[x] = next;
        let min = min(wins.len(), x + current as usize + 1);
        for y in scratchpad_wins.iter_mut().take(min).skip(x + 1) {
            *y += next;
        }
    }
    Some(scratchpad_wins.iter().sum())
}

fn compute_points(size: u32) -> u32 {
    if size == 0 {
        0
    } else {
        u32::pow(2, size - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(13));
        assert_eq!(result, Some(25010));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(30));
        assert_eq!(result, Some(9924412));
    }
}
