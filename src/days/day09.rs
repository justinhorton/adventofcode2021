use crate::days::template::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day09 {}

type Basin = usize;
type Tile = usize;

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let (width, parsed_input) = Self::parse_input(input);
        let total_len = parsed_input.len();

        let total_risk: usize = parsed_input
            .iter()
            .enumerate()
            .filter(|(index, this_value)| {
                Self::adjacent_values_with_indices(&parsed_input, width, total_len, index)
                    .iter()
                    .all(|(that_value, _)| *that_value > this_value)
            })
            .map(|(_, value)| 1 + value)
            .sum();
        total_risk.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (width, parsed_input) = Self::parse_input(input);
        let total_len = parsed_input.len();

        let mut seen: HashSet<Tile> = HashSet::new();
        let mut count_by_basin: HashMap<Basin, i32> = HashMap::new();
        let mut to_visit: Vec<Tile> = Vec::new();

        for (i, tile_index) in parsed_input
            .iter()
            .enumerate()
            .filter(|(_, tile_value)| **tile_value < 9)
            .map(|(index, _)| index)
            .enumerate()
        {
            let basin: Basin = i;

            if !seen.contains(&tile_index) {
                to_visit.push(tile_index);
            }

            while !to_visit.is_empty() {
                let next: Tile = to_visit.pop().unwrap();

                if seen.insert(next) {
                    let new_count = *count_by_basin.get(&basin).unwrap_or(&0) + 1;
                    count_by_basin.insert(basin, new_count);

                    for a in
                        Self::adjacent_indices(&parsed_input, width, total_len, &next, |&v| v < 9)
                    {
                        if !seen.contains(&a) {
                            to_visit.push(a)
                        }
                    }
                }
            }
        }

        let result: i32 = count_by_basin
            .into_values()
            .sorted()
            .into_iter()
            .rev()
            .take(3)
            .product();
        result.to_string()
    }
}

impl Day09 {
    fn parse_input(input: &str) -> (usize, Vec<usize>) {
        let trimmed = input.trim();
        let width = trimmed.lines().next().unwrap().len();
        let parsed = trimmed
            .lines()
            .flat_map(|line| {
                line.trim()
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect();
        (width, parsed)
    }

    fn adjacent_indices(
        parsed_input: &[usize],
        width: usize,
        total_len: usize,
        index: &Tile,
        value_filter: fn(&usize) -> bool,
    ) -> Vec<Tile> {
        Self::adjacent_values_with_indices(parsed_input, width, total_len, index)
            .iter()
            .filter(|(value, _)| value_filter(value))
            .map(|(_, index)| *index)
            .collect()
    }

    fn adjacent_values_with_indices<'a>(
        parsed_input: &'a [usize],
        width: usize,
        total_len: usize,
        index: &Tile,
    ) -> Vec<(&'a Tile, usize)> {
        let mut result: Vec<(&usize, usize)> = Vec::new();

        if index % width != 0 {
            result.push(
                parsed_input
                    .get(index - 1)
                    .map(|it| (it, index - 1))
                    .unwrap(),
            )
        }

        if (index + 1) % width != 0 {
            result.push(
                parsed_input
                    .get(index + 1)
                    .map(|it| (it, index + 1))
                    .unwrap(),
            )
        }

        if !((0..width).contains(index)) {
            result.push(
                parsed_input
                    .get(index - width)
                    .map(|it| (it, index - width))
                    .unwrap(),
            )
        }
        if !(((total_len - width)..total_len).contains(index)) {
            result.push(
                parsed_input
                    .get(index + width)
                    .map(|it| (it, index + width))
                    .unwrap(),
            )
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day09::Day09;
    use crate::Solution;

    const SAMPLE_1: &'static str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_sample_part1() {
        assert_eq!(15.to_string(), Day09 {}.part1(SAMPLE_1))
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(1134.to_string(), Day09 {}.part2(SAMPLE_1));
    }
}
