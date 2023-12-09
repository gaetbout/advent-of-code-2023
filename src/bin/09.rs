advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.trim_end().split('\n');

    Some(
        lines
            .map(|l| {
                let mut line: Vec<i32> = l.split_whitespace().map(|n| n.parse().unwrap()).collect();
                let mut lasts: Vec<i32> = vec![];
                while line.iter().any(|n| *n != 0) {
                    lasts.push(*line.last().unwrap());
                    let mut next_vec = vec![];
                    for x in 1..line.len() {
                        next_vec.push(line[x] - line[x - 1]);
                    }
                    line = next_vec;
                }
                lasts.iter().sum::<i32>()
            })
            .sum::<i32>(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.trim_end().split('\n');
    let mut res = 0_i32;
    for l in lines {
        let mut line: Vec<i32> = l
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .rev()
            .collect();
        let mut lasts: Vec<i32> = vec![];
        while line.iter().any(|n| *n != 0) {
            lasts.push(*line.last().unwrap());
            let mut next_vec = vec![];
            for x in 1..line.len() {
                next_vec.push(line[x] - line[x - 1]);
            }
            line = next_vec;
        }
        let mut local_res = lasts[0];
        for item in lasts.iter().skip(1) {
            local_res += item;
        }
        res += local_res;
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(114));
        assert_eq!(result, Some(1993300041));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(2));
        assert_eq!(result, Some(1038));
    }
}
