use std::collections::VecDeque;

use crate::problem::Problem;

pub struct Problem05 {}

#[derive(PartialEq)]
enum ParseState {
    LookingForCrates,
    SkipBlankLine,
    ReadingMoves,
}

#[derive(Debug)]
struct CraneInstruction {
    amount: u8,
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct CraneState {
    crate_columns: Vec<VecDeque<char>>,
    instructions: VecDeque<CraneInstruction>,
}

impl CraneState {
    fn process_instruction(&mut self) {
        let instruction = self.instructions.pop_front().expect("No more instructions");
        for _ in 0..instruction.amount {
            let crate_to_move = self
                .crate_columns
                .get_mut(instruction.from - 1)
                .expect("Tried to pull crate from nonexistent column")
                .pop_back()
                .expect("No crate to pull from column");
            self.crate_columns
                .get_mut(instruction.to - 1)
                .expect("Tried to add crate to nonexistent column")
                .push_back(crate_to_move);
        }
    }

    fn process_instruction_part2(&mut self) {
        let instruction = self.instructions.pop_front().expect("No more instructions");
        let from_column = self
            .crate_columns
            .get_mut(instruction.from - 1)
            .expect("Tried to pull crate from nonexistent column");
        let mut crate_to_move =
            from_column.split_off(from_column.len() - instruction.amount as usize);
        self.crate_columns
            .get_mut(instruction.to - 1)
            .expect("Tried to add crate to nonexistent column")
            .append(&mut crate_to_move);
    }

    fn process(&mut self) {
        while !self.instructions.is_empty() {
            self.process_instruction();
        }
    }

    fn process_part2(&mut self) {
        while !self.instructions.is_empty() {
            self.process_instruction_part2();
        }
    }

    fn get_top_crates(&self) -> String {
        let mut result = String::from("");

        self.crate_columns
            .iter()
            .filter(|col| !col.is_empty())
            .for_each(|col| result.push(*col.back().expect("No item in column")));

        result
    }
}

impl Problem05 {
    pub fn new() -> Problem05 {
        Problem05 {}
    }

    fn parse(&self, data: &str) -> CraneState {
        let mut state = ParseState::LookingForCrates;
        let mut crate_columns: Vec<VecDeque<char>> = Vec::with_capacity(10);
        let mut instructions: VecDeque<CraneInstruction> = VecDeque::new();
        for _ in 0..10 {
            crate_columns.push(VecDeque::new());
        }

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
                        crate_columns
                            .get_mut(column)
                            .expect("Column not present")
                            .push_front(*new_crate);
                    }
                    column += 1;
                    idx += 4;
                }
            } else if state == ParseState::ReadingMoves {
                let words = line.split(' ').collect::<Vec<_>>();
                instructions.push_back(CraneInstruction {
                    amount: words
                        .get(1)
                        .expect("Could not find amount")
                        .parse()
                        .expect("Could not parse amount"),
                    from: words
                        .get(3)
                        .expect("Could not find from")
                        .parse()
                        .expect("Could not parse from"),
                    to: words
                        .get(5)
                        .expect("Could not find to")
                        .parse()
                        .expect("Could not parse to"),
                })
            }
        }

        crate_columns = crate_columns
            .iter()
            .cloned()
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>();

        CraneState {
            crate_columns,
            instructions,
        }
    }

    fn solve_actual(&self, crane_state: &mut CraneState) -> String {
        crane_state.process();
        crane_state.get_top_crates()
    }

    fn solve_actual_part2(&self, crane_state: &mut CraneState) -> String {
        crane_state.process_part2();
        crane_state.get_top_crates()
    }
}

impl Problem for Problem05 {
    fn name(&self) -> &str {
        "Day 5: Supply Stacks"
    }

    fn solve(&self) -> String {
        let data = get_input!("./inputs/problem_05.txt");
        let mut crane_state = self.parse(&data);
        self.solve_actual(&mut crane_state)
    }

    fn solve_part2(&self) -> String {
        let data = get_input!("./inputs/problem_05.txt");
        let mut crane_state = self.parse(&data);
        self.solve_actual_part2(&mut crane_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem05::new();
        let mut data = problem.parse(
            "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!(problem.solve_actual(&mut data), "CMZ".to_string());
    }
}
