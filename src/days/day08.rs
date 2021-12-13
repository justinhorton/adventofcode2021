use crate::days::template::Solution;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day08 {}

impl Solution for Day08 {
    fn part1(&self, input: &str) -> String {
        let unique_segment_counts = [2, 4, 3, 7]; // 1: 2, 4: 4, 7: 3, 8: 7
        input
            .trim()
            .lines()
            .map(|line| {
                line.split(" | ")
                    .skip(1)
                    .flat_map(|it| it.split_whitespace())
                    .filter(|it| unique_segment_counts.contains(&it.len()))
                    .count()
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut displays: Vec<BrokenDisplayInfo> =
            input.trim().lines().map(BrokenDisplayInfo::from).collect();

        displays
            .iter_mut()
            .map(|display| {
                let decoder = display.build_decoder();
                decoder.decode(display.output_shown)
            })
            .sum::<u32>()
            .to_string()
    }
}

struct BrokenDisplayInfo<'a> {
    sequences_shown: [&'a str; 10],
    output_shown: [&'a str; 4],
}

struct Decoder {
    letter_decoder: HashMap<char, char>,
}

const UNRESOLVED: i8 = -1;

impl BrokenDisplayInfo<'_> {
    fn from(line: &str) -> BrokenDisplayInfo {
        let sequences_and_output: Vec<Vec<&str>> = line
            .split(" | ")
            .map(|part| part.split_whitespace().collect())
            .collect();
        let mut iter = sequences_and_output.iter();
        let mut sequences_shown = [""; 10];
        let mut output_shown = [""; 4];

        for (i, shown) in iter.next().unwrap().iter().enumerate() {
            sequences_shown[i] = shown
        }

        for (i, output) in iter.next().unwrap().iter().enumerate() {
            output_shown[i] = output;
        }

        BrokenDisplayInfo {
            sequences_shown,
            output_shown,
        }
    }

    /*
    # of segments:
    0: 6
    1: 2*
    2: 5
    3: 5
    4: 4*
    5: 5
    6: 6
    7: 3*
    8: 7*
    9: 6

    # of times on:
    a: 8
    b: 6*
    c: 8
    d: 7
    e: 4*
    f: 9*
    g: 7

    occurrences in 1,4,7,8 and overall:
    a -> 2x, 8x
    c -> 4x, 8x
    d -> 2x, 7x
    g -> 2x, 7x
    */
    fn build_decoder(&mut self) -> Decoder {
        let mut letter_decoder: HashMap<char, char> = HashMap::new();
        let mut resolved_sequences: [i8; 10] = [UNRESOLVED; 10];

        // sequences that must be these numbers due to unique segment count
        for (i, s) in self.sequences_shown.iter().enumerate() {
            resolved_sequences[i] = match s.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                _ => UNRESOLVED,
            };
        }

        let mut occurrences: HashMap<char, u32> = HashMap::new();
        for c in self.sequences_shown.iter().flat_map(|it| it.chars()) {
            *occurrences.entry(c).or_default() += 1;
        }

        let mut occurrences_in_1478: HashMap<char, u32> = HashMap::new();
        for ch in self
            .sequences_shown
            .iter()
            .enumerate()
            .filter(|(i, _)| resolved_sequences[*i] != UNRESOLVED)
            .flat_map(|(_, shown)| shown.chars())
        {
            *occurrences_in_1478.entry(ch).or_default() += 1
        }

        // fake segments that must correspond to these real segments based on number of occurrences
        for (shown_segment, times_appeared) in &occurrences {
            let real_segment: Option<char> = match times_appeared {
                6 => Some('b'),
                4 => Some('e'),
                9 => Some('f'),
                _ => None,
            };
            real_segment.map(|it| letter_decoder.insert(*shown_segment, it));
        }

        // remaining segments can be resolved by checking for numbers of occurrences in the 4 known
        // numbers and all numbers

        // 'c' appears 4x in 1478, 8x overall
        letter_decoder.insert(
            Self::single_char_with_occurrences(&occurrences_in_1478, &occurrences, 4, 8),
            'c',
        );
        // 'd' appears 2x in 1478, 7x overall
        letter_decoder.insert(
            Self::single_char_with_occurrences(&occurrences_in_1478, &occurrences, 2, 7),
            'd',
        );
        // 'g' appears 1x in 1478, 7x overall
        letter_decoder.insert(
            Self::single_char_with_occurrences(&occurrences_in_1478, &occurrences, 1, 7),
            'g',
        );

        // and that leaves...
        letter_decoder.insert(
            *occurrences
                .keys()
                .find(|&shown_char| !letter_decoder.contains_key(shown_char))
                .unwrap(),
            'a',
        );

        Decoder { letter_decoder }
    }

    fn single_char_with_occurrences(
        occurrences_in_1478: &HashMap<char, u32>,
        occurrences: &HashMap<char, u32>,
        in_1478: u32,
        in_all: u32,
    ) -> char {
        let mut possible: HashSet<char> = occurrences_in_1478
            .iter()
            .filter(|(_, times)| **times == in_1478)
            .map(|(fake_ch, _)| *fake_ch)
            .collect();

        possible.retain(|ch| occurrences.get(ch) == Some(&in_all));

        match *possible.into_iter().collect::<Vec<char>>().as_slice() {
            [single] => single,
            _ => panic!("Expected single result for {} (occurrences in 1, 4, 7, 8), {} (occurrences in all)", in_1478, in_all),
        }
    }
}

impl Decoder {
    fn decode(&self, output_shown: [&str; 4]) -> u32 {
        let mut decoded_strings: Vec<String> = Vec::new();

        for fake_output in output_shown {
            let decoded = fake_output
                .chars()
                .map(|ch| self.letter_decoder.get(&ch).unwrap())
                .sorted()
                .into_iter()
                .join("");
            decoded_strings.push(decoded);
        }

        let number_str = decoded_strings
            .iter()
            .map(|s| match s.as_str() {
                "abcefg" => 0,
                "cf" => 1,
                "acdeg" => 2,
                "acdfg" => 3,
                "bcdf" => 4,
                "abdfg" => 5,
                "abdefg" => 6,
                "acf" => 7,
                "abcdefg" => 8,
                "abcdfg" => 9,
                _ => panic!("Bad string to decode: {}", s),
            })
            .join("");

        number_str.parse::<u32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::Day08;
    use crate::Solution;

    const SAMPLE_1: &'static str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_sample_part1() {
        assert_eq!(26.to_string(), Day08 {}.part1(SAMPLE_1))
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(61229.to_string(), Day08 {}.part2(SAMPLE_1));
    }
}
