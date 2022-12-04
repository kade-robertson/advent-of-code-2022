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
        (self.second.0 <= self.first.0 && self.first.0 <= self.second.1)
            || (self.second.0 <= self.first.1 && self.first.1 <= self.second.1)
            || (self.first.0 <= self.second.0 && self.second.0 <= self.first.1)
            || (self.first.0 <= self.second.1 && self.second.1 <= self.first.1)
    }
}

impl Problem04 {
    pub fn new() -> Problem04 {
        Problem04 {}
    }

    fn parse(&self, data: &str) -> Vec<CleaningPair> {
        data.lines()
            .map(|l| {
                l.trim()
                    .split(|c| c == ',' || c == '-')
                    .take(4)
                    .map(|n| n.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .map(|vn| CleaningPair {
                first: (vn[0], vn[1]),
                second: (vn[2], vn[3]),
            })
            .collect::<Vec<CleaningPair>>()
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
