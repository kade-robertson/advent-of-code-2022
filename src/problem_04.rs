use crate::problem::Problem;

pub struct Problem04 {}

struct CleaningPair {
    first: (u8, u8),
    second: (u8, u8),
}

impl CleaningPair {
    pub fn self_contained(&self) -> bool {
        (self.first.0 <= self.second.0 && self.first.1 >= self.second.1)
            || (self.second.0 <= self.first.0 && self.second.1 >= self.first.1)
    }

    pub fn overlapping(&self) -> bool {
        let x = (self.second.0 <= self.first.0 && self.first.0 <= self.second.1)
            || (self.second.0 <= self.first.1 && self.first.1 <= self.second.1)
            || (self.first.0 <= self.second.0 && self.second.0 <= self.first.1)
            || (self.first.0 <= self.second.1 && self.second.1 <= self.first.1);

        x
    }
}

fn parse_number(iter: &mut core::str::Split<char>) -> u8 {
    iter.next().unwrap().parse::<u8>().unwrap()
}

impl Problem04 {
    pub fn new() -> Problem04 {
        Problem04 {}
    }

    fn parse(&self, data: &str) -> Vec<CleaningPair> {
        let mut all_pairs = Vec::new();

        for line in data.lines() {
            let pair_ranges = line
                .trim()
                .split(',')
                .map(|e| {
                    let mut range = e.split('-');
                    (parse_number(&mut range), parse_number(&mut range))
                })
                .collect::<Vec<(u8, u8)>>();

            all_pairs.push(CleaningPair {
                first: pair_ranges[0],
                second: pair_ranges[1],
            });
        }

        all_pairs
    }

    fn solve_actual(&self, cleaning_pairs: &[CleaningPair]) -> u16 {
        cleaning_pairs
            .iter()
            .map(|c| c.self_contained() as u16)
            .sum()
    }

    fn solve_actual_part2(&self, cleaning_pairs: &[CleaningPair]) -> u16 {
        cleaning_pairs.iter().map(|c| c.overlapping() as u16).sum()
    }
}

impl Problem for Problem04 {
    fn name(&self) -> &str {
        "Day 4: Camp Cleanup"
    }

    fn solve(&self) -> i64 {
        let data = get_input!("./inputs/problem_04.txt");
        let cleaning_pairs = self.parse(&data);
        self.solve_actual(&cleaning_pairs).into()
    }

    fn solve_part2(&self) -> i64 {
        let data = get_input!("./inputs/problem_04.txt");
        let cleaning_pairs = self.parse(&data);
        self.solve_actual_part2(&cleaning_pairs).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem04::new();
        let data = problem.parse(
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8",
        );
        assert_eq!(problem.solve_actual(&data), 2);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem04::new();
        let data = problem.parse(
            "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8",
        );
        assert_eq!(problem.solve_actual_part2(&data), 4);
    }
}
