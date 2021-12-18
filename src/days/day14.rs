use crate::Solution;
use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;
use std::fmt::format;

pub struct Day14 {}

impl Solution for Day14 {
    fn part1(&self, input: &str) -> String {
        let polymer_formula = parse_input(input);

        let formula_result = (0..10).fold(String::from(polymer_formula.template), |acc, _i| {
            apply_insertions(&acc, &polymer_formula.insertion_rules)
        });

        let char_counts = formula_result.chars().fold(HashMap::new(), |mut map, ch| {
            *map.entry(ch).or_insert(0) += 1;
            map
        });

        let (min_ch_count, max_ch_count) = match char_counts.values().into_iter().minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            _ => panic!("Something went wrong..."),
        };
        (max_ch_count - min_ch_count).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let polymer_formula = parse_input(input);

        let initial_chars = polymer_formula.template.chars().collect_vec();
        let initial_pairs = initial_chars
            .iter()
            .zip(initial_chars[1..].iter())
            .collect_vec();

        // number of occurrences of each 2-character sequence
        let mut seq_map: HashMap<String, usize> =
            initial_pairs.iter().fold(HashMap::new(), |mut map, &pair| {
                *map.entry(format!("{}{}", *pair.0, *pair.1))
                    .or_insert(0) += 1;
                map
            });
        // number of occurrences of each character
        let mut char_counts: HashMap<char, usize> =
            initial_chars.iter().fold(HashMap::new(), |mut map, ch| {
                *map.entry(*ch).or_insert(0) += 1;
                map
            });

        for _ in 0..40 {
            let seq_and_count: Vec<(String, usize)> = seq_map
                .iter()
                .map(|(k, v)| (k.to_string(), *v))
                .collect_vec();

            let mut cur_seq_map: HashMap<String, usize> = HashMap::new();
            for (seq, count) in seq_and_count {
                // char count increases for each rule match
                let inserted_ch = polymer_formula.insertion_rules.get(&seq).unwrap();
                *char_counts.entry(*inserted_ch).or_insert(0) += count;

                // sequence count increases according to the would-be character pairs
                let new_seq1: String = format!("{}{}", seq.chars().nth(0).unwrap(), inserted_ch);
                let new_seq2: String = format!("{}{}", inserted_ch, seq.chars().nth(1).unwrap());
                *cur_seq_map.entry(new_seq1).or_insert(0) += count;
                *cur_seq_map.entry(new_seq2).or_insert(0) += count;
            }

            seq_map = cur_seq_map;
        }

        let (min_ch_count, max_ch_count) = match char_counts.values().into_iter().minmax() {
            MinMaxResult::MinMax(min, max) => (min, max),
            _ => panic!("Something went wrong..."),
        };

        (max_ch_count - min_ch_count).to_string()
    }
}

fn apply_insertions(template: &String, rules: &HashMap<String, char>) -> String {
    let chars = template.chars().collect_vec();
    let pairs = chars.iter().zip(chars[1..].iter()).collect_vec();

    let mut result = String::from("");
    for (index, pair) in pairs.iter().enumerate() {
        let (&first, &second) = pair;

        if index == 0 {
            result.push(first);
        }

        let seq = format!("{}{}", first, second);
        result.push(*rules.get(seq.as_str()).unwrap());

        result.push(second)
    }

    result
}

fn parse_input(input: &str) -> PolymerFormula {
    let split = input.trim().split("\n\n").collect_vec();
    let template = split.get(0).unwrap().to_string();

    let insertion_rules: HashMap<String, char> = split
        .get(1)
        .unwrap()
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|it| (it.0.to_string(), it.1.chars().nth(0).unwrap()))
        .collect();

    PolymerFormula {
        template,
        insertion_rules,
    }
}

#[derive(Debug)]
struct PolymerFormula {
    template: String,
    insertion_rules: HashMap<String, char>,
}

#[cfg(test)]
mod tests {
    use crate::days::day14::Day14;
    use crate::Solution;

    const SAMPLE_1: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_sample1_part1() {
        assert_eq!(1588.to_string(), Day14 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample1_part2() {
        assert_eq!(2188189693529usize.to_string(), Day14 {}.part2(SAMPLE_1));
    }
}
