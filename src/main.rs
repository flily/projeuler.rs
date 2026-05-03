use std::sync::mpsc;
use std::thread;
use std::time;

use clap::{Parser, Subcommand};
use colored::{Colorize, Color};

use crate::common::Checkable;

mod common;
mod problems;


#[derive(Subcommand)]
enum Command {
    /// run solutions of problems
    Run {
        /// timeout with measurement unit, e.g. "1s", "500ms". Default is "500ms".
        #[arg(short, long="timeout", name="TIMEOUT", default_value="500ms")]
        timeout_str: String,
        /// do not limit execution time
        #[arg(short='o', long="no-timeout", default_value_t=false)]
        no_timeout: bool,
        /// check answers after running
        #[arg(short, long="check", default_value_t=false)]
        check_answers: bool,
        /// strict mode, all solutions MUST return correct answer.
        #[arg(short, long="strict", default_value_t=false)]
        strict_mode: bool,
        pids: Vec<i64>,
    },
    /// list problems
    List { 
        pids: Vec<i64>
    },
}

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn parse_duration(s: &str) -> Result<u64, String> {
    let s = s.trim();
    let value_str: &str;
    let base: u64;

    if s.ends_with("ms") {
        value_str = &s[..s.len() - 2];
        base = 1;

    } else if s.ends_with("s") {
        value_str = &s[..s.len() - 1];
        base = 1000;

    } else {
        value_str = s;
        base = 1000; // default to seconds if no unit is provided
    }

    let value = value_str.parse::<u64>()
        .map_err(|e| e.to_string())?;

    Ok(value * base)
}

enum SolutionResult {
    None,
    Correct,
    Wrong,
    Timeout,
    Crash,
}

fn solution_result_str(result: &SolutionResult) -> &str {
    match result {
        SolutionResult::None => "-",
        SolutionResult::Correct => "correct",
        SolutionResult::Wrong => "wrong",
        SolutionResult::Timeout => "timeout",
        SolutionResult::Crash => "crash",
    }
}

struct RunResult {
    pid: i64,
    title: &'static str,
    solution: &'static str,
    entry: common::Solution,
    answer: Option<i64>,
    got: Option<i64>,
    result: SolutionResult,
    cost_ms: f64,
}

impl Checkable for RunResult {
    fn check(&self, result: i64) -> bool {
        result == self.answer.unwrap()
    }
}

fn make_run_results(info: &common::Problem) -> Vec<RunResult> {
    info.solutions.iter().map(|sln| RunResult {
        pid: info.id,
        title: info.title,
        solution: sln.name,
        entry: sln.entry,
        answer: Some(info.answer),
        got: None,
        result: SolutionResult::None, // default to None, will be updated after running
        cost_ms: 0.0,
    }).collect()
}

fn run_solution(run_result: &mut RunResult, timeout_ms: u64, check_answer: bool) {
    let entry = run_result.entry;
    let (tx, rx) = mpsc::sync_channel(1);

    let t1 = time::Instant::now();
    thread::spawn(move || {
        let result = std::panic::catch_unwind(entry);
        let _ = tx.send(result);
    });
    let timeout = time::Duration::from_millis(timeout_ms);

    let response = if timeout_ms == 0 {
        rx.recv().map_err(|e| e.into())
    } else {
        rx.recv_timeout(timeout).map_err(|e| e.into())
    };

    match response {
        Ok(Ok(got)) => {
            run_result.got = Some(got);
            if check_answer {
                run_result.result = if run_result.check(got) {
                    SolutionResult::Correct
                } else {
                    SolutionResult::Wrong
                };
            }
        }
        Ok(Err(_)) => run_result.result = SolutionResult::Crash,
        Err(mpsc::RecvTimeoutError::Timeout) => run_result.result = SolutionResult::Timeout,
        Err(mpsc::RecvTimeoutError::Disconnected) => run_result.result = SolutionResult::Crash,
    }
    run_result.cost_ms = t1.elapsed().as_nanos() as f64 / 1_000_000.0;

}

fn field_contect_adjust<T: ToString>(s: T, i: usize) -> String {
    if i == 0 {
        s.to_string()
    } else {
        " ".to_string()
    }
}

fn color_result(result: &SolutionResult) -> colored::ColoredString {
    let s = solution_result_str(result).to_string();

    match result {
        SolutionResult::Correct => s.green(),
        SolutionResult::Timeout => s.yellow(),
        SolutionResult::Wrong => s.red(),
        SolutionResult::Crash => s.red(),
        _ => s.normal(),
    }
}

fn color_cost_time(cost_ms: f64) -> colored::ColoredString {
    let s = format!("{:.3} ms", cost_ms);

    if cost_ms < 100.0 {
        s.green()

    } else if cost_ms < 200.0 {
        s.cyan()

    } else if cost_ms < 400.0 {
        s.yellow()

    } else {
        s.red()
    }
}

fn do_run(pids: Vec<i64>, timeout_ms: u64, check_answers: bool, _: bool) {
    let sepline = "+".to_string()
        + &"-".repeat(4 + 2) + "+"      // PID
        + &"-".repeat(40 + 2) + "+"     // Title
        + &"-".repeat(40 + 2) + "+"     // Solution
        + &"-".repeat(9 + 2) + "+"      // Result
        + &"-".repeat(12 + 2) + "+"     // Time 12345.678 ms
    ;

    println!("{}", sepline);
    println!("| {:^4} | {:^40} | {:^40} | {:^9} | {:^12} |",
        "PID", "Title", "Solution", "Result", "Time");
    println!("{}", sepline);

    let mut count_problems = 0;
    let mut count_problems_succ = 0;
    let mut count_solutions = 0;
    let mut count_solutions_succ = 0;
    
    let start_time = time::Instant::now();
    for problem in problems::all_problems().iter() {
        let mut has_failure = false;
        if !pids.is_empty() && !pids.contains(&problem.id) {
            continue;
        }

        let mut solutions = make_run_results(problem);
        for sln in solutions.iter_mut() {
            let sln_timeout_ms = timeout_ms + problem.extra_time_ms.as_millis() as u64;
            run_solution(sln, sln_timeout_ms, check_answers);
        }

        for (i, sln) in solutions.iter().enumerate() {
            match sln.result {
                SolutionResult::Correct => {
                    count_solutions_succ += 1;
                },
                SolutionResult::Timeout => {},
                _ => has_failure = true,
            }

            let pid = field_contect_adjust(sln.pid, i);
            let title = field_contect_adjust(sln.title, i);
            let result = color_result(&sln.result);
            let cost = color_cost_time(sln.cost_ms);

            println!("| {:>4} | {:<40} | {:<40} | {:^9} | {:>12} |",
                pid, title, sln.solution, result, cost);
            count_solutions += 1;
        }

        if !has_failure {
            count_problems_succ += 1;
        }
        count_problems += 1;
    }
    let elapsed_time = start_time.elapsed().as_nanos() as f64 / 1_000_000.0;
    println!("{}", sepline);

    let succ_rate = if count_problems > 0 {
        let rate = (count_problems_succ as f64) / (count_problems as f64) * 100.0;
        if rate == 100.0 {
            format!("{:.2}", rate).green()
        } else {
            format!("{:.2}", rate).red()
        }

    } else {
        "-".yellow()
    };

    let problem_succ = if count_problems_succ == count_problems {
        count_problems_succ.to_string().green()
    } else {
        count_problems_succ.to_string().red()
    };
    let problem_total = count_problems.to_string().blue();

    println!("Problems: {}/{} ({}%) , Solutions: {}/{}",
        problem_succ, problem_total, succ_rate,
        count_solutions_succ, count_solutions);
    println!("Total time: {:.3} ms", elapsed_time);
}

fn do_list(pids: Vec<i64>) {
    let sepline = "+".to_string()
        + &"-".repeat(4 + 2) + "+"      // PID
        + &"-".repeat(40 + 2) + "+"     // Title
        + &"-".repeat(40 + 2) + "+"     // Solution
    ;

    println!("{}", sepline);
    println!("| {:^4} | {:^40} | {:^40} |", "PID", "Title", "Solution");
    println!("{}", sepline);

    let mut count_problems = 0;
    let mut count_solutions = 0;

    let colors = vec![
        Color::Blue,
        Color::Yellow,
        Color::Green,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];
    let problems = problems::all_problems();
    let mut j = 0;
    for problem in problems {
        if !pids.is_empty() && pids.contains(&problem.id) {
            continue;
        }

        for (i, sln) in problem.solutions.iter().enumerate() {
            let pid = field_contect_adjust(problem.id, i).blue();
            let title = field_contect_adjust(problem.title, i).green();
            let name = sln.name.color(colors[j % colors.len()]);
            println!("| {:>4} | {:<40} | {:<40} |", pid, title, name);
            count_solutions += 1;
            j += 1;
        }
        count_problems += 1;
    }

    println!("{}", sepline);
    println!("found {} problems with {} solutions",
        count_problems.to_string().yellow(),
        count_solutions.to_string().yellow(),
    );
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run { 
            pids, timeout_str, no_timeout, check_answers, strict_mode
        } => {
            let timeout_ms = if no_timeout { 0 } 
                else {
                    match parse_duration(&timeout_str) {
                        Ok(ms) => ms,
                        Err(e) => {
                            eprintln!("invalid timeout '{}': {}", timeout_str, e);
                            return;
                        }
                    }
                };
            do_run(pids, timeout_ms, check_answers, strict_mode);
        }

        Command::List { pids } => do_list(pids),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("100").unwrap(), 100_000);
        assert_eq!(parse_duration("100ms").unwrap(), 100);
        assert_eq!(parse_duration("1s").unwrap(), 1000);
        assert!(parse_duration("abc").is_err());
        assert!(parse_duration("1m").is_err());
    }
}
