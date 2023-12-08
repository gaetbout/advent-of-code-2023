use gcd::Gcd;
use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.trim_end().split('\n');
    let directions: Vec<char> = lines.next().unwrap().chars().collect();

    let mut instructions: HashMap<(&str, char), &str> = HashMap::new();
    for l in lines.skip(1) {
        let mut l_split = l.split('=');
        let key = &(l_split.next().unwrap())[0..3];
        let l_r = l_split.next().unwrap();
        instructions.insert((key, 'L'), &l_r[2..5]);
        instructions.insert((key, 'R'), &l_r[7..10]);
    }
    let mut response = 0;

    let mut key = "AAA";
    while key != "ZZZ" {
        key = instructions
            .get(&(key, directions[response % directions.len()]))
            .unwrap();
        response += 1;
    }
    Some(response as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.trim_end().split('\n');
    let directions: Vec<char> = lines.next().unwrap().chars().collect();

    let mut keys = vec![];
    let mut instructions: HashMap<(&str, char), &str> = HashMap::new();
    for l in lines.skip(1) {
        let mut l_split = l.split('=');
        let key = &(l_split.next().unwrap())[0..3];
        if key.ends_with('A') {
            keys.push(key);
        }
        let l_r = l_split.next().unwrap();
        instructions.insert((key, 'L'), &l_r[2..5]);
        instructions.insert((key, 'R'), &l_r[7..10]);
    }

    // Won't lie, I had to get some hints for this part
    let mut lcm = 1;
    for key in &keys {
        let mut sum = 0;
        let mut dest = key;
        while !dest.ends_with('Z') {
            dest = instructions
                .get(&(dest, directions[sum % directions.len()]))
                .unwrap();
            sum += 1;
        }
        lcm = (lcm * sum) / lcm.gcd(sum);
    }
    Some(lcm as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(6));
        assert_eq!(result, Some(12361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(6));
        assert_eq!(result, Some(18215611419223));
    }
}
