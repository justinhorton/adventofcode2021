use crate::days::template::Solution;
use itertools::repeat_n;

pub struct Day06 {}

impl Solution for Day06 {
    fn part1(&self, input: &str) -> String {
        let mut fish = Self::parse_input(input);

        for _ in 1..=80 {
            // create new fishies
            let start_fish_this_gen = fish.len();
            let num_new = fish.iter_mut().filter(|f| **f == 0).count();
            repeat_n(8, num_new).for_each(|nf| fish.push(nf));

            // apply decrements
            fish.iter_mut()
                .take(start_fish_this_gen)
                .for_each(|f| *f = if *f == 0 { 6 } else { *f - 1 });
        }

        fish.len().to_string()
    }

    /*
    Diff 0: 0
    Diff 1: 0
    Diff 2: 116
    Diff 3: 45
    Diff 4: 42
    Diff 5: 48
    Diff 6: 49
    Diff 7: 0
    Diff 8: 0
    Diff 9: 116 // day 2 + day 0
    Diff 10: 45 // day 3 + day 1
    Diff 11: 158 // day 4 + day 2
    Diff 12: 93 // day 5 + day 3
    Diff 13: 91 // day 6 + day 4
    Diff 14: 48 // day 7 + day 5
    ...
    */
    fn part2(&self, input: &str) -> String {
        let mut fish = Self::parse_input(input);
        let original_fish_count = fish.len();

        // no new fish on day 0 (initial)
        let mut new_fish_counts: [usize; 257] = [0; 257];

        // simulate normally through day 8
        for fish_count in new_fish_counts.iter_mut().take(9).skip(1) {
            // create new fishies
            let start_fish_this_gen = fish.len();
            let num_new = fish.iter_mut().filter(|f| **f == 0).count();
            repeat_n(8, num_new).for_each(|nf| fish.push(nf));

            // apply decrements
            fish.iter_mut()
                .take(start_fish_this_gen)
                .for_each(|f| *f = if *f == 0 { 6 } else { *f - 1 });

            *fish_count = num_new;
        }

        // diff(d) = diff(d - 9) + diff(d - 7)
        for d in 9..=256 {
            new_fish_counts[d] = new_fish_counts[d - 9] + new_fish_counts[d - 7];
        }

        let result = new_fish_counts.iter().sum::<usize>() + original_fish_count;
        result.to_string()
    }
}

impl Day06 {
    fn parse_input(input: &str) -> Vec<i8> {
        input
            .trim()
            .split(',')
            .map(|it| it.parse::<i8>().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day06::Day06;
    use crate::Solution;

    const SAMPLE_1: &'static str = "3,4,3,1,2";

    #[test]
    fn test_sample_part1() {
        assert_eq!(5934.to_string(), Day06 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(26984457539usize.to_string(), Day06 {}.part2(SAMPLE_1));
    }
}
