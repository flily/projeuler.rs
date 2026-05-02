pub type Solution = fn() -> i64;

pub struct SolutionInfo {
    pub name: String,
    pub entry: Solution,
}

pub struct Problem {
    pub id: i64,
    pub title: String,
    pub answer: i64,
    pub extra_time_ms: std::time::Duration,
    pub solutions: Vec<SolutionInfo>,
}

pub trait Checkable {
    fn run(&self, solution: Solution) -> i64;
    fn check(&self, solution: Solution) -> bool;
}

impl Checkable for Problem {
    fn run(&self, solution: Solution) -> i64 {
        solution()
    }

    fn check(&self, solution: Solution) -> bool {
        self.run(solution) == self.answer
    }
}
