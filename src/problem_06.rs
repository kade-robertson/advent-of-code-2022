use crate::problem::{Problem, Solution};

pub struct Problem06 {}

impl Problem06 {
    pub fn new() -> Problem06 {
        Problem06 {}
    }

    fn solve(&self, data: &str, window_size: usize) -> u64 {
        (data
            .chars()
            .collect::<Vec<char>>()
            .windows(window_size)
            .enumerate()
            .find(|(_, chrs)| {
                chrs.iter()
                    .fold((0u32, true), |(acc, valid), c| {
                        let chr_bit = 1 << (*c as u8 - b'a');
                        (acc ^ chr_bit, valid & (acc & chr_bit != chr_bit))
                    })
                    .1
            })
            .unwrap()
            .0
            + window_size) as u64
    }
}

impl Problem for Problem06 {
    fn name(&self) -> &str {
        "Day 6: Tuning Trouble"
    }

    fn solve(&self) -> Solution {
        let data = get_input!("./inputs/problem_06.txt");
        let first_four_unique_index = self.solve(&data, 4);
        Solution::U64(first_four_unique_index)
    }

    fn solve_part2(&self) -> Solution {
        let data = get_input!("./inputs/problem_06.txt");
        let first_four_unique_index = self.solve(&data, 14);
        Solution::U64(first_four_unique_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem06::new();
        assert_eq!(problem.solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(problem.solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(problem.solve("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(problem.solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(problem.solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem06::new();
        assert_eq!(problem.solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(problem.solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(problem.solve("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(problem.solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(problem.solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
