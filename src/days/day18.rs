use crate::Solution;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub struct Day18 {}

impl Solution for Day18 {
    fn part1(&self, input: &str) -> String {
        let pairs = input.trim().lines().map(parse_pair).collect_vec();

        let reduced: Node = pairs
            .iter()
            .skip(1)
            .fold(pairs[0].clone(), |n1, n2| pairwise_add(&n1, n2));

        reduced.calc_magnitude().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let pairs = input.trim().lines().map(parse_pair).collect_vec();

        pairs
            .iter()
            .permutations(2)
            .map(|perm| pairwise_add(perm[0], perm[1]).calc_magnitude())
            .max()
            .unwrap()
            .to_string()
    }
}

fn pairwise_add(n1: &Node, n2: &Node) -> Node {
    let mut reduced = Node::Pair {
        left: Box::new(n1.clone()),
        right: Box::new(n2.clone()),
    };

    while reduced
        .explode(0)
        .map(|_| ())
        .or_else(|| reduced.split(0))
        .is_some()
    {
        // keep reducing
    }

    reduced
}

fn parse_pair(input: &str) -> Node {
    if let Ok(n) = input.parse::<usize>() {
        // input part is a 'regular number'
        Node::Literal { value: n }
    } else {
        // find the top-level pair and recurse
        let (_, split_i) = input
            .chars()
            .enumerate()
            .fold_while((0, 0), |(depth, split), (i, ch)| match ch {
                '[' => Continue((depth + 1, split)),
                ']' => Continue((depth - 1, split)),
                ',' => {
                    if depth == 1 {
                        Done((1, i))
                    } else {
                        Continue((depth, split))
                    }
                }
                _ => Continue((depth, split)),
            })
            .into_inner();

        Node::Pair {
            // skip leading '['
            left: Box::new(parse_pair(&input[1..split_i])),
            // skip trailing ']'
            right: Box::new(parse_pair(&input[split_i + 1..input.len() - 1])),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Literal { value: usize },
    Pair { left: Box<Node>, right: Box<Node> },
}

type LeftValue = usize;
type RightValue = usize;
impl Node {
    fn calc_magnitude(&self) -> usize {
        match self {
            Node::Literal { value } => *value,
            Node::Pair { left, right } => left.calc_magnitude() * 3 + right.calc_magnitude() * 2,
        }
    }

    fn explode(&mut self, depth: usize) -> Option<(LeftValue, RightValue)> {
        return match self {
            Node::Literal { value: _ } => None,
            Node::Pair { left, right } => {
                if depth == 4 {
                    let left_val: LeftValue = match **left {
                        Node::Literal { value } => value,
                        _ => unreachable!("{:?}", **left),
                    };
                    let right_val: RightValue = match **right {
                        Node::Literal { value } => value,
                        _ => unreachable!("{:?}", **right),
                    };
                    *self = Node::Literal { value: 0 };
                    Some((left_val, right_val))
                } else if let Some((left_val, right_val)) = left.explode(depth + 1) {
                    right.propagate_explode(true, right_val);
                    Some((left_val, 0))
                } else if let Some((left_val, right_val)) = right.explode(depth + 1) {
                    left.propagate_explode(false, left_val);
                    Some((0, right_val))
                } else {
                    None
                }
            }
        };
    }

    fn propagate_explode(&mut self, reduce_left: bool, add: usize) {
        match self {
            Node::Literal { value } => {
                *value += add;
            }
            Node::Pair { left, right } => {
                if reduce_left {
                    left.propagate_explode(reduce_left, add);
                } else {
                    right.propagate_explode(reduce_left, add)
                }
            }
        }
    }

    fn split(&mut self, depth: usize) -> Option<()> {
        match self {
            Node::Literal { value } => {
                let v = *value;
                if v >= 10 {
                    *self = Node::Pair {
                        left: Box::new(Node::Literal { value: v / 2 }),
                        right: Box::new(Node::Literal { value: v - (v / 2) }),
                    };
                    Some(())
                } else {
                    None
                }
            }
            Node::Pair { left, right } => left
                .split(depth + 1)
                .or_else(|| right.split(depth + 1))
                .or(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day18::Day18;
    use crate::Solution;

    const SAMPLE_1: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn test_sample_pt1() {
        assert_eq!(4140.to_string(), Day18 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample_pt2() {
        assert_eq!(3993.to_string(), Day18 {}.part2(SAMPLE_1));
    }
}
