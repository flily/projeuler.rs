use std::io::{self, BufRead, Write};
use std::time;

use clap::{Parser, Subcommand};
use colored::{Color, Colorize};
use tokio::{time::timeout, runtime};

use crate::common::{Problem, SolutionInfo};
use crate::common::launcher;
use crate::common::launcher::{RunResult, FinalResult, ProblemSelection};
use management::ProblemManagement;

mod common;
mod problems;
mod management;

const DEFAULT_TIMEOUT_MS: u64 = 500;

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
        pids: Vec<String>,
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
        /// Accept the action without confirmation. If not set, the tool will show the files to be created and ask for confirmation before creating them.
        #[arg(short = 'y', long = "yes", default_value_t = false)]
        auto_confirm: bool,
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
        /// delete all files related to the problem, and update problems.rs. If not set, only
        /// remove index in problems.rs and keep the solution files.
        #[arg(long = "full", default_value_t = false)]
        full_delete: bool,
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


fn make_run_results(info: &common::Problem) -> Vec<RunResult> {
    info.solutions
        .iter()
        .map(|sln| RunResult {
            solution: sln.name,
            entry: sln.entry,
            answer: Some(info.answer),
            got: None,
            result: FinalResult::None, // default to None, will be updated after running
            cost: time::Duration::from_millis(0),
            extra_timeout_ms: info.extra_time_ms,
        })
        .collect()
}

fn simple_run_solution(run_result: &mut RunResult) {
    let entry = run_result.entry;

    let t1 = time::Instant::now();
    entry();
    run_result.cost = t1.elapsed();
}

async fn run_solution(run_result: &mut RunResult, timeout_ms: u64, check_answer: bool) {
    let entry = run_result.entry;

    let t1 = time::Instant::now();
    let task = tokio::task::spawn_blocking(move || {
        std::panic::catch_unwind(entry)
    });

    // Ok(val)    => completed successfully
    // Err(false) => crashed (panic)
    // Err(true)  => timed out
    let response: Result<i64, bool> = if timeout_ms == 0 {
        match task.await {
            Ok(Ok(val)) => Ok(val),
            _ => Err(false),
        }
    } else {
        let run_timeout = time::Duration::from_millis(timeout_ms);
        match timeout(run_timeout, task).await {
            Ok(Ok(Ok(val))) => Ok(val),
            Ok(_) => Err(false),
            Err(_) => Err(true),
        }
    };
    run_result.cost = t1.elapsed();

    match response {
        Ok(got) => {
            run_result.got = Some(got);
            if check_answer {
                run_result.result = if run_result.check(got) {
                    FinalResult::Correct
                } else {
                    FinalResult::Wrong
                };
            }
        }
        Err(true) => {
            run_result.result = FinalResult::Timeout;
        }
        Err(false) => {
            run_result.result = FinalResult::Crash;
        }
    }
}

fn cost_time_color(cost: time::Duration, timeout_ms: u64) -> colored::Color {
    let cost_ms = cost.as_nanos() as f64 / 1_000_000.0;
    let total_timeout = if timeout_ms > 0 { timeout_ms as f64 } else { DEFAULT_TIMEOUT_MS as f64 };
    let prop = cost_ms / total_timeout;

    if prop < 0.1 {
        Color::Green
    } else if prop < 0.2 {
        Color::Blue
    } else if prop < 0.3 {
        Color::Cyan
    } else if prop < 0.5 {
        Color::Yellow
    } else if prop < 0.8 {
        Color::Magenta
    } else {
        Color::Red
    }
}

fn color_cost_time(cost: time::Duration, color: colored::Color, is_best: bool) -> colored::ColoredString {
    let s = if cost < time::Duration::from_micros(1) {
        format!(">>  {:4.3} μs", cost.as_nanos() / 1_000)
    } else {
        format!("{:8.3} ms", cost.as_nanos() as f64 / 1_000_000.0)
    };

    if is_best {
        s.on_color(color).bold()
    } else {
        s.color(color)
    }
}

fn print_problem_result(problem: &common::Problem, problem_result: FinalResult, timeout_ms: u64, cost: time::Duration) {
    let total_timeout = (problem.solutions.len() as u64) * (timeout_ms + problem.extra_time_ms);

    let pid = problem_result.color_on(&problem.id.to_string());
    let title = problem_result.color_on(problem.title);
    let cost_color = cost_time_color(cost, total_timeout);
    let cost = color_cost_time(cost, cost_color, false);
    let result = problem_result.color_string();

    println!(
        "| {:>4} | {:<40} | {:^14} | {:^9} | {:>12} |",
        pid, title, "", result, cost,
    );
}

fn print_solution_result(run_result: &RunResult, timeout_ms: u64, is_best: bool) {
    let result = run_result.result.color_string();
    let solution = if is_best {
        format!("* {}", run_result.solution).on_color(run_result.result.color()).bold()
    } else {
        format!("+ {}", run_result.solution).color(run_result.result.color())
    };
    let cost_color = cost_time_color(run_result.cost, timeout_ms);
    let cost = match run_result.result {
        FinalResult::None => "-   ".yellow(),
        _ => color_cost_time(run_result.cost, cost_color, is_best),
    };
    let answer = if let Some(got) = run_result.got {
        got.to_string().color(run_result.result.color())
    } else {
        "NO RESULT".red()
    };
    let extra_timeout = if run_result.extra_timeout_ms > 0 {
        format!(" [+ {} ms]", run_result.extra_timeout_ms).yellow()
    } else {
        "".into()
    };

    println!(
        "| {:>4} | {:<40} | {:^14} | {:^9} | {:>12} |{}",
        "", solution, answer, result, cost, extra_timeout,
    );
}

fn make_problem_result(solutions: &[RunResult]) -> (FinalResult, i32) {
    let mut result = FinalResult::Timeout;
    let mut best_index = -1;
    let mut best_time = time::Duration::from_secs(0);

    for (i, sln) in solutions.iter().enumerate() {
        match sln.result {
            FinalResult::Timeout => {}
            FinalResult::Correct => {
                result = FinalResult::Correct;
                if sln.cost < best_time || best_index < 0 {
                    best_time = sln.cost;
                    best_index = i as i32;
                }
            }
            FinalResult::None => {} // skip None result, not run yet
            _ => {
                result = sln.result.clone();
                break;
            }
        }
    }

    (result, best_index)
}

fn print_one_solution_problem(problem: &common::Problem, run_result: &RunResult, timeout_ms: u64) {
    let total_timeout = timeout_ms + problem.extra_time_ms;
    let pid = run_result.result.color_on(&problem.id.to_string());
    let title = problem.title;
    let result = run_result.result.color_string();
    let cost_color = cost_time_color(run_result.cost, total_timeout);
    let cost = color_cost_time(run_result.cost, cost_color, matches!(run_result.result, FinalResult::Correct));
    let answer = if let Some(got) = run_result.got {
        got.to_string().color(run_result.result.color())
    } else {
        "NO RESULT".red()
    };
    let extra_timeout = if run_result.extra_timeout_ms > 0 {
        format!(" [+ {} ms]", run_result.extra_timeout_ms).yellow()
    } else {
        "".into()
    };

    match run_result.result {
         FinalResult::Correct => println!(
            "| {:>4} | {:<40} | {:^14} | {:^9} | {:>12} |{}",
            pid, title.on_color(run_result.result.color()).bold(), answer, result, cost, extra_timeout,
        ),
        _ => println!(
            "| {:>4} | {:<40} | {:^14} | {:^9} | {:>12} |{}",
            pid, title, answer, result, cost, extra_timeout,
        ),
    }
}

fn print_result(
    problem: &common::Problem,
    solutions: &[RunResult],
    timeout_ms: u64,
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
    print_problem_result(problem, problem_result, timeout_ms, cost);

    let mut correct_count = 0;
    for (i, sln) in solutions.iter().enumerate() {
        if let FinalResult::Correct = sln.result {
            correct_count += 1;
        }
        print_solution_result(sln, timeout_ms, best_index == (i as i32));
    }

    (correct_count, solutions.len() as i32)
}

fn do_run(pids: Vec<ProblemSelection>, timeout_ms: u64, check_answers: bool) {
    let rt = runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("failed to create tokio runtime");
    let sepline = "+".to_string()
        + &"-".repeat(4 + 2) + "+"      // PID
        + &"-".repeat(40 + 2) + "+"     // Title
        // + &"-".repeat(40 + 2) + "+"     // Solution
        + &"-".repeat(14 + 2) + "+"     // Answer
        + &"-".repeat(9 + 2) + "+"      // Result
        + &"-".repeat(12 + 2) + "+"     // Time 12345.678 ms
    ;

    println!("{}", sepline);
    println!(
        "| {:>4} | {:<40} | {:^14} | {:^9} | {:>12} |",
        "PID", "Title / Solution", "Answer", "Result", "Time",
    );
    println!("{}", sepline);

    let mut count_problems = 0;
    let mut count_problems_succ = 0;
    let mut count_solutions = 0;
    let mut count_solutions_succ = 0;

    let start_time = time::Instant::now();
    for problem in problems::all_problems().iter() {
        if !pids.is_empty() && !pids.iter().any(|sel| sel.check(problem)) {
            continue;
        }

        let mut solutions = make_run_results(problem);
        let problem_time_start = time::Instant::now();
        for (index, sln) in solutions.iter_mut().enumerate() {
            let flag = pids
            .iter()
            .any(|sel| sel.check(problem) && sel.check_run_result(index, sln));
            if !pids.is_empty() && !flag {
                continue;
            }

            let sln_timeout_ms = timeout_ms + problem.extra_time_ms;
            rt.block_on(run_solution(sln, sln_timeout_ms, check_answers));
            // run correct solutions again to get more accurate time cost.
            match sln.result {
                FinalResult::Correct if sln.cost < time::Duration::from_micros(5) => {
                    // only solutions use very short time can make big difference.
                    simple_run_solution(sln);
                }
                _ => {}
            }
        }
        let problem_time = problem_time_start.elapsed();

        let (correct_count, total_count) = print_result(problem, &solutions, timeout_ms, problem_time);

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
        "Problems: {}/{} ({}%) , Solutions: {}/{}, Solution timeout: {} ms",
        problem_succ, problem_total, succ_rate, count_solutions_succ, count_solutions, timeout_ms,
    );
    let time_cost = format!("{:.3} ms", elapsed_time).yellow();
    println!("Total time: {}", time_cost);

    rt.shutdown_background();
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
            let sln_name = format!("+ {}", sln.name);
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

fn print_action(action: &management::FileAction, path: &str) {
    println!("{:>8} {}", action.to_string(), path);
}

fn do_add(pid: i64, title: &str, answer: i64, sln_names: &[String], auto_confirm: bool, dry_run: bool) {
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

    let mut problem = Problem {
        id: pid,
        title: title_static,
        answer,
        extra_time_ms: 0,
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

    if action_count > 1 && !auto_confirm && !action_confirm("create template files showed above", "yes") {
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

fn do_delete(pids: Vec<i64>, full_delete: bool) {
    let mut total_actions = 0;
    let mut update_index_only = true;
    for pid in &pids {
        let problem = Problem::from_id(*pid);
        let action_count = problem.do_remove_actions(Some(print_action), full_delete, true).unwrap();
        total_actions += action_count;
        if action_count > 1 {
            update_index_only = false;
        }
        if action_count <= 0 {
            let target = format!("problem {}", pid.to_string().green().bold());
            println!("{:>8} {}", "SKIP".yellow().bold(), target);
            continue;
        }
    }

    if total_actions <= 0 {
        return;
    }

    if !update_index_only && !action_confirm("delete files showed above", "yes") {
        return;
    }

    let callback = |action: &management::FileAction, path: &str| {
        println!("{:>8} {}", action.finish_string(), path);
    };

    for pid in &pids {
        let problem = Problem::from_id(*pid);
        let action_result = problem.do_remove_actions(Some(callback), full_delete, false);
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
                match launcher::parse_duration(&timeout_str) {
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

            let mut solution_selection = Vec::new();
            for pid_str in pids {
                let sel = ProblemSelection::parse(&pid_str).unwrap();
                solution_selection.push(sel);
            }

            do_run(solution_selection, timeout_ms, check_answers);
        }

        Command::List { pids } => do_list(pids),

        Command::Delete {
            pids,
            full_delete } => do_delete(pids, full_delete),

        Command::Add {
            pid,
            title,
            answer,
            auto_confirm,
            dry_run,
            solutions,
        } => {
            let title_str = if title.is_empty() {
                format!("Problem {}", pid)
            } else {
                title
            };

            do_add(pid, &title_str, answer, &solutions, auto_confirm, dry_run);
        }
    }
}
