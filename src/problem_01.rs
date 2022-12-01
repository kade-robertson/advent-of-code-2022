use crate::problem::Problem;

pub struct Problem01 {}

impl Problem01 {
    pub fn new() -> Problem01 {
        Problem01 {}
    }

    fn solve_actual(&self, value: i64) -> i64 {
        0
    }

    fn solve_actual_part2(&self, value: i64) -> i64 {
        0
    }
}

impl Problem for Problem01 {
    fn name(&self) -> &str {
        "Day 1: Sonar Sweep"
    }

    fn solve(&self) -> i64 {
        self.solve_actual(0)
    }

    fn solve_part2(&self) -> i64 {
        self.solve_actual_part2(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem01::new();
        assert_eq!(problem.solve_actual(0), 0);
    }
}
