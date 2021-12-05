use crate::days::template::Solution;
use itertools::Itertools;

pub struct Day02 {}

impl Solution for Day02 {
    fn part1(&self, input: &str) -> String {
        let pos: (i32, i32) = parse_values(input)
            .iter()
            .fold((0, 0), |(x, y), (dir, delta)| match dir {
                Direction::Up => (x, y - delta),
                Direction::Down => (x, y + delta),
                Direction::Forward => (x + delta, y),
            });
        (pos.0 * pos.1).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let pos: (i32, i32, i32) =
            parse_values(input)
                .iter()
                .fold((0, 0, 0), |(x, y, aim), (dir, delta)| match dir {
                    Direction::Up => (x, y, aim - delta),
                    Direction::Down => (x, y, aim + delta),
                    Direction::Forward => (x + delta, y + aim * delta, aim),
                });
        (pos.0 * pos.1).to_string()
    }
}

fn parse_values(input: &str) -> Vec<(Direction, i32)> {
    input
        .trim()
        .lines()
        .map(|s| s.split_whitespace().into_iter().next_tuple().unwrap())
        .map(|(dir_str, num)| {
            let dir = match dir_str {
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => Direction::Forward,
            };
            (dir, num.parse::<i32>().unwrap())
        })
        .collect()
}

enum Direction {
    Up,
    Down,
    Forward,
}
