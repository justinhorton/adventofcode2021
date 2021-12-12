use crate::days::template::Solution;
use std::collections::LinkedList;

pub struct Day10 {}

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        let parsed_input = Self::parse_input(input);

        let mut syntax_score = 0;
        for line in parsed_input {
            let mut stack = LinkedList::new();
            line.chars().for_each(|ch| {
                let expected_open: Option<char> = Self::open_for(ch);

                if expected_open.is_some() {
                    let actual_open = stack.pop_back();
                    if expected_open != actual_open {
                        syntax_score += match ch {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => 0,
                        };
                    }
                } else {
                    stack.push_back(ch);
                }
            })
        }

        syntax_score.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let parsed_input = Self::parse_input(input);

        let mut all_scores: Vec<usize> = Vec::new();
        for line in parsed_input
            .iter()
            .filter(|&&l| self.part1(l) == 0.to_string())
        {
            let mut stack = LinkedList::new();

            for ch in line.chars() {
                if "[{(<".contains(ch) {
                    // open
                    stack.push_back(ch)
                } else if Self::open_for(ch) == stack.back().copied() {
                    // matching close
                    stack.pop_back();
                } else {
                    // incomplete
                    break;
                }
            }

            let score = stack
                .iter()
                .rev()
                .map(|&ch| Self::close_for(ch).unwrap())
                .fold(0, |acc, ch| acc * 5 + ")]}>".find(ch).unwrap() + 1);
            all_scores.push(score);
        }

        all_scores.sort_unstable();
        all_scores.get(all_scores.len() / 2).unwrap().to_string()
    }
}

impl Day10 {
    fn parse_input(input: &str) -> Vec<&str> {
        input.trim().lines().collect()
    }

    fn open_for(ch: char) -> Option<char> {
        ")]}>"
            .find(ch)
            .map(|index| "([{<".as_bytes()[index] as char)
    }

    fn close_for(ch: char) -> Option<char> {
        "([{<"
            .find(ch)
            .map(|index| ")]}>".as_bytes()[index] as char)
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day10::Day10;
    use crate::Solution;

    const SAMPLE_1: &'static str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_sample_part1() {
        assert_eq!(26397.to_string(), Day10 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(288957.to_string(), Day10 {}.part2(SAMPLE_1));
    }
}
