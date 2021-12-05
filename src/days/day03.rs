use crate::days::template::Solution;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub struct Day03 {}

impl Solution for Day03 {
    fn part1(&self, input: &str) -> String {
        let values = parse_values(input);
        let count_zeroes_ones: Vec<(usize, usize)> = zeroes_and_ones_by_bit(&values);

        let gamma_bin = count_zeroes_ones
            .iter()
            .map(|&counts| if counts.1 > counts.0 { '1' } else { '0' })
            .join("");
        let gamma = usize::from_str_radix(&gamma_bin, 2).unwrap();

        let epsilon_bin = gamma_bin
            .chars()
            .map(|ch| if ch == '1' { '0' } else { '1' })
            .join("");
        let epsilon = usize::from_str_radix(&epsilon_bin, 2).unwrap();

        (gamma * epsilon).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let parsed_values = parse_values(input);
        let value_len = parsed_values[0].len();

        let reduce_by_bit = |values: &mut Vec<&str>, invert: bool, acc, i: usize| {
            let most_common = most_common_char_for_bit(values, i);
            values.retain(|v| {
                let nth_char = v.chars().nth(i);
                let is_most_common = nth_char == Some(most_common);
                if invert {
                    !is_most_common
                } else {
                    is_most_common
                }
            });

            if values.len() != 1 {
                Continue(acc)
            } else {
                Done(values[0].to_string())
            }
        };

        let mut o2_values = parsed_values.to_vec();
        let o2_str = (0..value_len)
            .fold_while(String::new(), |acc, i| {
                reduce_by_bit(&mut o2_values, false, acc, i)
            })
            .into_inner();
        let o2_val = usize::from_str_radix(&o2_str, 2).unwrap();

        let mut co2_values = parsed_values.to_vec();
        let co2_str = (0..value_len)
            .fold_while(String::new(), |acc, i| {
                reduce_by_bit(&mut co2_values, true, acc, i)
            })
            .into_inner();
        let co2_val = usize::from_str_radix(&co2_str, 2).unwrap();

        (o2_val * co2_val).to_string()
    }
}

fn zeroes_and_ones_by_bit(values: &[&str]) -> Vec<(usize, usize)> {
    (0..values[0].len())
        .into_iter()
        .map(|digit_i| {
            // concatenate i-th digit of each line to produce value_len strings of values.len()
            // length
            values.iter().fold(String::new(), |vertical_str, value| {
                let ch = value.chars().nth(digit_i).unwrap();
                format!("{}{}", vertical_str, ch)
            })
        })
        .map(|vertical_str| {
            let ones = vertical_str.chars().filter(|&ch| ch == '1').count();
            (vertical_str.len() - ones, ones)
        })
        .collect()
}

fn most_common_char_for_bit(values: &[&str], i: usize) -> char {
    let count_zeroes_ones: (usize, usize) = zeroes_and_ones_single_bit(values, i);
    if count_zeroes_ones.1 >= count_zeroes_ones.0 {
        '1'
    } else {
        '0'
    }
}

fn zeroes_and_ones_single_bit(values: &[&str], index: usize) -> (usize, usize) {
    *zeroes_and_ones_by_bit(values).get(index).unwrap()
}

fn parse_values(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}
