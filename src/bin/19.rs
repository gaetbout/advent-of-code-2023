use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn to_value(&self, str: &str) -> u32 {
        match str {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            _ => self.s,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input = input.trim_end().split('\n');
    let mut workflows: HashMap<&str, Vec<&str>> = HashMap::new();
    loop {
        let line = input.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut line = line.split('{');
        let name = line.next().unwrap();
        let rules = line.next().unwrap();
        let rules = &rules[0..rules.len() - 1];
        workflows.insert(name, rules.split(',').collect());
    }
    let re = Regex::new(r"\d+").unwrap();
    let parts: Vec<Part> = input
        .map(|l| {
            let mut line = l.split('=');
            line.next();

            let x = re
                .find(line.next().unwrap())
                .map(|m| m.as_str())
                .unwrap()
                .parse()
                .unwrap();
            let m = re
                .find(line.next().unwrap())
                .map(|m| m.as_str())
                .unwrap()
                .parse()
                .unwrap();
            let a = re
                .find(line.next().unwrap())
                .map(|m| m.as_str())
                .unwrap()
                .parse()
                .unwrap();
            let s = re
                .find(line.next().unwrap())
                .map(|m| m.as_str())
                .unwrap()
                .parse()
                .unwrap();
            Part { x, m, a, s }
        })
        .collect();
    let mut res = 0;
    for part in parts.iter() {
        // let part = &parts[0];
        println!("PART IS {:?}", part);
        if is_accepted(part, &workflows) {
            println!("ACCEPTED");
            res += part.x + part.m + part.a + part.s;
        }
    }
    Some(res)
}

fn is_accepted(part: &Part, workflows: &HashMap<&str, Vec<&str>>) -> bool {
    let mut current_rule = workflows.get("in").unwrap().iter();
    let mut current_string = current_rule.next().unwrap();
    loop {
        println!("Current string: {}", current_string);
        if current_string.contains('<') {
            let mut init = current_string.split(':');
            let mut left = init.next().unwrap().split('<');
            let part_value: u32 = part.to_value(left.next().unwrap());
            let compa_value: u32 = left.next().unwrap().parse().unwrap();

            if part_value < compa_value {
                let next = init.next().unwrap();
                if next == "A" {
                    return true;
                } else if next == "R" {
                    return false;
                }
                current_rule = workflows.get(next).unwrap().iter();
                current_string = current_rule.next().unwrap();
                continue;
            } else {
                current_string = current_rule.next().unwrap();
                continue;
            }
        } else if current_string.contains('>') {
            let mut init = current_string.split(':');
            let mut left = init.next().unwrap().split('>');
            let part_value: u32 = part.to_value(left.next().unwrap());
            let compa_value: u32 = left.next().unwrap().parse().unwrap();

            if part_value > compa_value {
                let next = init.next().unwrap();
                if next == "A" {
                    return true;
                } else if next == "R" {
                    return false;
                }
                current_rule = workflows.get(next).unwrap().iter();
                current_string = current_rule.next().unwrap();
                continue;
            } else {
                current_string = current_rule.next().unwrap();
                continue;
            }
        } else {
            if current_string.eq(&"A") {
                return true;
            } else if current_string.eq(&"R") {
                return false;
            }
            current_rule = workflows.get(current_string).unwrap().iter();
            current_string = current_rule.next().unwrap();
        }
    }
}

pub fn part_two(_input: &str) -> Option<u32> {
    // Keep all way to reach A status
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
        // assert_eq!(result, Some(397134));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
