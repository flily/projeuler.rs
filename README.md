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


Build & Run
-----------
build and run the project with Cargo
```bash
$ cargo build
$ cargo run -- run --check
```

In some cases, binary builds in `debug` mode is SUPER slow, it can be much slower than naive
algorithms or implementations in CPython, (especially for problems using `HashSet`), run it in
`release` mode to get a reasonable time measurement to check algorithm designs and implementations.
```bash
$ cargo build --release
$ cargo run --release -- run --check
```

Usage
-----

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

Adding a New Problem
--------------------

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


Performance Issues
------------------

### `HashSet` and `HashMap` Performance

By default, Rust's standard library `HashSet` and `HashMap` use a secure hashing algorithm (SipHash)
that is designed to prevent DoS attacks. But solutions use `HashSet` or `HashMap` for intermediate
result caching can be much more slower than expected, even slower than naive algorithms or
implementations in CPython. In example of problem 92, I tested some alternative hash set
implementations and hashers:
  - Rust standard library `HashSet`, `std::collections::HashSet`, with capacity 10M.
  - `FxHashSet` from `rustc-hash` crate.
  - `AHashSet` from `ahash` crate.
  - `RapidHashSet` from `rapidhash` crate.
  - A `Vec[u8]` with 10M size as a linear-probing hash set.
  - `dict` in python, with the same solution, runs in CPython 3.14 and PyPy 7.3 (Python 3.10 compatible).

| Implementation               | Rust(dev) / CPython | ratio | Rust(rel)/ Pypy | ratio |
|------------------------------|-------------:|-------:|-------------:|-------:|
| naive (Rust, no cache)       |  1123.218 ms |  1.00x |   268.231 ms |  1.00x |
| naive (Python, no cache)     | 16810.059 ms | 14.97x |  2471.337 ms |  9.22x |
| `HashSet` (Pre-alloc)        | 12583.942 ms | 11.20x |  1144.188 ms |  4.27x |
| `FxHashSet`                  |  6312.936 ms |  5.62x |   325.469 ms |  1.21x |
| `Vec[u8]` (linear-probing)   |   370.493 ms |  0.33x |    69.456 ms |  0.26x |
| `AHashSet`                   |  6313.811 ms |  5.62x |   436.396 ms |  1.63x |
| `RapidHashSet`               |  6721.838 ms |  5.98x |   417.565 ms |  1.56x |
| `dict` with Python           |  6218.837 ms |  5.54x |  1463.929 ms |  5.46x |

Base on test results above, solutions only use `FxHashSet` and `FxHashMap` from `rustc-hash` crate
for now, other hash set implementation crates are not imported by default.
