use std::cmp::max;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let mut total_sum = 0;
    let lines = input.trim_end().split('\n');
    lines.for_each(|l| {
        let mut game_def = l.split(':');
        let number = game_def.next().unwrap();
        let games: std::str::Split<'_, char> = game_def.next().unwrap().split(';');
        let mut is_valid_game = true;
        games.for_each(|inner_game| {
            let round = inner_game.split(',');
            round.for_each(|inner_game| {
                let mut inner_game = inner_game.split(' ').skip(1);
                let amount: u32 = inner_game.next().unwrap().parse().unwrap();
                let color = inner_game.next().unwrap();
                let max = match color {
                    "red" => MAX_RED,
                    "green" => MAX_GREEN,
                    _ => MAX_BLUE,
                };
                if amount > max {
                    is_valid_game = false;
                }
            });
        });
        if is_valid_game {
            total_sum += number
                .split(' ')
                .nth(1)
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }
    });
    Some(total_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_sum = 0;
    let lines = input.trim_end().split('\n');
    lines.for_each(|l| {
        let mut game_def = l.split(':');
        let games = game_def.nth(1).unwrap().split(';');
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        games.for_each(|inner_game| {
            let round = inner_game.split(',');
            round.for_each(|inner_game| {
                let mut inner_game = inner_game.split(' ').skip(1);
                let amount: u32 = inner_game.next().unwrap().parse().unwrap();
                let color = inner_game.next().unwrap();
                match color {
                    "red" => max_red = max(max_red, amount),
                    "green" => max_green = max(max_green, amount),
                    _ => max_blue = max(max_blue, amount),
                };
            });
        });
        total_sum += max_red * max_blue * max_green;
    });
    Some(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(8));
        assert_eq!(result, Some(2439));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(2286));
        assert_eq!(result, Some(63711));
    }
}
