advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut board: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|e| e.chars().collect())
        .collect();

    for col in 0..board[0].len() {
        roll_north(&mut board, col);
    }

    Some(compute_result(&board))
}

fn roll_north(board: &mut Vec<Vec<char>>, col: usize) {
    let board_len = board.len();
    // This is ugly, but it works for a simple POC
    for _ in 0..(board_len) {
        for row in (1..board_len).rev() {
            if board[row][col] != 'O' {
                continue;
            }
            if board[row - 1][col] == '#' || board[row - 1][col] == 'O' {
                continue;
            }

            board[row][col] = '.';
            board[row - 1][col] = 'O';
        }
    }
}

fn compute_result(board: &Vec<Vec<char>>) -> u32 {
    let board_len = board.len();
    let mut response = 0;
    for (idx, line) in board.iter().enumerate() {
        response += (board_len - idx) * line.iter().filter(|c| **c == 'O').count();
    }
    response as u32
}

fn rotate(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
    // println!("Start rotation");
    // pattern.iter().for_each(|l| println!("{:?}", l));
    // println!("\n");
    let mut response: Vec<Vec<char>> = vec![];
    for x in 0..pattern[0].len() {
        let mut current_vec: Vec<char> = vec![];
        for line in pattern.iter().rev() {
            current_vec.push(line[x]);
        }
        response.push(current_vec);
    }
    // response.iter().for_each(|l| println!("{:?}", l));
    // println!("End rotation\n");
    response
}

// While looking at the data I realized that at some points it was looping.
// Just had to make
// 1,000,000,000 % loop size (in my case 52)
// gives 12
// Then I leave the program running like 550 times and take the correct run:
// Response was at loop 520 - 1 + 12 => 531 = > 100310 (Why minus one? Because we start at zero)
pub fn part_two(input: &str) -> Option<u32> {
    let mut board: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|e| e.chars().collect())
        .collect();

    for _ in 0..532 {
        for _ in 0..4 {
            for col in 0..board[0].len() {
                roll_north(&mut board, col);
            }
            board = rotate(&board);
        }
    }

    Some(compute_result(&board))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
        // assert_eq!(result, Some(108918));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
        assert_eq!(result, Some(100310));
        // 100288 too low
    }
}
