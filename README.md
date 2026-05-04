projeuler.rs
============

[![CI](https://github.com/flily/projeuler.rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/flily/projeuler.rs/actions/workflows/ci.yaml)
![GitHub](https://img.shields.io/github/license/flily/projeuler.rs)
![GitHub top language](https://img.shields.io/github/languages/top/flily/projeuler.rs)
![Solved problems](https://img.shields.io/github/directory-file-count/flily/projeuler.rs/src%2Fproblems?label=Solved)

Rust solutions for [Project Euler](https://projecteuler.net/) problems.
Each problem may include multiple solution approaches, to test and benchmark features of in Rust
language or between Rust and other languages.


Other related repositories
--------------------------
- [projeuler.py](https://github.com/flily/projeuler.py): Python solutions.
  ![GitHub top language](https://img.shields.io/github/languages/top/flily/projeuler.py)
  ![GitHub repo file count (file extension)](https://img.shields.io/github/directory-file-count/flily/projeuler.py/problems?label=Solved)
- [projeuler.go](https://github.com/flily/projeuler.go): Go solutions.
  ![GitHub top language](https://img.shields.io/github/languages/top/flily/projeuler.go)
  ![GitHub repo file count (file extension)](https://img.shields.io/github/directory-file-count/flily/projeuler.go/problems?label=Solved)


## Building

```bash
$ cargo build
$ cargo build --release
```

## Usage

```
$ cargo run -- <COMMAND> [OPTIONS] [PIDS...]
```

### Commands

| Command  | Description |
|----------|-------------|
| `run`    | Run solutions and print timing results |
| `list`   | List registered problems |
| `add`    | Scaffold template code for a new problem |
| `delete` | Delete all solution codes for one or more problems |
    
### `run` Command Options

| Flag | Short | Description |
|------|-------|-------------|
| `--timeout <TIMEOUT>` | `-t` | Timeout per solution, e.g. `1s` or `500ms` (default: `500ms`) |
| `--no-timeout` | `-o` | Disable the timeout limit |
| `--check` | `-c` | Verify answers after running |
| `--color` | | Force colour output even outside a terminal |
| `[PIDS...]` | | Optional list of problem IDs to run; omit to run all |

Cookbooks:
```bash
# run problem 1, 2, and 3
$ cargo run -- run 1 2 3

# run all problems and check answer
$ cargo run -- run --check

# run problem 42 with no timeout limit
$ cargo run -- run --no-timeout 42
```

### `add` Command Options

| Flag | Short | Description |
|------|-------|-------------|
| `<PID>` | | Problem ID (required, e.g. `100`) |
| `--title <TITLE>` | `-n` | Problem title (optional; defaults to `"Problem <PID>"`) |

## Adding a New Problem

Run the `add` command to template code for a new problem:

```bash
cargo run -- add <PID> [--title "<TITLE>"] [--answer <ANSWER>] [<SOLUTION_NAMES>...]
```

This creates:

| Path | Contents |
|------|----------|
| `src/problems/pXXXX/mod.rs` | `INFO` static with id, title, answer placeholder, and `naive` solution entry |
| `src/problems/pXXXX/naive.rs` | Stub `solve() -> i64` returning `0` |

It also patches `src/problems.rs` to add the `pub mod` declaration and register the problem in `all_problems()`, inserting both in sorted order by problem ID.

After scaffolding:

1. Implement the solution in `src/problems/pXXXX/naive.rs`.
2. Set the correct `answer` value in `src/problems/pXXXX/mod.rs`.
3. If the problem requires input data, place the file at `data/pXXXX.<ext>` and call `load_data()` from within the solution module.
4. Add extra solution modules as needed and register them in `mod.rs`.
