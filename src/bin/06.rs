advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.trim_end().split('\n');
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let mut response = 1;
    for (time, dist) in times.iter().zip(distances) {
        let mut local_res = 0;
        for x in 1..*time {
            // Could break after realising too short
            if (time - x) * x > dist {
                local_res += 1;
            }
        }
        response *= local_res;
    }
    Some(response)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.trim_end().split('\n');
    let time: u128 = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let dist: u128 = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let mut response = 0;
    for x in 1..time {
        // Could break after realising too short
        if (time - x) * x > dist {
            response += 1;
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
        // assert_eq!(result, Some(288));
        assert_eq!(result, Some(316800));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(71503));
        assert_eq!(result, Some(45647654));
    }
}
