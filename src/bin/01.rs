advent_of_code::solution!(1);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"[A-Za-z]").unwrap();
    Some(
        input
            .trim_end()
            .split('\n')
            .map(|f| {
                let str = re.replace_all(f, "");
                format!(
                    "{}{}",
                    str.chars().next().unwrap_or(' '),
                    str.chars().last().unwrap_or(' ')
                )
                .parse::<u32>()
                .unwrap_or(0)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"[A-Za-z]").unwrap();
    Some(
        input
            .trim_end()
            .split('\n')
            .map(|f| {
                let str = f
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "4")
                    .replace("five", "5e")
                    .replace("six", "6")
                    .replace("seven", "7")
                    .replace("eight", "e8t")
                    .replace("nine", "9");
                let str = re.replace_all(&str, "");
                format!(
                    "{}{}",
                    str.chars().next().unwrap_or(' '),
                    str.chars().last().unwrap_or(' ')
                )
                .parse::<u32>()
                .unwrap_or(0)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(56042));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55358));
    }
}
