use std::io::{self, BufRead, Write};
use std::sync::mpsc;
use std::thread;
use std::time;

use clap::{Parser, Subcommand};
use colored::{Color, Colorize};

use crate::common::{Checkable, Problem, SolutionInfo};
use management::ProblemManagement;

mod common;
mod problems;
mod management;

#[derive(Subcommand)]
enum Command {
    /// run solutions of problems
    #[command(visible_aliases = ["r"])]
    Run {
        /// timeout with measurement unit, e.g. "1s", "500ms". Default is "500ms".
        #[arg(short = 't', long = "timeout", name = "TIMEOUT", default_value = "500ms")]
        timeout_str: String,
        /// do not limit execution time
        #[arg(short = 'o', long = "no-timeout", default_value_t = false)]
        no_timeout: bool,
        /// check answers after running
        #[arg(short = 'c', long = "check", default_value_t = false)]
        check_answers: bool,
        /// always color the output, even when not running in a terminal
        #[arg(long = "color", default_value_t = false)]
        always_color: bool,
        pids: Vec<i64>,
    },
    /// list problems
    #[command(visible_aliases = ["l", "ls"])]
    List { pids: Vec<i64> },
    /// add a new problem and generate template code
    #[command(visible_aliases = ["a", "n", "new"])]
    Add {
        /// optional problem title
        #[arg(short = 'n', long = "title", default_value = "")]
        title: String,
        /// known answer for the problem
        #[arg(short = 'a', long = "answer", default_value_t = 0)]
        answer: i64,
        /// do not generate a solution file for the problem
        #[arg(short = 'd', long = "dry-run", default_value_t = false)]
        dry_run: bool,
        /// problem ID (e.g. 100)
        pid: i64,
        /// names of solution files to generate (e.g. "naive", "optimized")
        solutions: Vec<String>,
    },
    /// delete solution code for one or more problems
    #[command(visible_aliases = ["del", "rm"])] // don't assign short alias for delete
    Delete {
        /// problem IDs to delete
        #[arg(required = true)]
        pids: Vec<i64>,
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

    if let Some(stripped) = s.strip_suffix("ms") {
        value_str = stripped;
        base = 1;
    } else if let Some(stripped) = s.strip_suffix("s") {
        value_str = stripped;
        base = 1000;
    } else {
        value_str = s;
        base = 1000; // default to seconds if no unit is provided
    }

    let value = value_str.parse::<u64>().map_err(|e| e.to_string())?;

    Ok(value * base)
}

#[derive(Clone)]
enum FinalResult {
    None,
    Correct,
    Wrong,
    Timeout,
    Crash,
}

impl FinalResult {
    fn to_string(&self) -> &str {
        match self {
            FinalResult::None => "-",
            FinalResult::Correct => "correct",
            FinalResult::Wrong => "wrong",
            FinalResult::Timeout => "timeout",
            FinalResult::Crash => "crash",
        }
    }

    fn color(&self) -> colored::Color {
        match self {
            FinalResult::Correct => Color::Green,
            FinalResult::Wrong => Color::Red,
            FinalResult::Timeout => Color::Yellow,
            FinalResult::Crash => Color::Red,
            _ => Color::White,
        }
    }

    fn color_string(&self) -> colored::ColoredString {
        self.to_string().color(self.color())
    }

    fn color_on(&self, s: &str) -> colored::ColoredString {
        s.color(self.color())
    }
}

struct RunResult {
    solution: &'static str,
    entry: common::Solution,
    answer: Option<i64>,
    got: Option<i64>,
    result: FinalResult,
    cost_ms: f64,
}

impl Checkable for RunResult {
    fn check(&self, result: i64) -> bool {
        result == self.answer.unwrap()
    }
}

fn make_run_results(info: &common::Problem) -> Vec<RunResult> {
    info.solutions
        .iter()
        .map(|sln| RunResult {
            solution: sln.name,
            entry: sln.entry,
            answer: Some(info.answer),
            got: None,
            result: FinalResult::None, // default to None, will be updated after running
            cost_ms: 0.0,
        })
        .collect()
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
        rx.recv_timeout(timeout)
    };

    match response {
        Ok(Ok(got)) => {
            run_result.got = Some(got);
            if check_answer {
                run_result.result = if run_result.check(got) {
                    FinalResult::Correct
                } else {
                    FinalResult::Wrong
                };
            }
        }
        Ok(Err(_)) => run_result.result = FinalResult::Crash,
        Err(mpsc::RecvTimeoutError::Timeout) => run_result.result = FinalResult::Timeout,
        Err(mpsc::RecvTimeoutError::Disconnected) => run_result.result = FinalResult::Crash,
    }
    run_result.cost_ms = t1.elapsed().as_nanos() as f64 / 1_000_000.0;
}

fn color_cost_time(cost_ms: f64, timeout: f64) -> colored::ColoredString {
    let s = format!("{:.3} ms", cost_ms);
    let prop = cost_ms / timeout;

    if prop < 0.1 {
        s.green()
    } else if prop < 0.2 {
        s.blue()
    } else if prop < 0.3 {
        s.cyan()
    } else if prop < 0.5 {
        s.yellow()
    } else if prop < 0.8 {
        s.magenta()
    } else {
        s.red()
    }
}

fn print_problem_result(problem: &common::Problem, problem_result: FinalResult, timeout_ms: f64, cost_ms: f64) {
    let total_timeout = (problem.solutions.len() as f64) * timeout_ms;

    let pid = problem_result.color_on(&problem.id.to_string());
    let title = problem_result.color_on(problem.title);
    let cost = color_cost_time(cost_ms, total_timeout);
    let result = problem_result.color_string();
    let solutions = format!("+-- {} solutions", problem.solutions.len());
    println!(
        "| {:>4} | {:<40} | {:<40} | {:^14} | {:^9} | {:>12} |",
        pid, title, solutions, "", result, cost,
    );
}

fn print_solution_result(run_result: &RunResult, timeout_ms: f64, is_best: bool) {
    let result = run_result.result.color_string();
    let solution = if is_best {
        format!("* {}", run_result.solution).color(run_result.result.color()).bold().underline()
    } else {
        format!("+ {}", run_result.solution).color(run_result.result.color())
    };
    let cost: colored::ColoredString = if is_best {
        color_cost_time(run_result.cost_ms, timeout_ms).bold().underline()
    } else {
        color_cost_time(run_result.cost_ms, timeout_ms)
    };
    let answer = if let Some(got) = run_result.got {
        got.to_string().color(run_result.result.color())
    } else {
        "NO RESULT".red()
    };

    println!(
        "| {:>4} | {:<40} | {:<40} | {:^14} | {:^9} | {:>12} |",
        "", "", solution, answer, result, cost,
    );
}

fn make_problem_result(solutions: &[RunResult]) -> (FinalResult, i32) {
    let mut result = FinalResult::Timeout;
    let mut best_index = -1;
    let mut best_time = f64::MAX;

    for (i, sln) in solutions.iter().enumerate() {
        match sln.result {
            FinalResult::Timeout => {}
            FinalResult::Correct => {
                result = FinalResult::Correct;
                if sln.cost_ms < best_time {
                    best_time = sln.cost_ms;
                    best_index = i as i32;
                }

            }
            _ => {
                result = sln.result.clone();
                break;
            }
        }
    }

    (result, best_index)
}

fn print_one_solution_problem(problem: &common::Problem, run_result: &RunResult, timeout_ms: f64) {
    let pid = run_result.result.color_on(&problem.id.to_string());
    let title = run_result.result.color_on(problem.title);
    let result = run_result.result.color_string();
    let cost = color_cost_time(run_result.cost_ms, timeout_ms)
        .bold()
        .underline();
    let solution = run_result
        .result
        .color_on(&format!("- {}", run_result.solution))
        .bold()
        .underline();
    let answer = if let Some(got) = run_result.got {
        got.to_string().color(run_result.result.color())
    } else {
        "NO RESULT".red()
    };

    println!(
        "| {:>4} | {:<40} | {:<40} | {:^14} | {:^9} | {:>12} |",
        pid, title, solution, answer, result, cost,
    );
}

fn print_result(
    problem: &common::Problem,
    solutions: &[RunResult],
    timeout_ms: f64,
    cost: time::Duration,
) -> (i32, i32) {
    if solutions.len() == 1 {
        let sln = &solutions[0];
        print_one_solution_problem(problem, sln, timeout_ms);

        let c = match sln.result {
            FinalResult::Correct => (1, 1),
            _ => (0, 1),
        };

        return c;
    }

    let (problem_result, best_index) = make_problem_result(solutions);
    let cost_ms = cost.as_nanos() as f64 / 1_000_000.0;
    print_problem_result(problem, problem_result, timeout_ms, cost_ms);

    let mut correct_count = 0;
    for (i, sln) in solutions.iter().enumerate() {
        if let FinalResult::Correct = sln.result {
            correct_count += 1;
        }
        print_solution_result(sln, timeout_ms, best_index == (i as i32));
    }

    (correct_count, solutions.len() as i32)
}

fn do_run(pids: Vec<i64>, timeout_ms: u64, check_answers: bool) {
    let sepline = "+".to_string()
        + &"-".repeat(4 + 2) + "+"      // PID
        + &"-".repeat(40 + 2) + "+"     // Title
        + &"-".repeat(40 + 2) + "+"     // Solution
        + &"-".repeat(14 + 2) + "+"     // Answer
        + &"-".repeat(9 + 2) + "+"      // Result
        + &"-".repeat(12 + 2) + "+"     // Time 12345.678 ms
    ;

    println!("{}", sepline);
    println!(
        "| {:>4} | {:<40} | {:<40} | {:^14} | {:^9} | {:>12} |",
        "PID", "Title", "Solution", "Answer", "Result", "Time",
    );
    println!("{}", sepline);

    let mut count_problems = 0;
    let mut count_problems_succ = 0;
    let mut count_solutions = 0;
    let mut count_solutions_succ = 0;

    let start_time = time::Instant::now();
    for problem in problems::all_problems().iter() {
        if !pids.is_empty() && !pids.contains(&problem.id) {
            continue;
        }

        let mut solutions = make_run_results(problem);
        let problem_time_start = time::Instant::now();
        for sln in solutions.iter_mut() {
            let sln_timeout_ms = timeout_ms + problem.extra_time_ms.as_millis() as u64;
            run_solution(sln, sln_timeout_ms, check_answers);
        }
        let problem_time = problem_time_start.elapsed();

        let (correct_count, total_count) = print_result(problem, &solutions, timeout_ms as f64, problem_time);

        count_solutions_succ += correct_count;
        count_solutions += total_count;

        if correct_count > 0 {
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

    println!(
        "Problems: {}/{} ({}%) , Solutions: {}/{}",
        problem_succ, problem_total, succ_rate, count_solutions_succ, count_solutions,
    );
    let time_cost = format!("{:.3} ms", elapsed_time).yellow();
    println!("Total time: {}", time_cost);
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

    let colors = [Color::Yellow, Color::Green, Color::Magenta, Color::Cyan];
    let problems = problems::all_problems();
    let mut j = 0;
    for problem in problems {
        if !pids.is_empty() && !pids.contains(&problem.id) {
            continue;
        }

        let pid = problem.id.to_string().blue();
        let title = problem.title.to_string().green();

        if problem.solutions.len() == 1 {
            let sln = &problem.solutions[0];
            let sln_name = format!("- {}", sln.name);
            let name = sln_name.color(colors[j % colors.len()]);
            println!("| {:>4} | {:<40} | {:<40} |", pid, title, name);
            count_solutions += 1;
            j += 1;
            continue;
        }

        let sln_first = format!("+-- {} solutions", problem.solutions.len());
        println!("| {:>4} | {:<40} | {:<40} |", pid, title, sln_first);
        j = 0;

        for sln in problem.solutions.iter() {
            let sln_name = format!("+- {}", sln.name);
            let name = sln_name.color(colors[j % colors.len()]);
            println!("| {:>4} | {:<40} | {:<40} |", "", "", name);
            count_solutions += 1;
            j += 1;
        }
        count_problems += 1;
    }

    println!("{}", sepline);
    println!(
        "found {} problems with {} solutions",
        count_problems.to_string().yellow(),
        count_solutions.to_string().yellow(),
    );
}

fn action_confirm(action: &str, accept_word: &str) -> bool {
    let mut result = true;

    print!("Type \"{}\" to {}: ", accept_word.green().bold(), action);
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();

    if line.trim() != accept_word {
        println!("aborted");
        result = false;
    }

    result
}

impl Problem {
    fn from_id(pid: i64) -> Problem {
        Problem {
            id: pid,
            title: "",
            answer: 0,
            extra_time_ms: std::time::Duration::from_millis(0),
            solutions: vec![],
        }
    }
}

fn print_action(action: &management::FileAction, path: &str) {
    println!("{:>8} {}", action.to_string(), path);
}

fn do_add(pid: i64, title: &str, answer: i64, sln_names: &[String], dry_run: bool) {
    let title_static: &'static str = Box::leak(Box::new(title.to_string()));
    let solutions: Vec<SolutionInfo> = sln_names
        .iter()
        .map(|name| {
            let name_static: &'static str = Box::leak(Box::new(name.clone()));
            SolutionInfo {
                name: name_static,
                entry: || 0 ,
            }
        })
        .collect();

    let problem = Problem {
        id: pid,
        title: title_static,
        answer,
        extra_time_ms: std::time::Duration::from_millis(0),
        solutions,
    };

    println!(" {:>12}: {}", "Problem ID", pid.to_string().green());
    println!(" {:>12}: {}", "Title", title.yellow());
    println!(" {:>12}: {}", "Answer", answer.to_string().magenta());
    println!(" {:>12}: {}", "Solutions", sln_names.join(", "));
    println!();

    let action_count = problem.do_add_actions(Some(print_action), true).unwrap();
    println!();

    if dry_run || action_count <= 0 {
        return;
    }

    if !action_confirm("create template files showed above", "yes") {
        return;
    }

    let callback = |action: &management::FileAction, path: &str| {
        println!("{:>8} {}", action.finish_string(), path);
    };
    let action_result = problem.do_add_actions(Some(callback), false);
    if action_result.is_err() {
        println!("failed to create problem: {}", action_result.err().unwrap());
    } else {
        println!("problem {} added successfully", pid.to_string().green().bold());
    }
}

fn do_delete(pids: Vec<i64>) {
    for pid in &pids {
        let problem = Problem::from_id(*pid);
        let action_list = problem.do_remove_actions(Some(print_action), true).unwrap();
        if action_list <= 0 {
            let target = format!("problem {}", pid.to_string().green().bold());
            println!("{:>8} {}", "SKIP".yellow().bold(), target);
            continue;
        }
    }

    if !action_confirm("delete files showed above", "yes") {
        return;
    }

    let callback = |action: &management::FileAction, path: &str| {
        println!("{:>8} {}", action.finish_string(), path);
    };

    for pid in &pids {
        let problem = Problem::from_id(*pid);
        let action_result = problem.do_remove_actions(Some(callback), false);
        if action_result.is_err() {
            println!(
                "failed to delete problem {}: {}",
                pid.to_string().green().bold(),
                action_result.err().unwrap(),
            );
        } else {
            println!(
                "problem {} deleted successfully",
                pid.to_string().green().bold(),
            );
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Run {
            pids,
            timeout_str,
            no_timeout,
            check_answers,
            always_color,
        } => {
            let timeout_ms = if no_timeout {
                0
            } else {
                match parse_duration(&timeout_str) {
                    Ok(ms) => ms,
                    Err(e) => {
                        eprintln!("invalid timeout '{}': {}", timeout_str, e);
                        return;
                    }
                }
            };
            if always_color {
                colored::control::set_override(true);
            }

            do_run(pids, timeout_ms, check_answers);
        }

        Command::List { pids } => do_list(pids),

        Command::Delete { pids } => do_delete(pids),

        Command::Add {
            pid,
            title,
            answer,
            dry_run,
            solutions,
        } => {
            let title_str = if title.is_empty() {
                format!("Problem {}", pid)
            } else {
                title
            };

            do_add(pid, &title_str, answer, &solutions, dry_run);
        }
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
