use crate::days::template::Solution;
use itertools::Itertools;

pub struct Day04 {}

impl Solution for Day04 {
    fn part1(&self, input: &str) -> String {
        let (num_seq, mut boards) = parse_input(input);

        for num in &num_seq {
            boards.iter_mut().for_each(|b| b.mark_value(num));
            if let Some(winner) = boards.iter().find(|b| b.is_winner()) {
                return (winner.sum_unmarked() * num).to_string();
            }
        }

        String::from("No solution")
    }

    fn part2(&self, input: &str) -> String {
        let (num_seq, mut boards) = parse_input(input);

        for num in &num_seq {
            for b in &mut boards {
                b.mark_value(num);
            }

            let maybe_last_winner = boards.first().cloned();
            boards.retain(|b| !b.is_winner());

            if let Some(last_winner) = maybe_last_winner.filter(|_| boards.is_empty()) {
                return (last_winner.sum_unmarked() * num).to_string();
            }
        }

        String::from("No solution")
    }
}

const BOARD_SIZE: usize = 5;

fn parse_input(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let mut lines = input.trim().lines();
    let num_seq: Vec<i32> = lines
        .next()
        .map(|line| line.split(',').map(|num| num.parse::<i32>().unwrap()))
        .unwrap()
        .collect();

    let board_inputs = lines
        .map(|it| it.trim())
        .filter(|it| !it.is_empty())
        .chunks(BOARD_SIZE);

    let mut grids: Vec<BingoBoard> = Vec::new();
    for chunk in &board_inputs {
        let gridx: Vec<i32> = chunk
            .flat_map(|row_str| row_str.split_whitespace())
            .map(|it| it.parse::<i32>().unwrap())
            .collect();
        grids.push(BingoBoard::new(BOARD_SIZE, gridx));
    }

    (num_seq, grids)
}

#[derive(Debug, Clone)]
struct BingoBoard {
    grid: Vec<(bool, i32)>,
    size: usize,
}

impl BingoBoard {
    fn new(size: usize, nums: Vec<i32>) -> BingoBoard {
        BingoBoard {
            grid: nums.iter().map(|&it| (false, it)).collect(),
            size,
        }
    }

    fn mark_value(&mut self, value: &i32) {
        let found: Option<(usize, &(bool, i32))> =
            self.grid.iter().find_position(|(_, v)| *v == *value);
        if let Some((found_index, &el)) = found {
            self.grid[found_index] = (true, el.1);
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.grid
            .iter()
            .map(|(marked, value)| if !marked { *value } else { 0 })
            .sum()
    }

    fn is_winner(&self) -> bool {
        let flattened_marks: Vec<bool> = self.grid.iter().map(|(marked, _)| *marked).collect();
        let row_win = (0..self.size).any(|row_i| {
            flattened_marks
                .iter()
                .skip(row_i * self.size)
                .take(self.size)
                .all(|it| *it)
        });
        row_win
            || (0..self.size).any(|col_i| {
                flattened_marks
                    .iter()
                    .skip(col_i)
                    .step(self.size)
                    .all(|it| *it)
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day04::Day04;
    use crate::Solution;

    const SAMPLE_1: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_sample_part1() {
        assert_eq!(4512.to_string(), Day04 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(1924.to_string(), Day04 {}.part2(SAMPLE_1));
    }
}
