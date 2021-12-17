use crate::days::graph::Graph;
use crate::days::graph::NodeIndex;
use crate::Solution;
use std::collections::{HashMap, HashSet, LinkedList};

pub struct Day12 {}

const START_ID: &str = "start";
const END_ID: &str = "end";

impl Solution for Day12 {
    fn part1(&self, input: &str) -> String {
        let graph = Self::build_graph(input);

        let start = graph.node_index_by_id.get(START_ID).unwrap();
        let all_paths = &mut HashSet::new();

        find_paths(
            &graph,
            *start,
            &mut HashSet::new(),
            &LinkedList::new(),
            all_paths,
            &Part::One,
        );

        all_paths.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let graph = Self::build_graph(input);

        let start = graph.node_index_by_id.get(START_ID).unwrap();
        let all_paths = &mut HashSet::new();

        find_paths(
            &graph,
            *start,
            &mut HashSet::new(),
            &LinkedList::new(),
            all_paths,
            &Part::Two(&None),
        );

        all_paths.len().to_string()
    }
}

impl Day12 {
    fn build_graph(input: &str) -> Graph {
        let mut graph = Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_index_by_id: HashMap::new(),
        };

        for input_line in input.trim().lines() {
            let split: Vec<&str> = input_line.split('-').collect();
            let src = split.get(0).unwrap();
            let dst = split.get(1).unwrap();
            let src_n = graph.add_node(&src.to_string());
            let dst_n = graph.add_node(&dst.to_string());

            match (*src, *dst) {
                // start and end edges are unidirectional, all other edges bidirectional
                (START_ID, _) => {
                    graph.add_edge(src_n, dst_n);
                }
                (END_ID, _) => {
                    graph.add_edge(dst_n, src_n);
                }
                (_, START_ID) => {
                    graph.add_edge(dst_n, src_n);
                }
                (_, END_ID) => {
                    graph.add_edge(src_n, dst_n);
                }
                _ => {
                    graph.add_edge(src_n, dst_n);
                    graph.add_edge(dst_n, src_n);
                }
            };
        }
        graph
    }
}

enum Part<'a> {
    One,
    Two(&'a Option<&'a str>),
}

fn find_paths<'a>(
    graph: &'a Graph,
    cur_node: NodeIndex,
    small_seen: &mut HashSet<String>,
    cur_path: &LinkedList<&'a str>,
    paths: &mut HashSet<LinkedList<&'a str>>,
    part: &Part,
) {
    let cur_id = &graph.nodes[cur_node].id;

    if cur_id == END_ID {
        // found a whole path
        let mut new_path: LinkedList<&str> = cur_path.iter().copied().collect();
        new_path.push_back(END_ID);
        paths.insert(new_path);
    } else if cur_id == START_ID || !is_small_cave(cur_id) {
        for s in graph.successors(cur_node) {
            extending_path(cur_id, cur_path, |new_path| {
                find_paths(graph, s, small_seen, new_path, paths, part);
            });
        }
    } else if is_small_cave(cur_id) {
        let already_seen_small = small_seen.insert(cur_id.to_string());
        if already_seen_small {
            for s in graph.successors(cur_node) {
                extending_path(cur_id, cur_path, |new_path| {
                    find_paths(graph, s, small_seen, new_path, paths, part);
                });
            }

            small_seen.remove(cur_id);
        } else if let Part::Two(None) = part {
            for s in graph.successors(cur_node) {
                extending_path(cur_id, cur_path, |new_path| {
                    find_paths(
                        graph,
                        s,
                        small_seen,
                        new_path,
                        paths,
                        &Part::Two(&Some(cur_id)),
                    );
                });
            }
        }
    }
}

fn extending_path<'a>(
    cur_id: &'a str,
    cur_path: &LinkedList<&'a str>,
    mut action: impl FnMut(&LinkedList<&'a str>),
) {
    let mut new_path: LinkedList<&str> = cur_path.iter().copied().collect();
    new_path.push_back(cur_id);
    action(&new_path);
}

fn is_small_cave(id: &str) -> bool {
    id.to_ascii_lowercase() == id
}

#[cfg(test)]
mod tests {
    use crate::days::day12::Day12;
    use crate::Solution;

    const SAMPLE_1: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const SAMPLE_2: &'static str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const SAMPLE_3: &'static str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_sample1_part1() {
        assert_eq!(10.to_string(), Day12 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample2_part1() {
        assert_eq!(19.to_string(), Day12 {}.part1(SAMPLE_2));
    }

    #[test]
    fn test_sample3_part1() {
        assert_eq!(226.to_string(), Day12 {}.part1(SAMPLE_3));
    }

    #[test]
    fn test_sample1_part2() {
        assert_eq!(36.to_string(), Day12 {}.part2(SAMPLE_1));
    }

    #[test]
    fn test_sample2_part2() {
        assert_eq!(103.to_string(), Day12 {}.part2(SAMPLE_2));
    }

    #[test]
    fn test_sample3_part2() {
        assert_eq!(3509.to_string(), Day12 {}.part2(SAMPLE_3));
    }
}
