use std::collections::VecDeque;

use anyhow::{Context, Result};

use crate::problem::{Problem, Solution};

pub struct Problem05 {}

#[derive(PartialEq)]
enum ParseState {
    LookingForCrates,
    SkipBlankLine,
    ReadingMoves,
}

#[derive(Debug)]
struct CraneGame {
    crate_columns: Vec<VecDeque<char>>,
}

impl CraneGame {
    fn new() -> Self {
        Self {
            crate_columns: vec![VecDeque::new(); 10],
        }
    }

    fn add_crate(&mut self, column: usize, crate_name: char) -> Result<()> {
        self.crate_columns
            .get_mut(column)
            .context("Column missing")?
            .push_front(crate_name);

        Ok(())
    }

    fn process_instruction(
        &mut self,
        amount: usize,
        from_col: usize,
        to_col: usize,
        pick_multiple: bool,
    ) -> Result<()> {
        let column_from = self
            .crate_columns
            .get_mut(from_col - 1)
            .context("From column does not exist")?;

        let crates_to_move: VecDeque<char> = if pick_multiple {
            column_from.split_off(column_from.len() - amount)
        } else {
            column_from
                .drain((column_from.len() - amount)..)
                .rev()
                .collect()
        };

        let column_to = self
            .crate_columns
            .get_mut(to_col - 1)
            .context("To column does not exist")?;

        column_to.extend(crates_to_move);

        Ok(())
    }

    fn get_top_crates(&self) -> Result<String> {
        let mut result = String::from("");

        self.crate_columns
            .iter()
            .filter(|col| !col.is_empty())
            .for_each(|col| result.push(*col.back().expect("No item in column")));

        Ok(result)
    }
}

impl Problem05 {
    pub fn new() -> Problem05 {
        Problem05 {}
    }

    fn parse(&self, data: &str, pick_multiple: bool) -> Result<CraneGame> {
        let mut state = ParseState::LookingForCrates;
        let mut crane_game = CraneGame::new();

        for line in data.lines() {
            let mut column = 0;
            let mut idx = 0;
            let line_chars = line.chars().collect::<Vec<char>>();

            if state == ParseState::LookingForCrates && line.starts_with(" 1") {
                state = ParseState::SkipBlankLine;
                continue;
            }

            if state == ParseState::SkipBlankLine {
                state = ParseState::ReadingMoves;
                continue;
            }

            if state == ParseState::LookingForCrates {
                while let Some(ch) = line_chars.get(idx) {
                    if ch == &'[' {
                        let new_crate = line_chars.get(idx + 1).expect("Missing crate identifier");
                        crane_game.add_crate(column, *new_crate)?;
                    }
                    column += 1;
                    idx += 4;
                }
            } else if state == ParseState::ReadingMoves {
                let words = line.split(' ').collect::<Vec<_>>();
                crane_game.process_instruction(
                    words.get(1).context("Could not find amount")?.parse()?,
                    words.get(3).context("Could not find from")?.parse()?,
                    words.get(5).context("Could not find to")?.parse()?,
                    pick_multiple,
                )?;
            }
        }

        Ok(crane_game)
    }
}

impl Problem for Problem05 {
    fn name(&self) -> &str {
        "Day 5: Supply Stacks"
    }

    fn solve(&self) -> Solution {
        let data = get_input!("./inputs/problem_05.txt");
        let crane_game = self.parse(&data, false).unwrap();
        Solution::Str(
            crane_game
                .get_top_crates()
                .unwrap_or_else(|_| "failed".into()),
        )
    }

    fn solve_part2(&self) -> Solution {
        let data = get_input!("./inputs/problem_05.txt");
        let crane_game = self.parse(&data, true).unwrap();
        Solution::Str(
            crane_game
                .get_top_crates()
                .unwrap_or_else(|_| "failed".into()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem05::new();
        let crane_game = problem
            .parse(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
                false,
            )
            .unwrap();
        assert_eq!(crane_game.get_top_crates().unwrap(), "CMZ".to_string());
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem05::new();
        let crane_game = problem
            .parse(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
                true,
            )
            .unwrap();
        assert_eq!(crane_game.get_top_crates().unwrap(), "MCD".to_string());
    }
}
