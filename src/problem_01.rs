use crate::problem::{Problem, Solution};

pub struct Problem01 {}

impl Problem01 {
    pub fn new() -> Problem01 {
        Problem01 {}
    }

    fn parse(&self, data: &str) -> Vec<Vec<u32>> {
        let mut all_callories = Vec::new();
        let mut current_calories: Vec<u32> = Vec::new();

        for line in data.lines() {
            if line.is_empty() {
                all_callories.push(current_calories);
                current_calories = Vec::new();
            } else {
                current_calories.push(line.parse::<u32>().unwrap());
            }
        }

        all_callories
    }

    fn solve_actual(&self, calorie_counts: &[Vec<u32>]) -> u32 {
        calorie_counts.iter().map(|c| c.iter().sum()).max().unwrap()
    }

    fn solve_actual_part2(&self, calorie_counts: &[Vec<u32>]) -> u32 {
        let mut total_counts = calorie_counts
            .iter()
            .map(|c| c.iter().sum())
            .collect::<Vec<u32>>();
        total_counts.sort();

        total_counts.iter().rev().take(3).sum()
    }
}

impl Problem for Problem01 {
    fn name(&self) -> &str {
        "Day 1: Calorie Counting"
    }

    fn solve(&self) -> Solution {
        let data = get_input!("./inputs/problem_01.txt");
        let calorie_counts = self.parse(&data);
        Solution::U32(self.solve_actual(&calorie_counts))
    }

    fn solve_part2(&self) -> Solution {
        let data = get_input!("./inputs/problem_01.txt");
        let calorie_counts = self.parse(&data);
        Solution::U32(self.solve_actual_part2(&calorie_counts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let data: Vec<Vec<u32>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let problem = Problem01::new();
        assert_eq!(problem.solve_actual(&data), 24000);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let data: Vec<Vec<u32>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        let problem = Problem01::new();
        assert_eq!(problem.solve_actual_part2(&data), 45000);
    }
}
