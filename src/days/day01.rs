use crate::days::template::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part1(&self, input: &str) -> String {
        let values = parse_values(input);
        values
            .iter()
            .zip(values[1..].iter())
            .fold(0, |r, (prev, cur)| if cur > prev { r + 1 } else { r })
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let values = parse_values(input);
        let first_triples = values.windows(3);
        let second_triples = values[1..].windows(3);

        first_triples
            .into_iter()
            .zip(second_triples.into_iter())
            .fold(0, |r, (prev, cur)| {
                if cur.iter().sum::<i32>() > prev.iter().sum::<i32>() {
                    r + 1
                } else {
                    r
                }
            })
            .to_string()
    }
}

fn parse_values(input: &str) -> Vec<i32> {
    input
        .trim()
        .split_whitespace()
        .map(|it| it.parse::<i32>().unwrap())
        .collect()
}
