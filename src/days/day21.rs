use crate::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day21 {}

const PT1_SCORE_TO_WIN: usize = 1000;
const PT2_SCORE_TO_WIN: usize = 21;

impl Solution for Day21 {
    fn part1(&self, input: &str) -> String {
        let (mut player1, mut player2) = init_players(input);
        let mut die = SeqDie100 {
            cur_roll: 1,
            n_rolls: 0,
        };

        loop {
            player1.play_turn(&mut die);
            if player1.score >= PT1_SCORE_TO_WIN {
                return (player2.score * die.n_rolls).to_string();
            }
            std::mem::swap(&mut player1, &mut player2);
        }
    }

    fn part2(&self, input: &str) -> String {
        let (player1, player2) = init_players(input);
        let mut memo: HashMap<(Player, Player), (usize, usize)> = HashMap::new();
        let result = play_quantum(&mut memo, player1, player2);

        usize::max(result.0, result.1).to_string()
    }
}

fn init_players(input: &str) -> (Player, Player) {
    let lines = input.trim().lines().collect_vec();
    let parse_player_start: fn(&str) -> usize = |it| {
        it.split_once(": ")
            .map(|it| it.1.parse::<usize>().unwrap())
            .unwrap()
    };
    let (p1_start, p2_start) = match lines.as_slice() {
        [p1_str, p2_str] => (parse_player_start(p1_str), parse_player_start(p2_str)),
        _ => panic!("Invalid input"),
    };

    (
        Player {
            score: 0,
            pos: p1_start,
        },
        Player {
            score: 0,
            pos: p2_start,
        },
    )
}

fn play_quantum(
    memo: &mut HashMap<(Player, Player), (usize, usize)>,
    p1: Player,
    p2: Player,
) -> (usize, usize) {
    return if p2.score >= PT2_SCORE_TO_WIN {
        (0, 1)
    } else if let Some(score) = memo.get(&(p1, p2)) {
        *score
    } else {
        let (mut wins_p1, mut wins_p2) = (0, 0);
        for (roll_sum, occurrences) in rolls_to_occurrences() {
            let mut next_p1 = p1;
            next_p1.apply_roll_sum(roll_sum);

            let (wins_this_roll_p2, wins_this_roll_p1) = play_quantum(memo, p2, next_p1);
            wins_p1 += wins_this_roll_p1 * occurrences;
            wins_p2 += wins_this_roll_p2 * occurrences;
        }

        memo.insert((p1, p2), (wins_p1, wins_p2));
        (wins_p1, wins_p2)
    };
}

fn rolls_to_occurrences() -> Vec<(usize, usize)> {
    let mut possible_total_rolls: Vec<usize> = Vec::new();
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                possible_total_rolls.push(r1 + r2 + r3)
            }
        }
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for it in possible_total_rolls {
        *map.entry(it).or_default() += 1
    }
    map.iter().map(|it| (*it.0, *it.1)).collect_vec()
}

trait Die {
    fn do_roll(&mut self) -> usize;
}

struct SeqDie100 {
    cur_roll: usize,
    n_rolls: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Player {
    pos: usize,
    score: usize,
}

impl Player {
    fn play_turn(&mut self, die: &mut dyn Die) {
        let roll_sum = die.do_roll() + die.do_roll() + die.do_roll();
        self.apply_roll_sum(roll_sum)
    }

    fn apply_roll_sum(&mut self, roll_sum: usize) {
        let tmp = (roll_sum % 10) + self.pos;
        self.pos = if tmp <= 10 { tmp } else { tmp - 10 };
        self.score += self.pos;
    }
}

impl Die for SeqDie100 {
    fn do_roll(&mut self) -> usize {
        let r = self.cur_roll;
        self.cur_roll = if (r + 1) > 100 { 1 } else { r + 1 };
        self.n_rolls += 1;
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day21::Day21;
    use crate::Solution;

    const SAMPLE_1: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_sample1_part1() {
        assert_eq!(739785.to_string(), Day21 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample1_part2() {
        assert_eq!(444356092776315usize.to_string(), Day21 {}.part2(SAMPLE_1))
    }
}
