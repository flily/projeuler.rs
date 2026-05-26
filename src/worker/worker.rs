use std::time;

use crate::{common::{Problem, Solution, SolutionInfo}, problems};

use super::{FinalResult, RunResult, RunError};


pub struct Worker {
    pub problems: Vec<Problem>,
}

impl Worker {
    pub fn on(problems: Vec<Problem>) -> Self {
        Worker { problems: problems.iter().map(|p| p.clone()).collect() }
    }

    pub fn on_static(problems: Vec<&'static Problem>) -> Self {
        let ps = problems.iter().map(|p| (*p).clone()).collect();
        Worker::on(ps)
    }

    pub fn get_problem(&self, problem_id: i64) -> Result<Problem, RunError> {
        match self.problems.iter().find(|p| p.id == problem_id) {
            Some(p) => Ok(p.clone()),
            None => Err(RunError::ProblemNotFound { problem_id }),
        }
    }

    pub fn get_solution(&self, problem_id: i64, solution_id: usize) -> Result<SolutionInfo, RunError> {
        let problem = self.problems.iter().find(|p| p.id == problem_id);
        if problem.is_none() {
            return Err(RunError::ProblemNotFound { problem_id });
        }

        let problem = problem.unwrap();
        match problem.get_solution(solution_id) {
            Some(sln) => Ok(sln),
            None => Err(RunError::SolutionNotFound { problem_id, solution_id }),
        }
    }

    pub fn make_result(&self, problem_id: i64, solution_id: usize) -> Result<RunResult, RunError> {
        let problem = self.get_problem(problem_id)?;
        let result = problem.make_run_result_for(solution_id);
        if result.is_none() {
            return Err(RunError::SolutionNotFound { problem_id, solution_id });
        }

        let result = problem.make_run_result_for(solution_id);
        if result.is_none() {
            return Err(RunError::SolutionNotFound { problem_id, solution_id });
        }

        Ok(RunResult {
            solution: problem.solutions[solution_id].name.to_string(),
            entry: problem.solutions[solution_id].entry,
            answer: Some(problem.answer),
            got: None,
            result: FinalResult::None,
            cost: time::Duration::from_secs(0),
            extra_timeout_ms: problem.extra_time_ms,
        })
    }

    pub fn run(&self, pid: i64, sid: usize) -> Result<RunResult, RunError> {
        let problem = self.get_problem(pid)?;
        let solution = problem.get_solution(sid);
        if solution.is_none() {
            return Err(RunError::SolutionNotFound { problem_id: pid, solution_id: sid });
        }

        let solution = solution.unwrap();
        let mut result = problem.make_run_result_for(sid).unwrap();
        let start = time::Instant::now();
        let got = (solution.entry)();
        let cost = start.elapsed();

        result.got = Some(got);
        result.cost = cost;
        result.check();

        Ok(result)
    }
}
