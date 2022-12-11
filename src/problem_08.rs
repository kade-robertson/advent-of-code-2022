use crate::{
    problem::{Problem, Solution},
    square_grid::SquareGrid,
};

struct Forest {
    trees: SquareGrid<u8>,
}

impl Forest {
    pub fn new(grid_size: usize) -> Self {
        Self {
            trees: SquareGrid::new(grid_size),
        }
    }

    pub fn add_trees(&mut self, heights: &mut dyn Iterator<Item = u8>) {
        self.trees.extend(heights);
    }

    pub fn left_max_height(&self) -> SquareGrid<u8> {
        let mut new_grid = self.trees.clone();

        for row in 0..new_grid.size {
            for col in 1..new_grid.size {
                let previous = *self.trees.get(row, col - 1);
                let new_previous = *new_grid.get(row, col - 1);
                new_grid.set(row, col, previous.max(new_previous));
            }
        }

        new_grid
    }

    pub fn right_max_height(&self) -> SquareGrid<u8> {
        let mut new_grid = self.trees.clone();

        for row in 0..new_grid.size {
            for col in 2..new_grid.size + 1 {
                let col_rev = new_grid.size - col;
                let previous = *self.trees.get(row, col_rev + 1);
                let new_previous = *new_grid.get(row, col_rev + 1);
                new_grid.set(row, col_rev, previous.max(new_previous));
            }
        }

        new_grid
    }

    pub fn top_max_height(&self) -> SquareGrid<u8> {
        let mut new_grid = self.trees.clone();

        for row in 1..new_grid.size {
            for col in 0..new_grid.size {
                let previous = *self.trees.get(row - 1, col);
                let new_previous = *new_grid.get(row - 1, col);
                new_grid.set(row, col, previous.max(new_previous));
            }
        }

        new_grid
    }

    pub fn bottom_max_height(&self) -> SquareGrid<u8> {
        let mut new_grid = self.trees.clone();

        for row in 2..new_grid.size + 1 {
            let row_rev = new_grid.size - row;
            for col in 0..new_grid.size {
                let previous = *self.trees.get(row_rev + 1, col);
                let new_previous = *new_grid.get(row_rev + 1, col);
                new_grid.set(row_rev, col, previous.max(new_previous));
            }
        }

        new_grid
    }

    pub fn find_visible(&self) -> u32 {
        let exterior_total = (self.trees.size * 4) - 4;

        let left_heights = self.left_max_height();
        let right_heights = self.right_max_height();
        let top_heights = self.top_max_height();
        let bottom_heights = self.bottom_max_height();

        (exterior_total
            + self
                .trees
                .iter_no_border()
                .filter(|(h, row, col)| {
                    top_heights.get(*row, *col) < h
                        || bottom_heights.get(*row, *col) < h
                        || left_heights.get(*row, *col) < h
                        || right_heights.get(*row, *col) < h
                })
                .count()) as u32
    }

    pub fn best_scenic_score(&self) -> u32 {
        let end_of_grid = self.trees.size - 1;

        self.trees
            .iter_no_border()
            .map(|(h, row, col)| {
                let up_collision = (0usize..row)
                    .filter(|i| self.trees.get(*i, col) >= h)
                    .next_back()
                    .map_or(row, |t| row - t);

                let down_collision = (row + 1..self.trees.size)
                    .find(|i| self.trees.get(*i, col) >= h)
                    .map_or(end_of_grid - row, |t| t - row);

                let left_collision = (0usize..col)
                    .filter(|i| self.trees.get(row, *i) >= h)
                    .next_back()
                    .map_or(col, |t| col - t);

                let right_collision = (col + 1..self.trees.size)
                    .find(|i| self.trees.get(row, *i) >= h)
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
