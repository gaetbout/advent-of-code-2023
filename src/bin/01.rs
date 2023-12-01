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
    let one = Regex::new(r"one").unwrap();
    let two = Regex::new(r"two").unwrap();
    let three = Regex::new(r"three").unwrap();
    let four = Regex::new(r"four").unwrap();
    let five = Regex::new(r"five").unwrap();
    let six = Regex::new(r"six").unwrap();
    let seven = Regex::new(r"seven").unwrap();
    let eight = Regex::new(r"eight").unwrap();
    let nine = Regex::new(r"nine").unwrap();
    let re = Regex::new(r"[A-Za-z]").unwrap();
    Some(
        input
            .trim_end()
            .split('\n')
            .map(|f| {
                let str = one.replace_all(f, "o1e");
                let str = two.replace_all(&str, "t2o");
                let str = three.replace_all(&str, "t3e");
                let str = four.replace_all(&str, "4");
                let str = five.replace_all(&str, "5e");
                let str = six.replace_all(&str, "6");
                let str = seven.replace_all(&str, "7");
                let str = eight.replace_all(&str, "e8t");
                let str = nine.replace_all(&str, "9");
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
        assert_eq!(result, Some(281));
    }
}
