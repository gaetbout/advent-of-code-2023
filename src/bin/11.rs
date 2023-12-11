use std::cmp::{max, min};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u128> {
    do_logic(input, 2)
}

fn get_coords(image: &Vec<Vec<char>>) -> Vec<(u128, u128)> {
    let mut coords = vec![];
    for row in 0..image.len() {
        let len = image[row].len();
        for col in 0..len {
            if image[row][col] != '.' {
                coords.push((row as u128, col as u128));
            }
        }
    }
    coords
}

fn do_logic(input: &str, expansion: u128) -> Option<u128> {
    let image: Vec<Vec<char>> = input
        .trim_end()
        .split('\n')
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut rows = vec![];
    let mut cols = vec![];
    for row in 0..image.len() {
        if image[row].iter().all(|c| *c == '.') {
            rows.push(row);
        }
        let mut is_valid = true;
        for col in 0..image.len() {
            if image[col][row] == '#' {
                is_valid = false;
            }
        }
        if is_valid {
            cols.push(row);
        }
    }
    let coords = get_coords(&image);
    let mut distances = vec![];
    let mut exp_tot = 0;
    for x in 0..coords.len() {
        let (curr_x, curr_y) = coords[x];
        for y in x + 1..coords.len() {
            let (next_x, next_y) = coords[y];
            let (min_x, max_x) = min_max(curr_x, next_x);
            let (min_y, max_y) = min_max(curr_y, next_y);
            let x_range = min_x..max_x;
            let y_range = min_y..max_y;
            for row in rows.clone() {
                if x_range.contains(&(row as u128)) {
                    exp_tot += 1;
                }
            }
            for col in cols.clone() {
                if y_range.contains(&(col as u128)) {
                    exp_tot += 1;
                }
            }
            distances.push(max_x - min_x + max_y - min_y)
        }
    }
    Some((exp_tot * (expansion - 1)) + distances.iter().sum::<u128>())
}

pub fn part_two(input: &str) -> Option<u128> {
    do_logic(input, 1000000)
}

// Will return (MIN, MAX)
fn min_max(x: u128, y: u128) -> (u128, u128) {
    let max = max(x, y);
    let min = min(x, y);
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(374));
        assert_eq!(result, Some(10885634));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(8410));
        assert_eq!(result, Some(707505470642));
    }
}
