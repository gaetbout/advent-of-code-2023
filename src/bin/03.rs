use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim_end().split('\n');
    let mut data: Vec<Vec<char>> = vec![];
    lines.for_each(|line| data.push(line.chars().collect()));
    let mut res = 0;
    for row in 0..data.len() {
        for column in 0..data[row].len() {
            let el = data[row][column];
            if el != '.' && !el.is_ascii_digit() {
                res += find_around(row, column, &data);
            }
        }
    }

    Some(res)
}

fn find_around(row: usize, column: usize, data: &[Vec<char>]) -> u32 {
    let mut resp = HashSet::new();

    for x in row - 1..row + 2 {
        for y in column - 1..column + 2 {
            if data[x][y].is_ascii_digit() {
                resp.insert(read_number(x, y, data));
            }
        }
    }
    resp.into_iter().sum()
}

fn read_number(row: usize, column: usize, data: &[Vec<char>]) -> u32 {
    let row_data = &data[row];
    let mut local_colum_start = column;
    while row_data[local_colum_start - 1].is_ascii_digit() {
        if local_colum_start == 1 {
            if row_data[0].is_ascii_digit() {
                local_colum_start = 0;
            }
            break;
        }
        local_colum_start -= 1;
    }

    let mut local_colum_end = column;
    while row_data[local_colum_end + 1].is_ascii_digit() {
        if local_colum_end == row_data.len() - 2 {
            if row_data[row_data.len() - 1].is_ascii_digit() {
                local_colum_end = row_data.len() - 1;
            }
            break;
        }
        local_colum_end += 1;
    }

    let mut res = 0;
    for x in row_data
        .iter()
        .take(local_colum_end + 1)
        .skip(local_colum_start)
    {
        res = (res * 10) + x.to_digit(10).unwrap()
    }
    res
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim_end().split('\n');
    let mut data: Vec<Vec<char>> = vec![];
    lines.for_each(|line| data.push(line.chars().collect()));
    let mut res = 0;
    for row in 0..data.len() {
        for column in 0..data[row].len() {
            let el = data[row][column];
            if el == '*' {
                res += find_around_v2(row, column, &data);
            }
        }
    }

    Some(res)
}

fn find_around_v2(row: usize, column: usize, data: &[Vec<char>]) -> u32 {
    let mut resp = HashSet::new();

    for x in row - 1..row + 2 {
        for y in column - 1..column + 2 {
            if data[x][y].is_ascii_digit() {
                // println!("row: {} column: {} el: {:?}", x, y, data[x][y]);
                resp.insert(read_number(x, y, data));
            }
        }
    }
    if resp.len() == 2 {
        resp.into_iter().product()
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(4361));
        assert_eq!(result, Some(544664));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(467835));
        assert_eq!(result, Some(84495585));
    }
}
