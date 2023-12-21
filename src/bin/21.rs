use std::{
    collections::{HashSet, VecDeque},
    hash::{Hash, Hasher},
};

advent_of_code::solution!(21);

#[derive(Debug, PartialEq, Eq)]
struct Position {
    row: usize,
    col: usize,
    step: u32,
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.step.hash(state);
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();
    let starting_pos = get_start_pos(&map);
    let mut to_visit: VecDeque<Position> = VecDeque::new();
    to_visit.push_back(starting_pos);
    let mut visited: HashSet<Position> = HashSet::new();
    let max_steps = 6;

    while let Some(pos) = to_visit.pop_front() {
        if pos.step == max_steps {
            return Some(visited.len());
        }
        visited.insert(pos);
    }

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn get_start_pos(map: &Vec<Vec<char>>) -> Position {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'S' {
                return Position { row, col, step: 0 };
            }
        }
    }
    Position {
        row: 0,
        col: 0,
        step: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
