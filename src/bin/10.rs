use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

advent_of_code::solution!(10);

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
    dist: u32, // TODO Take this out
    symbol: char,
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

impl Eq for Position {}

impl Position {
    fn should_add_north(&self, matrix: &[Vec<char>]) -> Option<Position> {
        // Limits
        if self.row == 0 {
            return None;
        }
        // Check current allows to go up
        if self.symbol != '|' && self.symbol != 'L' && self.symbol != 'J' && self.symbol != 'S' {
            return None;
        }
        let north = matrix[self.row - 1][self.col];
        if north == '|' || north == '7' || north == 'F' {
            return Some(Position {
                row: self.row - 1,
                col: self.col,
                dist: self.dist + 1,
                symbol: north,
            });
        }
        None
    }

    fn should_add_south(&self, matrix: &[Vec<char>]) -> Option<Position> {
        // Limits
        if self.row == matrix.len() - 1 {
            return None;
        }
        // Check current allows to go down
        if self.symbol != '|' && self.symbol != '7' && self.symbol != 'F' && self.symbol != 'S' {
            return None;
        }
        let south = matrix[self.row + 1][self.col];
        if south == '|' || south == 'L' || south == 'J' {
            return Some(Position {
                row: self.row + 1,
                col: self.col,
                dist: self.dist + 1,
                symbol: south,
            });
        }
        None
    }

    fn should_add_west(&self, matrix: &[Vec<char>]) -> Option<Position> {
        // Limits
        if self.col == 0 {
            return None;
        }
        // Check current allows to go west
        if self.symbol != '-' && self.symbol != 'J' && self.symbol != '7' && self.symbol != 'S' {
            return None;
        }
        let west = matrix[self.row][self.col - 1];
        if west == '-' || west == 'L' || west == 'F' {
            return Some(Position {
                row: self.row,
                col: self.col - 1,
                dist: self.dist + 1,
                symbol: west,
            });
        }
        None
    }

    fn should_add_east(&self, matrix: &[Vec<char>]) -> Option<Position> {
        // Limits
        if self.col == matrix[self.row].len() - 1 {
            return None;
        }
        // Check current allows to go east
        if self.symbol != '-' && self.symbol != 'F' && self.symbol != 'L' && self.symbol != 'S' {
            return None;
        }
        let east = matrix[self.row][self.col + 1];
        if east == '-' || east == 'J' || east == '7' {
            return Some(Position {
                row: self.row,
                col: self.col + 1,
                dist: self.dist + 1,
                symbol: east,
            });
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut position_to_visit: VecDeque<Position> = VecDeque::new();
    position_to_visit.push_back(find_start(&matrix)?);
    let mut visited_positions: HashSet<Position> = HashSet::new();
    while !position_to_visit.is_empty() {
        let current_pos = position_to_visit.pop_front().unwrap();
        if let Some(pos) = current_pos.should_add_north(&matrix) {
            if !visited_positions.contains(&pos) && !position_to_visit.contains(&pos) {
                position_to_visit.push_back(pos);
            }
        }
        if let Some(pos) = current_pos.should_add_south(&matrix) {
            if !visited_positions.contains(&pos) && !position_to_visit.contains(&pos) {
                position_to_visit.push_back(pos);
            }
        }
        if let Some(pos) = current_pos.should_add_west(&matrix) {
            if !visited_positions.contains(&pos) && !position_to_visit.contains(&pos) {
                position_to_visit.push_back(pos);
            }
        }
        if let Some(pos) = current_pos.should_add_east(&matrix) {
            if !visited_positions.contains(&pos) && !position_to_visit.contains(&pos) {
                position_to_visit.push_back(pos);
            }
        }
        visited_positions.insert(current_pos);
    }
    let max_pos = visited_positions.iter().max_by_key(|p| p.dist).unwrap();
    Some(max_pos.dist)
}

fn find_start(matrix: &[Vec<char>]) -> Option<Position> {
    for (row, line) in matrix.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == 'S' {
                return Some(Position {
                    row,
                    col,
                    dist: 0,
                    symbol: 'S',
                });
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut position_to_visit: VecDeque<Position> = VecDeque::new();
    position_to_visit.push_back(find_start(&matrix)?);
    let mut visited_positions: HashSet<Position> = HashSet::new();
    while !position_to_visit.is_empty() {
        let current_pos = position_to_visit.pop_front().unwrap();
        if let Some(pos) = current_pos.should_add_north(&matrix) {
            if !visited_positions.contains(&pos) && !position_to_visit.contains(&pos) {
                position_to_visit.push_back(pos);
            }
        }
        if let Some(pos) = current_pos.should_add_south(&matrix) {
            if !visited_positions.contains(&pos) && !position_to_visit.contains(&pos) {
                position_to_visit.push_back(pos);
            }
        }
        if let Some(pos) = current_pos.should_add_west(&matrix) {
            if !visited_positions.contains(&pos) && !position_to_visit.contains(&pos) {
                position_to_visit.push_back(pos);
            }
        }
        if let Some(pos) = current_pos.should_add_east(&matrix) {
            if !visited_positions.contains(&pos) && !position_to_visit.contains(&pos) {
                position_to_visit.push_back(pos);
            }
        }
        visited_positions.insert(current_pos);
    }

    println!("visited_positions {:?}", visited_positions.len());

    // Replace dead pipes by Ground tile
    let rows: usize = matrix.len();
    let cols: usize = matrix[0].len();
    for (row_idx, row) in matrix.iter_mut().enumerate() {
        for (col, ch) in row.iter_mut().enumerate().take(cols) {
            if !visited_positions.contains(&Position {
                row: row_idx,
                col,
                dist: 0,
                symbol: *ch,
            }) {
                *ch = '.';
            }
        }
    }

    // Expand the matrix * 2
    let mut new_matrix: Vec<Vec<char>> = vec![];
    for _ in 0..(rows * 2) {
        let mut tmp = vec![];
        for _ in 0..(cols * 2) {
            tmp.push(' ')
        }
        new_matrix.push(tmp);
    }

    for row in 0..rows {
        for col in 0..cols {
            new_matrix[row * 2][col * 2] = matrix[row][col];
        }
    }

    let rows = new_matrix.len();
    let cols = new_matrix[0].len();

    // Replace border ground by zero's
    for x in new_matrix[0].iter_mut() {
        if *x == '.' || *x == ' ' {
            *x = '0';
        }
    }

    for x in new_matrix[rows - 1].iter_mut() {
        if *x == '.' || *x == ' ' {
            *x = '0';
        }
    }

    for row in new_matrix.iter_mut().take(rows) {
        if row[0] == '.' || row[0] == ' ' {
            row[0] = '0';
        }
        if row[cols - 1] == '.' || row[cols - 1] == ' ' {
            row[cols - 1] = '0';
        }
    }

    for row in 1..rows {
        for col in 1..cols {
            if new_matrix[row][col] == ' ' {
                if (new_matrix[row - 1][col] == 'F'
                    || new_matrix[row - 1][col] == '|'
                    || new_matrix[row - 1][col] == 'S'
                    || new_matrix[row - 1][col] == '7')
                    && (new_matrix[row + 1][col] == 'J'
                        || new_matrix[row + 1][col] == '|'
                        || new_matrix[row + 1][col] == 'S'
                        || new_matrix[row + 1][col] == 'L')
                {
                    new_matrix[row][col] = '|';
                }

                if (new_matrix[row][col - 1] == 'F'
                    || new_matrix[row][col - 1] == '-'
                    || new_matrix[row][col - 1] == 'S'
                    || new_matrix[row][col - 1] == 'L')
                    && (new_matrix[row][col + 1] == 'J'
                        || new_matrix[row][col + 1] == '-'
                        || new_matrix[row][col + 1] == 'S'
                        || new_matrix[row][col + 1] == '7')
                {
                    new_matrix[row][col] = '-';
                }
            }
        }
    }

    // Bit ugly but works out :D
    // Could do a walking allgortihm instead
    for _ in 1..100 {
        for row in 1..(rows) {
            for col in 1..(cols) {
                if new_matrix[row][col] == '.'
                    && (new_matrix[row - 1][col] == '0'
                        || new_matrix[row + 1][col] == '0'
                        || new_matrix[row][col - 1] == '0'
                        || new_matrix[row][col + 1] == '0')
                {
                    new_matrix[row][col] = '0';
                }

                if new_matrix[row][col] == ' '
                    && (new_matrix[row - 1][col] == '0'
                        || new_matrix[row + 1][col] == '0'
                        || new_matrix[row][col - 1] == '0'
                        || new_matrix[row][col + 1] == '0')
                {
                    new_matrix[row][col] = '0';
                }
            }
        }
    }

    let mut amount = 0;
    for row in new_matrix.iter().take(rows).skip(1) {
        for col in row.iter().take(cols).skip(1) {
            if *col == '.' {
                amount += 1;
            }
        }
    }

    // print(&new_matrix);
    Some(amount)
}

// fn print(matrix: &[Vec<char>]) {
//     println!("_________________________________________________________________");
//     matrix.iter().for_each(|l| {
//         l.iter().for_each(|c| print!("{}", c));
//         println!("")
//     });
//     println!("_________________________________________________________________");
// }
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(80));
        assert_eq!(result, Some(6867));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(595));
    }
}
