use regex::Regex;
use std::collections::HashMap;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut res = 0;
    for x in input.trim_end().split(',') {
        res += get_hash(x);
    }
    Some(res)
}

fn get_hash(str: &str) -> u32 {
    let mut hash = 0;
    for c in str.chars() {
        hash = ((hash + c as u32) * 17) % 256;
    }
    hash
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lens_boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();
    let alpha_re = Regex::new(r"[a-zA-Z]*").unwrap();
    for x in input.trim_end().split(',') {
        let to_hash = alpha_re.find(x).unwrap().as_str();
        let box_number = get_hash(to_hash);
        let current_value = lens_boxes.entry(box_number).or_default();

        if x.contains('=') {
            let lens_size: u32 = x.split('=').last().unwrap().parse().unwrap();
            match current_value.iter().position(|x| x.0 == to_hash) {
                Some(idx) => current_value[idx] = (to_hash, lens_size),
                None => current_value.push((to_hash, lens_size)),
            }
        } else if let Some(idx) = current_value.iter().position(|x| x.0 == to_hash) {
            current_value.remove(idx);
        }
    }
    let mut response = 0;
    for x in 0..256 {
        if let Some(current_box) = lens_boxes.get(&x) {
            for (idx, (_, lens_size)) in current_box.iter().enumerate() {
                response += (x + 1) * (idx as u32 + 1) * lens_size;
            }
        }
    }
    Some(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(1320));
        assert_eq!(result, Some(506891));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(145));
        assert_eq!(result, Some(230462));
    }
}
