use crate::Solution;
use itertools::Itertools;
use regex::Regex;
use std::cmp::{max, Ordering};
use std::collections::HashSet;
use std::ops::RangeInclusive;

pub struct Day17 {}

impl Solution for Day17 {
    fn part1(&self, input: &str) -> String {
        let (x_range, y_range) = parse_input(input);

        let mut max: Option<isize> = None;
        for v_x in potential_v_x(&x_range) {
            for v_y in -1000..1000 {
                let max_y = max_y_if_hit(v_x, v_y, &x_range, &y_range);
                max = max.max(max_y);
            }
        }
        max.unwrap().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (x_range, y_range) = parse_input(input);

        let mut all_hit: HashSet<(isize, isize)> = HashSet::new();
        for v_x in potential_v_x(&x_range) {
            for v_y in -1000..1000 {
                let hit = max_y_if_hit(v_x, v_y, &x_range, &y_range);
                hit.map(|_it| all_hit.insert((v_x, v_y)));
            }
        }
        all_hit.len().to_string()
    }
}

fn potential_v_x(x_range: &RangeInclusive<isize>) -> RangeInclusive<isize> {
    0..=*x_range.end()
}

fn max_y_if_hit(
    start_v_x: isize,
    start_v_y: isize,
    x_range: &RangeInclusive<isize>,
    y_range: &RangeInclusive<isize>,
) -> Option<isize> {
    let (mut x_pos, mut y_pos) = (0, 0);
    let (mut v_x, mut v_y) = (start_v_x, start_v_y);

    let x_bound_max = *x_range.end();
    let y_bound_min = *y_range.start();

    let mut max_y = y_pos;
    let mut hit_target_area = false;
    loop {
        x_pos += v_x;
        y_pos += v_y;
        max_y = max(max_y, y_pos);

        if x_range.contains(&x_pos) && y_range.contains(&y_pos) {
            hit_target_area = true;
        }

        if x_pos > x_bound_max || y_pos < y_bound_min {
            break;
        }

        v_x = next_x_velocity(v_x);
        v_y = next_y_velocity(v_y);
    }

    if hit_target_area {
        Some(max_y)
    } else {
        None
    }
}

fn next_x_velocity(v_prev: isize) -> isize {
    v_prev
        + match v_prev.cmp(&0) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
}

fn next_y_velocity(v_prev: isize) -> isize {
    v_prev - 1
}

fn parse_input(input: &str) -> (RangeInclusive<isize>, RangeInclusive<isize>) {
    let regex = Regex::new(r"-?\d+").unwrap();
    match regex
        .captures_iter(input)
        .map(|m| m.get(0).unwrap().as_str())
        .map(|it| it.parse::<isize>().unwrap())
        .collect_vec()
        .as_slice()
    {
        [x1, x2, y1, y2] => (*x1..=*x2, *y1..=*y2),
        _ => panic!("Invalid input: {}", input),
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day17::Day17;
    use crate::Solution;

    const SAMPLE_1: &'static str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_sample_pt1() {
        assert_eq!(45.to_string(), Day17 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample_pt2() {
        assert_eq!(112.to_string(), Day17 {}.part2(SAMPLE_1));
    }
}
