use crate::problem::{Problem, Solution};

struct Forest {
    grid_size: usize,
    trees: Vec<u8>,
}

impl Forest {
    pub fn new(grid_size: usize) -> Self {
        Self {
            grid_size,
            trees: Vec::with_capacity(grid_size * grid_size),
        }
    }

    pub fn add_trees(&mut self, heights: &mut dyn Iterator<Item = u8>) {
        self.trees.extend(heights);
    }

    pub fn tree_at(&self, row: usize, col: usize) -> u8 {
        self.trees[(row * self.grid_size) + col]
    }

    pub fn find_visible(&self) -> u32 {
        let exterior_total = (self.grid_size * 4) - 4;
        let end_of_grid = self.grid_size - 1;

        (exterior_total
            + self
                .trees
                .iter()
                .enumerate()
                .map(|(i, c)| (c, i / self.grid_size, i % self.grid_size))
                .filter(|(_, row, col)| {
                    *row != 0 && *row != end_of_grid && *col != 0 && *col != end_of_grid
                })
                .filter(|(h, row, col)| {
                    (0..*row).all(|i| self.tree_at(i, *col) < **h)
                        || (*row + 1..self.grid_size).all(|i| self.tree_at(i, *col) < **h)
                        || (0..*col).all(|i| self.tree_at(*row, i) < **h)
                        || (*col + 1..self.grid_size).all(|i| self.tree_at(*row, i) < **h)
                })
                .count()) as u32
    }

    pub fn best_scenic_score(&self) -> u32 {
        let end_of_grid = self.grid_size - 1;

        self.trees
            .iter()
            .enumerate()
            .map(|(i, c)| (c, i / self.grid_size, i % self.grid_size))
            .map(|(h, row, col)| {
                if row == 0 || row == end_of_grid || col == 0 || col == end_of_grid {
                    return 0;
                }
                let up_collision = (0usize..row)
                    .filter(|i| self.tree_at(*i, col) >= *h)
                    .next_back()
                    .map_or(row, |t| row - t);

                let down_collision = (row + 1..self.grid_size)
                    .find(|i| self.tree_at(*i, col) >= *h)
                    .map_or(end_of_grid - row, |t| t - row);

                let left_collision = (0usize..col)
                    .filter(|i| self.tree_at(row, *i) >= *h)
                    .next_back()
                    .map_or(col, |t| col - t);

                let right_collision = (col + 1..self.grid_size)
                    .find(|i| self.tree_at(row, *i) >= *h)
                    .map_or(end_of_grid - col, |t| t - col);

                up_collision * down_collision * left_collision * right_collision
            })
            .max()
            .unwrap() as u32
    }
}

pub struct Problem08 {}

impl Problem08 {
    pub fn new() -> Problem08 {
        Problem08 {}
    }

    fn parse(&self, data: &str) -> Forest {
        let grid_size = data.lines().next().unwrap().trim().len();
        let mut forest = Forest::new(grid_size);

        forest.add_trees(
            &mut data
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| (c as u8) - b'0'),
        );

        forest
    }

    fn solve_actual(&self, forest: &Forest) -> u32 {
        forest.find_visible()
    }

    fn solve_actual_part2(&self, forest: &Forest) -> u32 {
        forest.best_scenic_score()
    }
}

impl Problem for Problem08 {
    fn name(&self) -> &str {
        "Day 8: Treetop Tree House"
    }

    fn solve(&self) -> Solution {
        let data = get_input!("./inputs/problem_08.txt");
        let forest = self.parse(&data);
        Solution::U32(self.solve_actual(&forest))
    }

    fn solve_part2(&self) -> Solution {
        let data = get_input!("./inputs/problem_08.txt");
        let forest = self.parse(&data);
        Solution::U32(self.solve_actual_part2(&forest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem08::new();
        let forest = problem.parse(
            "30373
        25512
        65332
        33549
        35390",
        );

        assert_eq!(problem.solve_actual(&forest), 21);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem08::new();
        let forest = problem.parse(
            "30373
        25512
        65332
        33549
        35390",
        );

        assert_eq!(problem.solve_actual_part2(&forest), 8);
    }
}
