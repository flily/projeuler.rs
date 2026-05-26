use std::error;
use std::io::{self, BufRead, Read, Write};
use std::time::{self, Duration};
use std::net::{TcpListener, TcpStream};

use clap::{Parser, Subcommand};
use colored::{Color, Colorize};
use tokio::time::MissedTickBehavior::Skip;
use tokio::{time::timeout, runtime};

use crate::common::{Problem, SolutionInfo};
use crate::common::launcher;
use crate::common::launcher::ProblemSelection;
use crate::worker::message::{self, Message, MessageResult, ParsedMessage};
use crate::worker::{FinalResult, RunError, RunResult, Worker, messenger};
use management::ProblemManagement;

mod common;
mod problems;
mod management;
mod worker;

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
        /// run in local mode without starting a worker process.
        #[arg(short = 'l', long = "local", default_value_t = false)]
        local_mode: bool,
        /// port to start worker process and listen, default is 1707.
        #[arg(short = 'p', long = "port", default_value_t = 1707)]
        port: u16,
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
    Client {
        /// port to connect, default is 1707
        #[arg(short = 'p', long = "port", default_value_t = 1707)]
        port: u16,
        #[arg(long = "pid", required = true)]
        pid: i64,
        #[arg(long = "sid", required = true)]
        sid: i64,
    },
    Worker {
        #[arg(short = 'p', long = "port", default_value_t = 1707)]
        port: u16,
        #[arg(long = "pid", default_value_t = -1)]
        pid: i64,
        #[arg(long = "sid", default_value_t = 0)]
        solution_id: usize,
    },
}

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn simple_run_solution(run_result: &mut RunResult) {
    let entry = run_result.entry;

    let t1 = time::Instant::now();
    entry();
    run_result.cost = t1.elapsed();
}

async fn run_solution(worker: &Worker, problem_id: i64, index: usize, timeout_ms: u64, check_answer: bool) -> RunResult {
    let solution = worker.get_solution(problem_id, index).unwrap();
    let mut run_result = worker.make_result(problem_id, index).unwrap();

    let t1 = time::Instant::now();
    let task = tokio::task::spawn_blocking(move || {
        std::panic::catch_unwind(solution.entry)
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
                run_result.check();
            } else {
                run_result.result = FinalResult::Unknown;   // we got a result but not checked
            }
        }
        Err(true) => {
            run_result.result = FinalResult::Timeout;
        }
        Err(false) => {
            run_result.result = FinalResult::Crash;
        }
    }

    run_result
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

fn make_problem_result(result_list: &[RunResult]) -> (FinalResult, i32) {
    let mut result = FinalResult::Timeout;
    let mut best_index = -1;
    let mut best_time = time::Duration::from_secs(0);

    for (i, sln) in result_list.iter().enumerate() {
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
            FinalResult::Unknown => {} // skip Unknown result, not checked yet
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
    let pid = run_result.result.color_on(&problem.id.to_string().bold());
    let title = problem.title.on_color(run_result.result.color());
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
            pid, title.bold(), answer, result, cost, extra_timeout,
        ),
        _ => println!(
            "| {:>4} | {:<40} | {:^14} | {:^9} | {:>12} |{}",
            pid, title, answer, result, cost, extra_timeout,
        ),
    }
}

fn print_result(
    problem: &common::Problem,
    results: &[RunResult],
    timeout_ms: u64,
    cost: time::Duration,
) -> (i32, i32) {
    if results.len() == 1 {
        let sln = &results[0];
        print_one_solution_problem(problem, sln, timeout_ms);

        let c = match sln.result {
            FinalResult::Correct => (1, 1),
            _ => (0, 1),
        };

        return c;
    }

    let (problem_result, best_index) = make_problem_result(results);
    print_problem_result(problem, problem_result, timeout_ms, cost);

    let mut correct_count = 0;
    for (i, sln) in results.iter().enumerate() {
        if let FinalResult::Correct = sln.result {
            correct_count += 1;
        }
        print_solution_result(sln, timeout_ms, best_index == (i as i32));
    }

    (correct_count, results.len() as i32)
}

enum WorkMode {
    Local {
        rt: runtime::Runtime
    },
    Remote {
        client: messenger::Messenger,
        child: std::process::Child,
        port: u16,
        progname: String,
    },
}

struct RunContext {
    mode: WorkMode,
}

impl RunContext {
    pub fn local() -> Self {
        Self {
            mode: WorkMode::Local {
                rt: runtime::Builder::new_current_thread()
                    .enable_time()
                    .build()
                    .expect("failed to create tokio runtime"),
            },
        }
    }

    pub fn remote(progname: String, port: u16) -> Result<Self, std::io::Error> {
        let child = RunContext::launch_worker(&progname, port)?;
        std::thread::sleep(time::Duration::from_millis(100));

        let client = messenger::Messenger::connect(port)?;
        Ok(Self {
            mode: WorkMode::Remote { client, child, port, progname },
        })
    }

    pub fn launch_worker(progname: &String, port: u16) -> Result<std::process::Child, std::io::Error> {
        let child = std::process::Command::new(progname)
            .arg("worker")
            .arg("-p")
            .arg(port.to_string())
            .spawn()
            .expect("failed to start worker process");
        Ok(child)
    }

    pub fn reconnect(&mut self) -> Result<(), std::io::Error> {
        if let WorkMode::Remote { child,  port, progname, .. } = &mut self.mode {
            child.kill()?;

            let child = Self::launch_worker(progname, *port + 1)?;
            std::thread::sleep(time::Duration::from_millis(50));
            let client = messenger::Messenger::connect(*port + 1)?;
            self.mode = WorkMode::Remote {
                client,
                child,
                port: *port + 1,
                progname: progname.clone(),
            };
        }
        Ok(())
    }

    fn run(&mut self, worker: &Worker, problem_id: i64, solution_id: usize, timeout_ms: u64) -> RunResult {
        let result = match &mut self.mode {
            WorkMode::Local { rt } => {
                rt.block_on(run_solution(worker, problem_id, solution_id, timeout_ms, false))
            }
            WorkMode::Remote { client, .. } => {
                let timeout = if timeout_ms > 0 {
                    Some(Duration::from_millis(timeout_ms))
                } else {
                    None
                };
                match client.run(problem_id, solution_id, timeout) {
                    Ok(run_result) => {
                        let mut result = worker.make_result(problem_id, solution_id).unwrap();
                        result.got = Some(run_result.got.unwrap());
                        result.cost = run_result.cost;
                        result.result = FinalResult::Unknown;
                        result
                    }
                    Err(RunError::Timeout) => {
                        let _ = self.reconnect();

                        RunResult {
                            solution: worker.get_solution(problem_id, solution_id).unwrap().name.to_string(),
                            entry: worker.get_solution(problem_id, solution_id).unwrap().entry,
                            answer: Some(worker.get_problem(problem_id).unwrap().answer),
                            got: None,
                            result: FinalResult::Timeout,
                            cost: Duration::from_millis(timeout_ms),
                            extra_timeout_ms: worker.get_problem(problem_id).unwrap().extra_time_ms,
                        }
                    }
                    Err(e) => {
                        println!("Run error: {:?}", e);
                        RunResult::basic(0, time::Duration::from_secs(0))
                    }
                }
            }
        };

        result
    }

    fn shutdown(self) {
        match self.mode {
            WorkMode::Local { rt } => {
                rt.shutdown_background();
            }
            WorkMode::Remote { mut child, client, .. } => {
                let _ = child.kill();
                let _ = client.close();
            }
         }
    }
}

fn do_run(ctx: &mut RunContext, pids: Vec<ProblemSelection>, timeout_ms: u64, check_answers: bool) {
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
    let worker = Worker::on_static(problems::all_problems());
    for problem in worker.problems.iter() {
        if !pids.is_empty() && !pids.iter().any(|sel| sel.check(problem)) {
            continue;
        }

        // let mut solutions = problem.make_run_result_list();

        let mut results = Vec::<RunResult>::new();
        let problem_time_start = time::Instant::now();
        for (index, sln) in problem.solutions.iter().enumerate() {
            let flag = pids
                .iter()
                .any(|sel| sel.check(problem) && sel.check_solution(index, sln));
            if !pids.is_empty() && !flag {
                continue;
            }

            if sln.name.starts_with("_") {
                // skip solution with name start with "_", treat it as unfinished or not suggested to run.
                continue;
            }

            let sln_timeout_ms = timeout_ms + problem.extra_time_ms;
            let mut run_result = ctx.run(&worker, problem.id, index, sln_timeout_ms);
            if check_answers {
                run_result.check();
            }
            
            results.push(run_result);
        }
        let problem_time = problem_time_start.elapsed();

        let (correct_count, total_count) = print_result(problem, &results, timeout_ms, problem_time);

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

fn worker_test(worker: &Worker, pid: i64, solution_id: usize) {
    let result = worker.run(pid, solution_id);
    if result.is_err() {
        println!("run_solution error: {:?}", result.err().unwrap());
        return;
    }

    let run_result = result.unwrap();
    println!("run_solution: {:?}", run_result);
}

fn do_worker(port: u16, pid: i64, solution_id: usize) {
    let listener = messenger::MessengerListener::listen(port)
        .expect(&format!("failed to bind port {}", port));

    let worker = Worker::on_static(problems::all_problems());

    if pid > 0 {
        worker_test(&worker, pid, solution_id);
        return;
    }

    let mut conn = listener.accept()
        .expect("failed to accept connection");

    loop {
        let r = conn.recv();
        match r {
            Ok(ParsedMessage::Ping(msg)) => {
                let pong = msg.to_pong();
                conn.send(&pong).expect("send pong failed")
            }
            Ok(ParsedMessage::Run(msg)) => {
                let pid = msg.problem_id as i64;
                let sid = msg.solutions_id as usize;
                let result = worker.run(pid, sid);
                let response = match result {
                    Ok(run_result) => {
                        msg.reply(run_result.cost, run_result.answer.unwrap(), message::MessageResultFlags::NONE)
                    }
                    Err(RunError::ProblemNotFound { problem_id }) => {
                        MessageResult::problem_not_found(problem_id as i32)
                    }
                    Err(RunError::SolutionNotFound { problem_id, solution_id }) => {
                        MessageResult::solution_not_found(problem_id as i32, solution_id as i32)
                    }
                    Err(_) => {
                        // impossible
                        MessageResult::problem_not_found(0)
                    }
                };
                conn.send(&response).unwrap();
            }
            Ok(_) => {
                println!("Received unexpected message");
                break;
            }
            Err(e) => {
                println!("Failed to receive message: {:?}", e);
                break;
            }
        }
    }
}

fn do_client(port: u16, pid: i64, sid: i64) {
    let mut m = messenger::Messenger::connect(port)
        .expect(&format!("failed to connect to {}", port));

    m.ping().expect("ping failed");

    let result = m.run(pid, sid as usize, None);
    match result {
        Ok(run_result) => {
            println!("Run result: {:?}", run_result);
        }
        Err(e) => {
            println!("Run error: {:?}", e);
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
            local_mode,
            port,
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

            let mut ctx = if local_mode {
                println!("running in local mode, solutions will run in the same process...");
                RunContext::local()
            } else {
                println!("starting worker process on port {}...", port);
                let progname = std::env::current_exe().unwrap();
                RunContext::remote(progname.to_str().unwrap().to_string(), port).expect("")
            };

            do_run(&mut ctx, solution_selection, timeout_ms, check_answers);
            ctx.shutdown();
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
        Command::Worker {
            port,
            pid,
            solution_id,
        } => {
            do_worker(port, pid, solution_id);
        }
        Command::Client {
            port,
            pid,
            sid,
        } => {
            do_client(port, pid, sid);
        }
    }
}
