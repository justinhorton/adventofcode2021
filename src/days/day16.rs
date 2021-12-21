use crate::Solution;
use itertools::Itertools;
use log::debug;
use std::cmp::Ordering;

pub struct Day16 {}

impl Solution for Day16 {
    fn part1(&self, input: &str) -> String {
        let bin = parse_input_as_bin_str(input);

        let mut pos = 0;
        let mut version_sum = 0;
        while let Some((version, next_pos)) = read_version_shallow(&bin, pos) {
            version_sum += version;
            pos = next_pos;
        }

        version_sum.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let bin = parse_input_as_bin_str(input);
        let (result, _pos) = eval_packet(&bin, 0).expect("Bad packet");
        result.to_string()
    }
}

fn parse_input_as_bin_str(input: &str) -> Vec<char> {
    input
        .trim()
        .chars()
        .map(|ch| u32::from_str_radix(&*format!("{}", ch), 16).unwrap())
        .map(|it| format!("{:01$b}", it, 4))
        .join("")
        .chars()
        .collect_vec()
}

// a LiteralValue packet w/ value encoded in 5 bits
const MIN_PACKET_BITS: usize = 11;

fn read_version_shallow(bin: &[char], start_pos: usize) -> Option<(usize, usize)> {
    if start_pos + MIN_PACKET_BITS > bin.len() {
        return None;
    }

    let mut pos = start_pos;

    let version = consume_bits_as_usize(bin, &mut pos, 3);
    let type_id = consume_bits_as_usize(bin, &mut pos, 3);

    match type_id {
        4 => {
            // literal value
            let mut literal = String::from("");

            let mut group = consume_bits_as_char_slice(bin, &mut pos, 5);
            literal.push_str(&*group[1..].iter().join(""));

            while group[0] == '1' {
                group = consume_bits_as_char_slice(bin, &mut pos, 5);
                literal.push_str(&*group[1..].iter().join(""));
            }
        }
        _ => {
            // operator
            let length_type_id = consume_bits_as_char_slice(bin, &mut pos, 1);
            match *length_type_id {
                ['0'] => {
                    consume_bits_as_usize(bin, &mut pos, 15);
                }
                _ => {
                    consume_bits_as_usize(bin, &mut pos, 11);
                }
            }
        }
    }

    Some((version, pos))
}

fn eval_packet(bin: &[char], start_pos: usize) -> Option<(usize, usize)> {
    if start_pos + MIN_PACKET_BITS > bin.len() {
        debug!(
            "No packets remaining at pos={}, len={}",
            start_pos,
            bin.len()
        );
        return None;
    }

    let mut pos = start_pos;

    let version = consume_bits_as_usize(bin, &mut pos, 3);
    let type_id = consume_bits_as_usize(bin, &mut pos, 3);
    debug!("Packet v{} ({})", version, type_id);

    let result: usize = match type_id {
        4 => {
            // literal value
            let mut literal = String::from("");

            let mut group = consume_bits_as_char_slice(bin, &mut pos, 5);
            literal.push_str(&*group[1..].iter().join(""));

            while group[0] == '1' {
                group = consume_bits_as_char_slice(bin, &mut pos, 5);
                literal.push_str(&*group[1..].iter().join(""));
            }

            let r = usize::from_str_radix(&*literal, 2).unwrap();
            debug!("Literal {}", r);
            r
        }
        _ => {
            // operator
            let length_type_id = consume_bits_as_char_slice(bin, &mut pos, 1);
            let mut sub_packet_results: Vec<usize> = Vec::new();
            match *length_type_id {
                ['0'] => {
                    let num_sub_bits = consume_bits_as_usize(bin, &mut pos, 15);
                    debug!("Operator ({} sub-bits)", num_sub_bits);

                    let end = pos + num_sub_bits;
                    while pos < end {
                        let eval = eval_packet(&bin[pos..], 0);
                        sub_packet_results.push(eval.unwrap().0);
                        pos += eval.unwrap().1;
                    }
                }
                _ => {
                    let num_sub_packets = consume_bits_as_usize(bin, &mut pos, 11);
                    debug!("Operator ({} sub-packets)", num_sub_packets);

                    for _p in 0..num_sub_packets {
                        let eval = eval_packet(&bin[pos..], 0);
                        sub_packet_results.push(eval.unwrap().0);
                        pos += eval.unwrap().1;
                    }
                }
            }

            match type_id {
                0 => {
                    let r = sub_packet_results.iter().sum();
                    debug!("SUM({:?}) = {}", sub_packet_results, r);
                    r
                }
                1 => {
                    let r = sub_packet_results.iter().product();
                    debug!("PRODUCT({:?}) = {}", sub_packet_results, r);
                    r
                }
                2 => {
                    let r = *sub_packet_results.iter().min().unwrap();
                    debug!("MIN({:?}) = {}", sub_packet_results, r);
                    r
                }
                3 => {
                    let r = *sub_packet_results.iter().max().unwrap();
                    debug!("MAX({:?}) = {}", sub_packet_results, r);
                    r
                }
                5..=MIN_PACKET_BITS => {
                    // >, <, =
                    let (left, right) = match sub_packet_results.as_slice() {
                        [first, second] => (first, second),
                        _ => panic!("Expected exactly 2 sub-packets for comparison"),
                    };
                    let ordering = match type_id {
                        5 => Ordering::Greater,
                        6 => Ordering::Less,
                        7 => Ordering::Equal,
                        _ => panic!("Unreachable"),
                    };

                    let r = if left.cmp(right) == ordering { 1 } else { 0 };
                    debug!("{} {:?} {}={}", left, ordering, right, r);
                    r
                }
                x => panic!("Unknown operator type: {}", x),
            }
        }
    };

    Some((result, pos))
}

fn consume_bits_as_usize(bin: &[char], start_pos: &mut usize, num_bits: usize) -> usize {
    usize_from_slice(consume_bits_as_char_slice(bin, start_pos, num_bits))
}

fn usize_from_slice(char_slice: &[char]) -> usize {
    usize::from_str_radix(&*String::from_iter(char_slice.iter()), 2).unwrap()
}

fn consume_bits_as_char_slice<'a>(
    bin: &'a [char],
    start_pos: &mut usize,
    num_bits: usize,
) -> &'a [char] {
    let result = &bin[*start_pos..(*start_pos + num_bits)];
    *start_pos += num_bits;
    result
}

#[cfg(test)]
mod tests {
    use crate::days::day16::Day16;
    use crate::Solution;

    #[test]
    fn sample1_part1() {
        assert_eq!(6.to_string(), Day16 {}.part1("D2FE28"));
    }

    #[test]
    fn operator_packet_part1() {
        assert_eq!(9.to_string(), Day16 {}.part1("38006F45291200"))
    }

    #[test]
    fn operator_packet_add() {
        // 1 + 2
        assert_eq!(14.to_string(), Day16 {}.part1("C200B40A82"));
        assert_eq!(3.to_string(), Day16 {}.part2("C200B40A82"))
    }

    #[test]
    fn operator_packet_mul() {
        // 6 * 9
        assert_eq!(54.to_string(), Day16 {}.part2("04005AC33890"));
    }

    #[test]
    fn min() {
        // min(7, 8, 9)
        assert_eq!(7.to_string(), Day16 {}.part2("880086C3E88112"));
    }

    #[test]
    fn max() {
        // max(7, 8, 9)
        assert_eq!(9.to_string(), Day16 {}.part2("CE00C43D881120"));
    }

    #[test]
    fn less_than() {
        // 5 < 15
        assert_eq!(1.to_string(), Day16 {}.part2("D8005AC2A8F0"));
    }

    #[test]
    fn not_greater_than() {
        // 5 > 15
        assert_eq!(0.to_string(), Day16 {}.part2("F600BC2D8F"));
    }

    #[test]
    fn not_equal() {
        // 5 == 15
        assert_eq!(0.to_string(), Day16 {}.part2("9C005AC2F8F0"));
    }

    #[test]
    fn composite_expr() {
        // 1 + 3 = 2 * 2
        assert_eq!(1.to_string(), Day16 {}.part2("9C0141080250320F1802104A08"));
    }

    #[test]
    fn more_samples_part1() {
        assert_eq!(16.to_string(), Day16 {}.part1("8A004A801A8002F478"));
        assert_eq!(12.to_string(), Day16 {}.part1("620080001611562C8802118E34"));
        assert_eq!(
            23.to_string(),
            Day16 {}.part1("C0015000016115A2E0802F182340")
        );
        assert_eq!(
            31.to_string(),
            Day16 {}.part1("A0016C880162017C3686B18A3D4780")
        );
    }
}
