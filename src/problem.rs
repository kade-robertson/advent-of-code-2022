#[allow(dead_code)]
pub enum Solution {
    U64(u64),
    U32(u32),
    U16(u16),
    USize(usize),
    Str(String),
}

pub trait Problem {
    fn name(&self) -> &str;
    fn solve(&self) -> Solution;
    fn solve_part2(&self) -> Solution;
}
