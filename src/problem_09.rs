use std::collections::HashSet;

use crate::problem::{Problem, Solution};

struct KnotSimulator {
    knot_points: Vec<(i16, i16)>,
    tail_visited: HashSet<i32>,
}

impl KnotSimulator {
    pub fn new(length: usize) -> Self {
        Self {
            knot_points: vec![(0, 0); length],
            tail_visited: HashSet::from([0]),
        }
    }

    pub fn step(&mut self, direction: char, distance: u8) {
        (0..distance).for_each(|_| {
            let head = self.knot_points.first_mut().unwrap();
            // First, move the head to its new position
            match direction {
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                _ => unimplemented!(),
            };

            for i in 1..self.knot_points.len() {
                let previous = *self.knot_points.get(i - 1).unwrap();
                let current = self.knot_points.get_mut(i).unwrap();
                let xdelta = previous.0 - current.0;
                let ydelta = previous.1 - current.1;
                *current = match (xdelta, ydelta) {
                    (2, 0) => (current.0 + 1, current.1),
                    (-2, 0) => (current.0 - 1, current.1),
                    (0, 2) => (current.0, current.1 + 1),
                    (0, -2) => (current.0, current.1 - 1),
                    (2, 1) | (2, 2) | (1, 2) => (current.0 + 1, current.1 + 1),
                    (2, -1) | (2, -2) | (1, -2) => (current.0 + 1, current.1 - 1),
                    (-2, 1) | (-2, 2) | (-1, 2) => (current.0 - 1, current.1 + 1),
                    (-2, -1) | (-2, -2) | (-1, -2) => (current.0 - 1, current.1 - 1),
                    _ => *current,
                }
            }

            let tail = self.knot_points.last().unwrap();
            self.tail_visited
                .insert(((tail.0 as i32) << 8) + tail.1 as i32);
        });
    }
}

pub struct Problem09 {}

impl Problem09 {
    pub fn new() -> Problem09 {
        Problem09 {}
    }

    fn parse(&self, data: &str, knot_size: usize) -> KnotSimulator {
        let mut simulator = KnotSimulator::new(knot_size);

        data.lines().for_each(|l| {
            let mut instruction = l.trim().split(' ');
            simulator.step(
                instruction.next().unwrap().chars().next().unwrap(),
                instruction.next().unwrap().parse::<u8>().unwrap(),
            );
        });

        simulator
    }

    fn solve_actual(&self, simulator: &KnotSimulator) -> usize {
        simulator.tail_visited.len()
    }
}

impl Problem for Problem09 {
    fn name(&self) -> &str {
        "Day 9: Rope Bridge"
    }

    fn solve(&self) -> Solution {
        let data = get_input!("./inputs/problem_09.txt");
        let simulator = self.parse(&data, 2);
        Solution::USize(self.solve_actual(&simulator))
    }

    fn solve_part2(&self) -> Solution {
        let data = get_input!("./inputs/problem_09.txt");
        let simulator = self.parse(&data, 10);
        Solution::USize(self.solve_actual(&simulator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_actual_from_example() {
        let problem = Problem09::new();
        let simulator = problem.parse(
            "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2",
            2,
        );
        assert_eq!(problem.solve_actual(&simulator), 13);
    }

    #[test]
    fn test_solve_actual_part2_from_example() {
        let problem = Problem09::new();
        let simulator = problem.parse(
            "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2",
            10,
        );
        assert_eq!(problem.solve_actual(&simulator), 1);
    }

    #[test]
    fn test_solve_actual_part2_from_example_2() {
        let problem = Problem09::new();
        let simulator = problem.parse(
            "R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20",
            10,
        );
        assert_eq!(problem.solve_actual(&simulator), 36);
    }
}
