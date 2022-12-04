use crate::problem::Problem;

pub struct Problem04 {}

struct CleaningPair {
    first: (u8, u8),
    second: (u8, u8),
}

impl CleaningPair {
    /**
     * I did attempt to make this a bit faster by avoiding the ugly
     * .next().unwrap() usage on the iterator by doing this instead. It was
     * technically faster, but only by about 1µs and the resulting
     * implementation looked even worse.
     *
     * pub fn new(pair_str: &str) -> Self {
     *     let mut pair = CleaningPair { values: [0; 4] };
     *
     *     pair_str
     *         .trim()
     *         .split(|c| c == ',' || c == '-')
     *         .take(4)
     *         .enumerate()
     *         .for_each(|(i, n)| {
     *             pair.values[i] = n.parse::<u8>().unwrap();
     *         });
     *
     *     pair
     * }
     */

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
                let mut range_values = l
                    .trim()
                    .split(|c| c == ',' || c == '-')
                    .take(4)
                    .map(|n| n.parse::<u8>().unwrap());

                CleaningPair {
                    first: (range_values.next().unwrap(), range_values.next().unwrap()),
                    second: (range_values.next().unwrap(), range_values.next().unwrap()),
                }
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
