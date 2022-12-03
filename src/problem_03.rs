use crate::problem::Problem;

pub struct Problem03 {}

#[derive(Debug)]
struct Rucksack {
    first: u64,
    second: u64,
    combined: u64,
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
            // a-z starts at index 97, so we subtract 96.
            (c as u8) - 96
        }
    }

    fn set_bit(&self, start: u64, priority: u8) -> u64 {
        let bit_to_set = 1 << priority;

        if start & bit_to_set == 0 {
            start + bit_to_set
        } else {
            start
        }
    }

    fn parse(&self, data: &str) -> Vec<Rucksack> {
        let mut all_rucksacks = Vec::new();

        for line in data.lines() {
            let stripped_line = line.trim();
            let halfway = stripped_line.len() / 2;
            all_rucksacks.push(Rucksack {
                first: stripped_line
                    .chars()
                    .take(halfway)
                    .fold(0, |acc, c| self.set_bit(acc, self.char_to_priority(c))),
                second: stripped_line
                    .chars()
                    .skip(halfway)
                    .fold(0, |acc, c| self.set_bit(acc, self.char_to_priority(c))),
                combined: stripped_line
                    .chars()
                    .fold(0, |acc, c| self.set_bit(acc, self.char_to_priority(c))),
            })
        }

        all_rucksacks
    }

    fn solve_actual(&self, rucksacks: &[Rucksack]) -> u32 {
        rucksacks
            .iter()
            .map(|r| (r.first & r.second).trailing_zeros())
            .sum()
    }

    fn solve_actual_part2(&self, rucksacks: &[Rucksack]) -> u32 {
        rucksacks
            .chunks(3)
            .map(|rs| {
                rs.iter()
                    .skip(1)
                    .fold(rs[0].combined, |acc, r| acc & r.combined)
                    .trailing_zeros()
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
