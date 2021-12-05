use crate::days::template::Solution;
use clap::{App, Arg};
use std::path::{Path, PathBuf};

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
    let solution = match day_int {
        1 => days::day01::Day01 {},
        d => panic!("Day {} not yet supported", d),
    };

    let input = read_day_input(&day_int);
    println!("[{},1]: {}", day_int, solution.part1(&input));
    println!("[{},2]: {}", day_int, solution.part2(&input));
}

fn read_day_input(day_int: &usize) -> String {
    let input_path = PathBuf::from(format!("{}/inputs/day{:02}.txt", env!("CARGO_MANIFEST_DIR"), day_int));
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
