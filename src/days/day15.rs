use crate::Solution;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub struct Day15 {}

impl Solution for Day15 {
    fn part1(&self, input: &str) -> String {
        let grid: Grid<usize> = parse_input(input);
        let goal_point = (grid.width - 1, grid.height - 1);

        lowest_risk_to_goal(&grid, START_POINT, goal_point)
            .map(|it| it.to_string())
            .unwrap_or_else(|| "No solution".to_string())
    }

    fn part2(&self, input: &str) -> String {
        let prototype_grid: Grid<usize> = parse_input(input);

        let mut full_values: Vec<usize> = Vec::new();
        for y in 0..(prototype_grid.height * 5) {
            for x in 0..(prototype_grid.width * 5) {
                let prototype_value = *prototype_grid
                    .value_at(x % prototype_grid.width, y % prototype_grid.height)
                    .unwrap();
                let new_value = {
                    let tmp =
                        prototype_value + (x / prototype_grid.width) + (y / prototype_grid.width);
                    if tmp >= 10 {
                        tmp % 10 + 1
                    } else {
                        tmp
                    }
                };
                full_values.push(new_value)
            }
        }

        let grid = Grid {
            height: prototype_grid.height * 5,
            width: prototype_grid.width * 5,
            values: full_values,
        };
        let goal_point = (grid.width - 1, grid.height - 1);

        lowest_risk_to_goal(&grid, START_POINT, goal_point)
            .map(|it| it.to_string())
            .unwrap_or_else(|| "No solution".to_string())
    }
}

fn parse_input(input: &str) -> Grid<usize> {
    Grid::from(input.trim(), |ch| ch.to_digit(10).unwrap() as usize)
}

type Point = (usize, usize);
const START_POINT: Point = (0, 0);

/// https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
fn lowest_risk_to_goal(g: &Grid<usize>, start: Point, goal: Point) -> Option<i32> {
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut total_risk: HashMap<Point, i32> = HashMap::new();

    total_risk.insert(start, 0);
    heap.push(State {
        risk: 0,
        point: start,
    });

    let lookup_risk =
        |point: &Point, risk_map: &HashMap<Point, i32>| *risk_map.get(point).unwrap_or(&i32::MAX);

    while let Some(State { risk, point }) = heap.pop() {
        if point == goal {
            return Some(risk);
        } else if risk > lookup_risk(&point, &total_risk) {
            continue;
        } else {
            let (x, y) = point;
            for (nx, ny) in g.adjacent_cells(x, y) {
                let neighbor_point = (nx, ny);

                let risk_through_here = risk + *g.value_at(nx, ny).unwrap() as i32;
                let existing_risk = lookup_risk(&neighbor_point, &total_risk);

                if risk_through_here < existing_risk {
                    let next_state = State {
                        risk: risk_through_here,
                        point: neighbor_point,
                    };
                    heap.push(next_state);
                    total_risk.insert(neighbor_point, next_state.risk);
                }
            }
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    risk: i32,
    point: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Grid<T> {
    values: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn from(str: &str, char_to_t: fn(char) -> T) -> Grid<T> {
        let trimmed = str.trim();
        let width = trimmed.lines().next().unwrap().len();
        let height = trimmed.lines().count();

        let values = trimmed
            .lines()
            .flat_map(|line| line.chars())
            .map(char_to_t)
            .collect();
        Grid {
            values,
            width,
            height,
        }
    }

    fn value_at(&self, x: usize, y: usize) -> Option<&T> {
        let index: usize = x + y * self.width;
        self.values.get(index)
    }

    fn adjacent_cells(&self, x: usize, y: usize) -> Vec<Point> {
        fn value_at_internal<T>(
            width: usize,
            height: usize,
            values: &[T],
            x: isize,
            y: isize,
        ) -> Option<&T> {
            if !(0..width).contains(&usize::try_from(x).unwrap_or(width + 1))
                || !(0..height).contains(&usize::try_from(y).unwrap_or(height + 1))
            {
                None
            } else {
                let index: usize = (x + y * width as isize) as usize;
                values.get(index)
            }
        }

        let xi = isize::try_from(x).unwrap();
        let yi = isize::try_from(y).unwrap();
        let possible_adj_cells: Vec<(isize, isize)> =
            vec![(xi - 1, yi), (xi + 1, yi), (xi, yi + 1), (xi, yi - 1)];

        possible_adj_cells
            .iter()
            .filter_map(|(x1, y1)| {
                value_at_internal(self.width, self.height, &self.values, *x1, *y1)
                    .map(|_| (*x1, *y1))
            })
            .map(|(x1, y1)| (usize::try_from(x1).unwrap(), usize::try_from(y1).unwrap()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day15::Day15;
    use crate::Solution;

    const SAMPLE_1: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    const SAMPLE_1_FULL: &'static str = "11637517422274862853338597396444961841755517295286
13813736722492484783351359589446246169155735727126
21365113283247622439435873354154698446526571955763
36949315694715142671582625378269373648937148475914
74634171118574528222968563933317967414442817852555
13191281372421239248353234135946434524615754563572
13599124212461123532357223464346833457545794456865
31254216394236532741534764385264587549637569865174
12931385212314249632342535174345364628545647573965
23119445813422155692453326671356443778246755488935
22748628533385973964449618417555172952866628316397
24924847833513595894462461691557357271266846838237
32476224394358733541546984465265719557637682166874
47151426715826253782693736489371484759148259586125
85745282229685639333179674144428178525553928963666
24212392483532341359464345246157545635726865674683
24611235323572234643468334575457944568656815567976
42365327415347643852645875496375698651748671976285
23142496323425351743453646285456475739656758684176
34221556924533266713564437782467554889357866599146
33859739644496184175551729528666283163977739427418
35135958944624616915573572712668468382377957949348
43587335415469844652657195576376821668748793277985
58262537826937364893714847591482595861259361697236
96856393331796741444281785255539289636664139174777
35323413594643452461575456357268656746837976785794
35722346434683345754579445686568155679767926678187
53476438526458754963756986517486719762859782187396
34253517434536462854564757396567586841767869795287
45332667135644377824675548893578665991468977611257
44961841755517295286662831639777394274188841538529
46246169155735727126684683823779579493488168151459
54698446526571955763768216687487932779859814388196
69373648937148475914825958612593616972361472718347
17967414442817852555392896366641391747775241285888
46434524615754563572686567468379767857948187896815
46833457545794456865681556797679266781878137789298
64587549637569865174867197628597821873961893298417
45364628545647573965675868417678697952878971816398
56443778246755488935786659914689776112579188722368
55172952866628316397773942741888415385299952649631
57357271266846838237795794934881681514599279262561
65719557637682166874879327798598143881961925499217
71484759148259586125936169723614727183472583829458
28178525553928963666413917477752412858886352396999
57545635726865674683797678579481878968159298917926
57944568656815567976792667818781377892989248891319
75698651748671976285978218739618932984172914319528
56475739656758684176786979528789718163989182927419
67554889357866599146897761125791887223681299833479";

    #[test]
    fn test_sample1_part1() {
        assert_eq!(40.to_string(), Day15 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample1_full_part1() {
        assert_eq!(315.to_string(), Day15 {}.part1(SAMPLE_1_FULL))
    }

    #[test]
    fn test_sample1_part2() {
        assert_eq!(315.to_string(), Day15 {}.part2(SAMPLE_1))
    }
}
