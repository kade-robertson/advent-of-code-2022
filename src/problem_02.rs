use crate::problem::Problem;

pub struct Problem02 {}

#[derive(Debug, PartialEq, Eq)]
enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
struct RPSGame {
    elf: RPSMove,
    me: RPSMove,
}

impl RPSGame {
    const fn determine_result(&self) -> u16 {
        match (&self.elf, &self.me) {
            (RPSMove::Rock, RPSMove::Paper)
            | (RPSMove::Paper, RPSMove::Scissors)
            | (RPSMove::Scissors, RPSMove::Rock) => 6,
            (RPSMove::Rock, RPSMove::Scissors)
            | (RPSMove::Paper, RPSMove::Rock)
            | (RPSMove::Scissors, RPSMove::Paper) => 0,
            _ => 3,
        }
    }

    const fn calculate_points(&self) -> u16 {
        (match self.me {
            RPSMove::Rock => 1,
            RPSMove::Paper => 2,
            RPSMove::Scissors => 3,
        }) + self.determine_result()
    }
}

impl Problem02 {
    pub fn new() -> Problem02 {
        Problem02 {}
    }

    fn parse(&self, data: &str) -> Vec<RPSGame> {
        let mut all_games = Vec::new();

        for line in data.lines() {
            let mut moves = line.split_ascii_whitespace().take(2);

            all_games.push(RPSGame {
                elf: match moves.next().unwrap() {
                    "A" => RPSMove::Rock,
                    "B" => RPSMove::Paper,
                    _ => RPSMove::Scissors,
                },
                me: match moves.next().unwrap() {
                    "X" => RPSMove::Rock,
                    "Y" => RPSMove::Paper,
                    _ => RPSMove::Scissors,
                },
            })
        }

        all_games
    }

    fn parse_part2(&self, data: &str) -> Vec<RPSGame> {
        let mut all_games = Vec::new();

        for line in data.lines() {
            let mut moves = line.split_ascii_whitespace().take(2);

            let elf_move = moves.next().unwrap();

            all_games.push(RPSGame {
                elf: match elf_move {
                    "A" => RPSMove::Rock,
                    "B" => RPSMove::Paper,
                    _ => RPSMove::Scissors,
                },
                me: match (moves.next().unwrap(), elf_move) {
                    ("X", "A") | ("Y", "C") | ("Z", "B") => RPSMove::Scissors,
                    ("X", "B") | ("Y", "A") | ("Z", "C") => RPSMove::Rock,
                    _ => RPSMove::Paper,
                },
            })
        }

        all_games
    }

    fn solve_actual(&self, rps_games: &[RPSGame]) -> u16 {
        rps_games.iter().map(|g| g.calculate_points()).sum()
    }
}

impl Problem for Problem02 {
    fn name(&self) -> &str {
        "Day 2: Rock Paper Scissors"
    }

    fn solve(&self) -> i64 {
        let data = get_input!("./inputs/problem_02.txt");
        let rps_games = self.parse(&data);
        self.solve_actual(&rps_games).into()
    }

    fn solve_part2(&self) -> i64 {
        let data = get_input!("./inputs/problem_02.txt");
        let rps_games = self.parse_part2(&data);
        self.solve_actual(&rps_games).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let data: Vec<RPSGame> = vec![
            RPSGame {
                elf: RPSMove::Rock,
                me: RPSMove::Paper,
            },
            RPSGame {
                elf: RPSMove::Paper,
                me: RPSMove::Rock,
            },
            RPSGame {
                elf: RPSMove::Scissors,
                me: RPSMove::Scissors,
            },
        ];
        let problem = Problem02::new();
        assert_eq!(problem.solve_actual(&data), 15);
    }

    #[test]
    fn test_parse_part2_from_example() {
        let data = "A Y\nB X\nC Z".to_owned();
        let problem = Problem02::new();
        let parsed = problem.parse_part2(&data);
        let expected_output: Vec<RPSGame> = vec![
            RPSGame {
                elf: RPSMove::Rock,
                me: RPSMove::Rock,
            },
            RPSGame {
                elf: RPSMove::Paper,
                me: RPSMove::Rock,
            },
            RPSGame {
                elf: RPSMove::Scissors,
                me: RPSMove::Rock,
            },
        ];

        assert_eq!(parsed, expected_output)
    }
}
