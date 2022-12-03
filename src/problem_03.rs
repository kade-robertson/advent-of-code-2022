use std::collections::HashSet;

use crate::problem::Problem;

pub struct Problem03 {}

struct Rucksack {
    first: HashSet<u8>,
    second: HashSet<u8>,
    combined: HashSet<u8>,
}

impl Problem03 {
    pub fn new() -> Problem03 {
        Problem03 {}
    }

    fn char_to_priority(&self, c: char) -> u8 {
        if c.is_uppercase() {
            // A-Z starts at index 65, so for the range to be from 27-52 we
            // subtract 38.
            (c as u8) - 38
        } else {
            // A-Z starts at index 97, so we subtract 96.
            (c as u8) - 96
        }
    }

    fn parse(&self, data: &str) -> Vec<Rucksack> {
        let mut all_rucksacks = Vec::new();

        for line in data.lines() {
            let stripped_line = line.trim();
            let halfway = stripped_line.len() / 2;
            all_rucksacks.push(Rucksack {
                first: HashSet::from_iter(
                    stripped_line
                        .chars()
                        .take(halfway)
                        .map(|c| self.char_to_priority(c)),
                ),
                second: HashSet::from_iter(
                    stripped_line
                        .chars()
                        .skip(halfway)
                        .map(|c| self.char_to_priority(c)),
                ),
                combined: HashSet::from_iter(
                    stripped_line.chars().map(|c| self.char_to_priority(c)),
                ),
            })
        }

        all_rucksacks
    }

    fn solve_actual(&self, rucksacks: &[Rucksack]) -> u16 {
        rucksacks
            .iter()
            .map(|r| *r.first.intersection(&r.second).next().unwrap() as u16)
            .sum()
    }

    fn solve_actual_part2(&self, rucksacks: &[Rucksack]) -> u16 {
        rucksacks
            .chunks(3)
            .map(|rs| {
                *rs.iter()
                    .skip(1)
                    .fold(rs[0].combined.clone(), |acc, r| {
                        acc.intersection(&r.combined).cloned().collect()
                    })
                    .iter()
                    .next()
                    .unwrap() as u16
            })
            .sum()
    }
}

impl Problem for Problem03 {
    fn name(&self) -> &str {
        "Day 3: Rucksack Reorganization"
    }

    fn solve(&self) -> i64 {
        let data = get_input!("./inputs/problem_03.txt");
        let rucksacks = self.parse(&data);
        self.solve_actual(&rucksacks).into()
    }

    fn solve_part2(&self) -> i64 {
        let data = get_input!("./inputs/problem_03.txt");
        let rucksacks = self.parse(&data);
        self.solve_actual_part2(&rucksacks).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem03::new();
        let data = problem.parse("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(problem.solve_actual(&data), 157);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem03::new();
        let data = problem.parse("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw");

        assert_eq!(problem.solve_actual_part2(&data), 70);
    }
}
