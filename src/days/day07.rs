use crate::days::template::Solution;
use itertools::Itertools;
use std::iter::repeat_with;

pub struct Day07 {}

impl Solution for Day07 {
    fn part1(&self, input: &str) -> String {
        let sorted: Vec<i32> = Self::sorted_input(input);
        let median: i32 = sorted[sorted.len() / 2];
        let result: i32 = sorted.iter().map(|n| (median - n).abs()).sum();
        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let sorted: Vec<i32> = Self::sorted_input(input);
        // the optimal target might be anywhere from the min to max input number, at least naively
        let start_range = *sorted.first().unwrap();
        let end_range = *sorted.last().unwrap();
        let result = (start_range..=end_range)
            .min_by_key(|target| Self::total_cost_for_target(&sorted, target))
            .map(|chosen_target| Self::total_cost_for_target(&sorted, &chosen_target))
            .unwrap();
        result.to_string()
    }
}

impl Day07 {
    fn sorted_input(input: &str) -> Vec<i32> {
        input
            .trim()
            .split(',')
            .map(|it| it.parse::<i32>().unwrap())
            .sorted()
    }

    fn total_cost_for_target(sorted: &[i32], target: &i32) -> i32 {
        let cost_fn: fn(i32, i32) -> i32 = |value: i32, target: i32| {
            let mut cost = 0;
            let mut cost_inc = 1;

            // trying this unnecessary trick out
            repeat_with(|| {
                cost += cost_inc;
                cost_inc += 1;
                cost
            })
            .take((target - value).abs() as usize)
            .last()
            .unwrap_or(0)
        };
        sorted.iter().map(|n| cost_fn(*n, *target)).sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::Day07;
    use crate::Solution;

    const SAMPLE_1: &'static str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_sample_part1() {
        assert_eq!(37.to_string(), Day07 {}.part1(SAMPLE_1))
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(168.to_string(), Day07 {}.part2(SAMPLE_1));
    }
}
