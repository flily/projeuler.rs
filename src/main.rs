use std::sync::mpsc;
use std::thread;
use std::time;

use clap::{Parser, Subcommand};

mod common;
mod problems;


#[derive(Subcommand)]
enum Command {
    /// run solutions of problems
    Run {
        /// timeout with measurement unit, e.g. "1s", "500ms". Default is "1s".
        #[arg(short, long="timeout", name="TIMEOUT", default_value="1s")]
        timeout_str: String,
        /// do not limit execution time
        #[arg(long="no-timeout", default_value_t=false)]
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
    List { pids: Vec<i64> },
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
        SolutionResult::None => "NONE",
        SolutionResult::Correct => "correct",
        SolutionResult::Wrong => "wrong",
        SolutionResult::Timeout => "timeout",
        SolutionResult::Crash => "crash",
    }
}

struct RunResult {
    pid: i64,
    title: String,
    solution: String,
    entry: common::Solution,
    answer: Option<i64>,
    got: Option<i64>,
    result: SolutionResult,
    cost_ms: f64,
}

fn make_run_results(info: &common::Problem) -> Vec<RunResult> {
    info.solutions.iter().map(|sln| RunResult {
        pid: info.id,
        title: info.title.clone(),
        solution: sln.name.clone(),
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
    match rx.recv_timeout(timeout) {
        Ok(Ok(got)) => {
            run_result.got = Some(got);
            if check_answer {
                run_result.result = if got == run_result.answer.unwrap() {
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

fn do_run(pids: Vec<i64>, timeout_ms: u64, check_answers: bool, _: bool) {
    let sepline = "+".to_string()
        + &"-".repeat(4 + 2) + "+"      // PID
        + &"-".repeat(40 + 2) + "+"     // Title
        + &"-".repeat(20 + 2) + "+"     // Solution
        + &"-".repeat(9 + 2) + "+"      // Result
        + &"-".repeat(12 + 2) + "+"     // Time 12345.678 ms
    ;

    println!("{}", sepline);
    println!("| {:^4} | {:^40} | {:^20} | {:^9} | {:^12} |",
        "PID", "Title", "Solution", "Result", "Time");
    println!("{}", sepline);

    for problem in problems::all_problems().iter() {
        if !pids.is_empty() && !pids.contains(&problem.id) {
            continue;
        }

        let mut solutions = make_run_results(problem);
        for sln in solutions.iter_mut() {
            run_solution(sln, timeout_ms, check_answers);
        }

        for sln in solutions {
            println!("| {:>4} | {:<40} | {:<20} | {:^9} | {:>9.3} ms |",
                sln.pid, sln.title, sln.solution, solution_result_str(&sln.result), sln.cost_ms);
        }
    }

    println!("{}", sepline);
}

fn do_list(pids: Vec<i64>) {
    let problems = problems::all_problems();
    for problem in problems {
        if !pids.is_empty() && pids.contains(&problem.id) {
            continue;
        }

        println!("{}: {}", problem.id, problem.title);
    }
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
