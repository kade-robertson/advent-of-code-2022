pub trait Problem {
    fn name(&self) -> &str;
    fn solve(&self) -> String;
    fn solve_part2(&self) -> String;
}
