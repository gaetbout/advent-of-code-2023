use std::str::Split;
advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    destination: u32,
    source: u32,
    len: u32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.trim_end().split('\n');
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let lists = parse_map(lines);
    Some(get_min(numbers, lists))
}

fn get_min(numbers: Vec<u32>, lists: Vec<Vec<Map>>) -> u32 {
    let mut resp: Vec<u32> = vec![];
    for mut num in numbers {
        for lamas in &lists {
            for map in lamas {
                if num >= map.source && num < (map.source.checked_add(map.len).unwrap_or(u32::MAX))
                {
                    num = num - map.source + map.destination;
                    break;
                }
            }
        }
        resp.push(num);
    }
    *resp.iter().min().unwrap()
}
fn parse_map(lines: Split<'_, char>) -> Vec<Vec<Map>> {
    let mut lists: Vec<Vec<Map>> = vec![];
    let mut current_list = vec![];
    for line in lines.skip(2) {
        if line.is_empty() {
            continue;
        } else if line.chars().next().unwrap().is_alphabetic() {
            lists.push(current_list);
            current_list = vec![];
        } else {
            let lama: Vec<u32> = line.split(' ').map(|s| s.parse().unwrap()).collect();
            current_list.push(Map {
                destination: lama[0],
                source: lama[1],
                len: lama[2],
            })
        }
    }
    lists.push(current_list);
    lists
}

// There is prob a way to make it more efficient, but I just left it running for ~50mn
pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.trim_end().split('\n');
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let lists = parse_map(lines);
    let mut new_numbers = vec![];
    for idx in (0..numbers.len()).step_by(2) {
        let start = numbers[idx];
        let extend = numbers[idx + 1];
        for i in start..(start + extend) {
            new_numbers.push(i);
        }
    }
    new_numbers.dedup();
    Some(get_min(new_numbers, lists))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(35));
        assert_eq!(result, Some(214922730));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(46));
        assert_eq!(result, Some(148041808));
    }
}
