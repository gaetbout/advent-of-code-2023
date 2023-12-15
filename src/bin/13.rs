use std::cmp;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let mut response = 0;
    for pattern in create_patterns(input) {
        response += get_amount(&pattern, 0);
    }

    Some(response)
}

fn create_patterns(input: &str) -> Vec<Vec<Vec<char>>> {
    let patterns: Vec<&str> = input.trim_end().split('\n').collect();
    let mut to_sum: Vec<Vec<Vec<char>>> = vec![];
    let mut current_pattern: Vec<Vec<char>> = vec![];
    for line in patterns {
        if line.is_empty() {
            to_sum.push(current_pattern);
            current_pattern = vec![];
            continue;
        }
        current_pattern.push(line.chars().collect());
    }
    to_sum.push(current_pattern);
    to_sum
}
// Same logic for vertical line, just need to rotate the matrix
fn get_amount(pattern: &[Vec<char>], max_mistake: usize) -> usize {
    let max_dist = get_mirror_index(&rotate(pattern), max_mistake);
    let sec_max = get_mirror_index(pattern, max_mistake);
    cmp::max(100 * sec_max, max_dist)
}

fn get_mirror_index(pattern: &[Vec<char>], max_mistake: usize) -> usize {
    for mirror_index in 1..pattern.len() {
        if compare_lines(pattern, mirror_index, max_mistake) {
            return mirror_index;
        }
    }
    0
}

fn compare_lines(pattern: &[Vec<char>], mirror_index: usize, max_mistake: usize) -> bool {
    let mut current_mistake = 0;
    for dist in 0..mirror_index {
        if mirror_index + dist >= pattern.len() {
            return current_mistake == max_mistake;
        }
        for (a, b) in pattern[mirror_index - 1 - dist]
            .iter()
            .zip(pattern[mirror_index + dist].iter())
        {
            if a != b {
                if current_mistake == max_mistake {
                    return false;
                }
                current_mistake += 1;
            }
        }
    }
    current_mistake == max_mistake
}

fn rotate(pattern: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut response: Vec<Vec<char>> = vec![];
    for x in 0..pattern[0].len() {
        let mut current_vec: Vec<char> = vec![];
        for line in pattern {
            current_vec.push(line[x]);
        }
        response.push(current_vec);
    }
    response
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut response = 0;
    for pattern in create_patterns(input) {
        response += get_amount(&pattern, 1);
    }

    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(405));
        assert_eq!(result, Some(40006));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(400));
        assert_eq!(result, Some(28627));
    }
}
