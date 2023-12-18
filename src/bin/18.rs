use std::{
    cmp,
    collections::{HashMap, HashSet},
    ops::Range,
};

advent_of_code::solution!(18);

const START: usize = 1000;
pub fn part_one(input: &str) -> Option<usize> {
    let mut coords: HashSet<(usize, usize)> = HashSet::new();
    let mut current_row = START;
    let mut current_col = START;
    for line in input.trim_end().split('\n') {
        let mut line = line.split_whitespace();
        let direction = line.next().unwrap();
        let amount: u32 = line.next().unwrap().parse().unwrap();

        for _ in 0..amount {
            coords.insert((current_row, current_col));
            match direction {
                "R" => {
                    current_col += 1;
                }
                "D" => {
                    current_row += 1;
                }
                "L" => {
                    current_col -= 1;
                }
                _ => {
                    current_row -= 1;
                }
            }
        }
    }

    // helper_print_map(&coords);
    let mut min_row = START;
    for (row, _) in coords.iter() {
        min_row = cmp::min(min_row, *row);
    }
    let mut cols: Vec<usize> = coords
        .iter()
        .filter(|(local_row, _)| *local_row == min_row)
        .map(|c| c.1)
        .collect();
    cols.sort();

    let mut to_visit = vec![(min_row + 1, cols[0] + 1)];
    while let Some((row, col)) = to_visit.pop() {
        coords.insert((row, col));
        if !coords.contains(&(row - 1, col)) {
            to_visit.push((row - 1, col))
        }
        if !coords.contains(&(row + 1, col)) {
            to_visit.push((row + 1, col))
        }
        if !coords.contains(&(row, col - 1)) {
            to_visit.push((row, col - 1))
        }
        if !coords.contains(&(row, col + 1)) {
            to_visit.push((row, col + 1))
        }
    }
    // helper_print_map(&coords);

    Some(coords.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut coords: HashMap<usize, Vec<Range<usize>>> = HashMap::new();
    let mut current_row: usize = 100000000;
    let mut current_col: usize = 100000000;
    for line in input.trim_end().split('\n') {
        let hex_value = line.split_whitespace().last().unwrap();

        let direction = match &hex_value[hex_value.len() - 2..hex_value.len() - 1] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            _ => "U",
        };

        let amount: usize =
            i64::from_str_radix(&hex_value[2..hex_value.len() - 2], 16).unwrap() as usize;
        println!("{}", amount);
        println!("{}", direction);

        match direction {
            "R" => {
                let val = coords.entry(current_row).or_default();
                val.push(current_col..current_col + amount);
                current_col += amount;
            }
            "D" => {
                for _ in 0..amount {
                    let val = coords.entry(current_row).or_default();
                    val.push(current_col..(current_col + 1));
                    current_row += 1;
                }
            }
            "L" => {
                let val = coords.entry(current_row).or_default();
                val.push(current_col - amount..current_col);
                current_col -= amount;
            }
            _ => {
                for _ in 0..amount {
                    let val = coords.entry(current_row).or_default();
                    val.push(current_col..current_col + 1);
                    current_row -= 1;
                }
            }
        }
        // println!("{:?}", coords);
    }

    let mut keys: Vec<&usize> = coords.keys().collect();
    keys.sort();
    println!("_____________________________________________________________________________________________");
    println!("_____________________________________________________________________________________________");
    // TODO Figure out how to compute range computation
    // Keep a window of opened stuff and close as it comes.
    // For each row compute sum of all opened ranges
    for key in keys {
        let mut cols = coords.get(key).unwrap().clone();
        cols.sort_by(|a, b| a.start.cmp(&b.start));
        println!("{:?}", cols);
    }
    Some(coords.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
        // 71927 too low 87350 too high
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(952408144115));
    }
}
