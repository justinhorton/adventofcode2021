use std::cmp::Ordering;
use crate::days::template::Solution;
use std::collections::HashMap;

pub struct Day05 {}

impl Solution for Day05 {
    fn part1(&self, input: &str) -> String {
        Self::compute_result(input, |segment| segment.horiz_vert_points())
    }

    fn part2(&self, input: &str) -> String {
        Self::compute_result(input, |segment| segment.points())
    }
}

impl Day05 {
    fn parse_input(input: &str) -> Vec<LineSegment> {
        input
            .trim()
            .lines()
            .map(|line| {
                let mut split = line.split(" -> ");
                LineSegment {
                    start: Point::from(split.next().unwrap()),
                    end: Point::from(split.next().unwrap()),
                }
            })
            .collect()
    }

    fn compute_result(input: &str, get_points: fn(&LineSegment) -> Vec<Point>) -> String {
        let segments = Self::parse_input(input);
        let mut map: HashMap<Point, u64> = HashMap::new();

        for seg in &segments {
            for point in get_points(seg) {
                let new_count = map.get(&point).map(|cur| cur + 1).unwrap_or(1);
                map.insert(point, new_count);
            }
        }

        map.values().filter(|&&it| it > 1).count().to_string()
    }
}

#[derive(Debug, Clone)]
struct LineSegment {
    start: Point,
    end: Point,
}
impl LineSegment {
    fn horiz_vert_points(&self) -> Vec<Point> {
        if self.start.x == self.end.x || self.start.y == self.end.y {
            self.points()
        } else {
            Vec::new()
        }
    }

    fn points(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        let x_step = self.x_step();
        let y_step = self.y_step();
        let mut x = self.start.x;
        let mut y = self.start.y;

        while points.last() != Some(&self.end.clone()) {
            points.push(Point { x, y });
            x += x_step;
            y += y_step;
        }

        points
    }

    fn x_step(&self) -> i64 {
        Self::step(self.start.x, self.end.x)
    }

    fn y_step(&self) -> i64 {
        Self::step(self.start.y, self.end.y)
    }

    fn step(start: i64, end: i64) -> i64 {
        match start.cmp(&end) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => -1,
        }
    }
}
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn from(s: &str) -> Point {
        let mut coords = s.split(',').map(|it| it.parse::<i64>().unwrap());
        Point {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day05::{Day05, LineSegment, Point};
    use crate::Solution;

    const SAMPLE_1: &'static str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_sample_part1() {
        assert_eq!(5.to_string(), Day05 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(12.to_string(), Day05 {}.part2(SAMPLE_1));
    }
}
