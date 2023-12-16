use std::{cmp, collections::HashSet};

advent_of_code::solution!(16);

#[derive(Debug, Eq, PartialEq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Movement {
    row: usize,
    col: usize,
    entry_direction: Direction,
}

pub fn part_one(input: &str) -> Option<usize> {
    let beam_map: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|e| e.chars().collect())
        .collect();
    Some(compute_distance(
        &beam_map,
        Movement {
            row: 0,
            col: 0,
            entry_direction: Direction::West,
        },
    ))
}

fn compute_distance(beam_map: &[Vec<char>], start_at: Movement) -> usize {
    let height = beam_map.len();
    let width = beam_map[0].len();
    let mut visited: HashSet<Movement> = HashSet::new();
    let mut to_visit = vec![start_at];
    while let Some(movement) = to_visit.pop() {
        match beam_map[movement.row][movement.col] {
            '|' => match movement.entry_direction {
                Direction::North => {
                    if movement.row < height - 1 {
                        let next = Movement {
                            row: movement.row + 1,
                            col: movement.col,
                            entry_direction: Direction::North,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::South => {
                    if movement.row > 0 {
                        let next = Movement {
                            row: movement.row - 1,
                            col: movement.col,
                            entry_direction: Direction::South,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                _ => {
                    if movement.row < height - 1 {
                        let next = Movement {
                            row: movement.row + 1,
                            col: movement.col,
                            entry_direction: Direction::North,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                    if movement.row > 0 {
                        let next = Movement {
                            row: movement.row - 1,
                            col: movement.col,
                            entry_direction: Direction::South,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
            },
            '-' => match movement.entry_direction {
                Direction::East => {
                    if movement.col > 0 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col - 1,
                            entry_direction: Direction::East,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::West => {
                    if movement.col < width - 1 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col + 1,
                            entry_direction: Direction::West,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                _ => {
                    if movement.col > 0 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col - 1,
                            entry_direction: Direction::East,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                    if movement.col < width - 1 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col + 1,
                            entry_direction: Direction::West,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
            },
            '/' => match movement.entry_direction {
                Direction::North => {
                    if movement.col > 0 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col - 1,
                            entry_direction: Direction::East,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::South => {
                    if movement.col < width - 1 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col + 1,
                            entry_direction: Direction::West,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::East => {
                    if movement.row < height - 1 {
                        let next = Movement {
                            row: movement.row + 1,
                            col: movement.col,
                            entry_direction: Direction::North,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::West => {
                    if movement.row > 0 {
                        let next = Movement {
                            row: movement.row - 1,
                            col: movement.col,
                            entry_direction: Direction::South,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
            },
            '\\' => match movement.entry_direction {
                Direction::North => {
                    if movement.col < width - 1 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col + 1,
                            entry_direction: Direction::West,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::South => {
                    if movement.col > 0 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col - 1,
                            entry_direction: Direction::East,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::East => {
                    if movement.row > 0 {
                        let next = Movement {
                            row: movement.row - 1,
                            col: movement.col,
                            entry_direction: Direction::South,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::West => {
                    if movement.row < height - 1 {
                        let next = Movement {
                            row: movement.row + 1,
                            col: movement.col,
                            entry_direction: Direction::North,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
            },
            _ => match movement.entry_direction {
                Direction::North => {
                    if movement.row < height - 1 {
                        let next = Movement {
                            row: movement.row + 1,
                            col: movement.col,
                            entry_direction: Direction::North,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::South => {
                    if movement.row > 0 {
                        let next = Movement {
                            row: movement.row - 1,
                            col: movement.col,
                            entry_direction: Direction::South,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::East => {
                    if movement.col > 0 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col - 1,
                            entry_direction: Direction::East,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
                Direction::West => {
                    if movement.col < width - 1 {
                        let next = Movement {
                            row: movement.row,
                            col: movement.col + 1,
                            entry_direction: Direction::West,
                        };
                        if !visited.contains(&next) {
                            to_visit.push(next);
                        }
                    }
                }
            },
        }
        visited.insert(movement);
    }
    let mut all_coords: HashSet<(usize, usize)> = HashSet::new();
    for pos in visited {
        all_coords.insert((pos.row, pos.col));
    }
    all_coords.len()
}
pub fn part_two(input: &str) -> Option<usize> {
    let beam_map: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|e| e.chars().collect())
        .collect();
    let mut entry_points: Vec<Movement> = vec![];
    let height = beam_map.len();
    let width = beam_map[0].len();
    for row in 0..height {
        entry_points.push(Movement {
            row,
            col: 0,
            entry_direction: Direction::West,
        });

        entry_points.push(Movement {
            row,
            col: width - 1,
            entry_direction: Direction::East,
        });
    }

    for col in 0..width {
        entry_points.push(Movement {
            row: 0,
            col,
            entry_direction: Direction::North,
        });

        entry_points.push(Movement {
            row: height - 1,
            col,
            entry_direction: Direction::South,
        });
    }

    let mut max = 0;
    while let Some(entry_point) = entry_points.pop() {
        max = cmp::max(compute_distance(&beam_map, entry_point), max);
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(46));
        assert_eq!(result, Some(6902));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(51));
        assert_eq!(result, Some(7697));
    }
}
