use crate::days::template::Solution;
use clap::{App, Arg};
use std::path::PathBuf;

mod days;

fn main() {
    let app = App::new("AoC Runner")
        .version("1.0")
        .about("Runs AoC 2021 solutions")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .required(true)
                .value_name("DAY_NUMBER")
                .validator(is_valid_day),
        );

    let matches = app.get_matches();

    let day_int = matches
        .value_of("day")
        .map(|s| s.parse::<usize>().unwrap())
        .unwrap();
    let solution = get_day_impl(day_int);

    let input = read_day_input(&day_int);
    println!("[{},1]: {}", day_int, solution.part1(&input));
    println!("[{},2]: {}", day_int, solution.part2(&input));
}

fn get_day_impl(day_int: usize) -> Box<dyn Solution> {
    match day_int {
        1 => Box::new(days::day01::Day01 {}),
        2 => Box::new(days::day02::Day02 {}),
        3 => Box::new(days::day03::Day03 {}),
        4 => Box::new(days::day04::Day04 {}),
        5 => Box::new(days::day05::Day05 {}),
        6 => Box::new(days::day06::Day06 {}),
        7 => Box::new(days::day07::Day07 {}),
        8 => Box::new(days::day08::Day08 {}),
        9 => Box::new(days::day09::Day09 {}),
        10 => Box::new(days::day10::Day10 {}),
        11 => Box::new(days::day11::Day11 {}),
        12 => Box::new(days::day12::Day12 {}),
        13 => Box::new(days::day13::Day13 {}),
        14 => Box::new(days::day14::Day14 {}),
        15 => Box::new(days::day15::Day15 {}),
        d => panic!("Day {} not yet supported", d),
    }
}

fn read_day_input(day_int: &usize) -> String {
    let input_path = PathBuf::from(format!(
        "{}/inputs/day{:02}.txt",
        env!("CARGO_MANIFEST_DIR"),
        day_int
    ));
    let read = std::fs::read_to_string(input_path);
    match read {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e),
    }
}

fn is_valid_day(val: String) -> Result<(), String> {
    match val.parse::<usize>() {
        Ok(int_val) => {
            if (1..=25).contains(&int_val) {
                Ok(())
            } else {
                Err(String::from("must be in the range 1 < DAY_NUMBER <= 25"))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
