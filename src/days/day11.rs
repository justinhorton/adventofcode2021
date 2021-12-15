use crate::days::template::Solution;
use itertools::Itertools;
use std::cmp::min;
use std::collections::HashSet;
use std::fmt;

pub struct Day11 {}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        let mut octopus_grid = Self::parse_input(input);
        octopus_grid.do_octopus_things(100).to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut octopus_grid = Self::parse_input(input);
        octopus_grid.trick_the_octopuses().to_string()
    }
}

impl Day11 {
    fn parse_input(input: &str) -> FlashingOctopusGrid {
        FlashingOctopusGrid {
            grid: Grid::from(input, |ch| ch.to_digit(10).unwrap()),
        }
    }
}

struct FlashingOctopusGrid {
    grid: Grid<u32>,
}

impl fmt::Debug for FlashingOctopusGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let chunks = self.grid.values.chunks(self.grid.width);
        writeln!(f, "{}", chunks.map(|c| c.iter().join(" ")).join("\n"))
    }
}

/*
First, the energy level of each octopus increases by 1.

Then, any octopus with an energy level greater than 9 flashes. This increases the energy level of
all adjacent octopuses by 1, including octopuses that are diagonally adjacent. If this causes an
octopus to have an energy level greater than 9, it also flashes. This process continues as long as
new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most
once per step.)

Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of
its energy to flash.
 */
impl FlashingOctopusGrid {
    fn do_octopus_things(&mut self, steps: usize) -> usize {
        let mut total_flashes = 0;
        for _i in 1..=steps {
            self.increment_all_energies();
            total_flashes += self.flash();
        }
        total_flashes
    }

    fn trick_the_octopuses(&mut self) -> usize {
        let mut i = 0;
        loop {
            i += 1;
            self.increment_all_energies();
            let flashes = self.flash();
            if flashes == self.grid.values.len() {
                return i;
            }
        }
    }

    fn increment_all_energies(&mut self) {
        self.grid.values.iter_mut().for_each(|v| *v += 1)
    }

    fn increment_energy(&mut self, index: usize) {
        let v = self.grid.values.get_mut(index).unwrap();
        *v = min(*v + 1, 10)
    }

    fn flash(&mut self) -> usize {
        // gather indices of initial flashes
        let mut all_flashed: HashSet<usize> = self
            .grid
            .values
            .iter()
            .enumerate()
            .filter(|(_, v)| **v > 9)
            .map(|(i, _)| i)
            .collect();

        let just_flashed: HashSet<usize> = all_flashed.iter().copied().collect();
        self.apply_flashes(&mut all_flashed, &just_flashed);

        for flashed_index in &all_flashed {
            let energy = self.grid.values.get_mut(*flashed_index).unwrap();
            *energy = 0;
        }

        all_flashed.len()
    }

    fn apply_flashes(&mut self, flashed: &mut HashSet<usize>, flashed_prev: &HashSet<usize>) {
        let mut flashed_cur: HashSet<usize> = HashSet::new();

        for flashed_index in flashed_prev {
            for adjacent_index in self.grid.adjacent_indices(flashed_index) {
                if !flashed.contains(&adjacent_index) {
                    self.increment_energy(adjacent_index);

                    let energy = self.grid.values.get(adjacent_index).unwrap();
                    if *energy > 9 {
                        flashed_cur.insert(adjacent_index);
                    }
                }
            }
        }

        if !flashed_cur.is_empty() {
            flashed.extend(flashed_cur.iter());
            self.apply_flashes(flashed, &flashed_cur);
        }
    }
}

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

    fn value_at(&self, x: isize, y: isize) -> Option<&T> {
        if !(0..self.width).contains(&usize::try_from(x).unwrap_or(self.width + 1))
            || !(0..self.height).contains(&usize::try_from(y).unwrap_or(self.height + 1))
        {
            None
        } else {
            let index: usize = (x + y * self.width as isize) as usize;
            self.values.get(index)
        }
    }

    fn index_to_xy(&self, index: &usize) -> Option<(usize, usize)> {
        if *index >= self.values.len() {
            return None;
        }
        let y = index / self.width;
        let x = index % self.width;
        Some((x, y))
    }

    fn xy_to_index(&self, x: isize, y: isize) -> usize {
        let potential = y * (self.width as isize) + x;
        usize::try_from(potential).unwrap()
    }

    fn adjacent_indices(&self, index: &usize) -> Vec<usize> {
        let (xu, yu) = self.index_to_xy(index).unwrap();
        let (xi, yi) = (xu as isize, yu as isize);
        let adj_coords = vec![
            (xi - 1, yi),
            (xi - 1, yi - 1),
            (xi - 1, yi + 1),
            (xi + 1, yi),
            (xi + 1, yi - 1),
            (xi + 1, yi + 1),
            (xi, yi + 1),
            (xi, yi - 1),
        ];

        adj_coords
            .iter()
            .filter(|(x1, y1)| self.value_at(*x1, *y1).is_some())
            .map(|(x1, y1)| self.xy_to_index(*x1, *y1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day11::Day11;
    use crate::Solution;

    const SAMPLE_1: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_sample_part1() {
        assert_eq!(1656.to_string(), Day11 {}.part1(SAMPLE_1));
    }

    #[test]
    fn test_sample_part2() {
        assert_eq!(195.to_string(), Day11 {}.part2(SAMPLE_1));
    }
}
