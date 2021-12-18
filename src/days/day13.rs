use crate::days::day13::Fold::{Left, Up};
use crate::Solution;
use itertools::Itertools;
use std::cmp::max;

pub struct Day13 {}

const CHAR_FILL: char = '⚫';
const CHAR_NO_FILL: char = '⚪';

impl Solution for Day13 {
    fn part1(&self, input: &str) -> String {
        let (points, folds) = parse_input(input);

        let visible_points_1_fold = folds
            .iter()
            .take(1)
            .fold(points, |acc_points, fold| {
                acc_points.iter().map(|p| p.translate(fold)).collect_vec()
            })
            .into_iter()
            .unique()
            .count();

        visible_points_1_fold.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let (points, folds) = parse_input(input);

        let visible_points = folds
            .iter()
            .fold(points.iter().copied().collect_vec(), |acc_points, fold| {
                acc_points.iter().map(|p| p.translate(fold)).collect_vec()
            })
            .into_iter()
            .unique()
            .collect_vec();

        // draw the answer
        let (max_x, max_y) = &visible_points.iter().fold((0, 0), |(acc_x, acc_y), cur| {
            (max(cur.x, acc_x), max(cur.y, acc_y))
        });

        let mut chars: Vec<char> = vec!['\n'];
        for y in 0..=*max_y {
            for x in 0..=*max_x {
                if visible_points.contains(&Point { x, y }) {
                    chars.push(CHAR_FILL)
                } else {
                    chars.push(CHAR_NO_FILL)
                }
            }
            chars.push('\n')
        }

        String::from_iter(chars)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(point_str: &str) -> Point {
        match *point_str.split(',').collect_vec().as_slice() {
            [x, y] => Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            },
            _ => panic!("Bad point: {}", point_str),
        }
    }

    fn translate(&self, fold: &Fold) -> Point {
        match fold {
            Up(y_coord) => {
                if self.y > *y_coord {
                    let offset = (self.y - y_coord) * 2;
                    Point {
                        x: self.x,
                        y: self.y - offset,
                    }
                } else {
                    *self
                }
            }
            Left(x_coord) => {
                if self.x > *x_coord {
                    let offset = (self.x - x_coord) * 2;
                    Point {
                        x: self.x - offset,
                        y: self.y,
                    }
                } else {
                    *self
                }
            }
        }
    }
}

fn parse_input(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let (point_data, fold_data) = input.trim().split_once("\n\n").unwrap();

    let points = point_data.trim().lines().map(Point::from).collect_vec();

    let folds = fold_data
        .trim()
        .lines()
        .map(|f| {
            let on_axis: usize = f.split('=').collect_vec().get(1).unwrap().parse().unwrap();
            if f.contains("y=") {
                Up(on_axis)
            } else {
                Left(on_axis)
            }
        })
        .collect_vec();

    (points, folds)
}

#[derive(Debug)]
enum Fold {
    Up(usize),
    Left(usize),
}

#[cfg(test)]
mod tests {
    use crate::days::day13::Day13;
    use crate::Solution;

    const SAMPLE_1: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_sample1_part1() {
        assert_eq!(17.to_string(), Day13 {}.part1(SAMPLE_1));
    }
}
