pub type Solution = fn() -> i64;

pub struct SolutionInfo {
    pub name: &'static str,
    pub entry: Solution,
}

pub struct Problem {
    pub id: i64,
    pub title: &'static str,
    pub answer: i64,
    pub extra_time_ms: std::time::Duration,
    pub solutions: Vec<SolutionInfo>,
}

pub trait Checkable {
    fn check(&self, solution: i64) -> bool;
}

impl Checkable for Problem {
    fn check(&self, solution: i64) -> bool {
        solution == self.answer
    }
}
