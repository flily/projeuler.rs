use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

use colored::Colorize;

use regex::Regex;

use crate::common::{Problem, SolutionInfo};

pub enum FileAction {
    MakeDir,
    CreateMod,
    CreateFile,
    Remove,
    RemoveDir,
    UpdateIndex,
}

pub type FileActionCallback = fn(action: &FileAction, path: &str);

impl FileAction {
    pub fn to_string(&self) -> colored::ColoredString {
        match self {
            FileAction::MakeDir => "MKDIR".green().bold(),
            FileAction::CreateMod => "CREATE".green().bold(),
            FileAction::CreateFile => "CREATE".green().bold(),
            FileAction::Remove => "RM".red().bold(),
            FileAction::RemoveDir => "RMDIR".red().bold(),
            FileAction::UpdateIndex => "UPDATE".yellow().bold(),
        }
    }

    pub fn finish_string(&self) -> colored::ColoredString {
        match self {
            FileAction::MakeDir => "MKDIR".green().bold(),
            FileAction::CreateMod => "NEW".green().bold(),
            FileAction::CreateFile => "NEW".green().bold(),
            FileAction::Remove => "REMOVED".red().bold(),
            FileAction::RemoveDir => "REMOVED".red().bold(),
            FileAction::UpdateIndex => "UPDATED".yellow().bold(),
        }
    }
}

struct ProblemIndex {
    id_list: Vec<i64>,
}

static PROBLEM_INDEX_REGEX_IMPORT: &str = r"^pub mod p(\d+);$";
static PROBLEM_INDEX_REGEX_INFO: &str = r"^\s+&p(\d+)::INFO,$";

impl ProblemIndex {
    fn load(filename: &str) -> Result<Self, io::Error> {
        let mut id_list = Vec::<i64>::new();
        let file = fs::File::open(filename)?;
        let reader = io::BufReader::new(file);

        let regex_import = Regex::new(PROBLEM_INDEX_REGEX_IMPORT).unwrap();
        let regex_info = Regex::new(PROBLEM_INDEX_REGEX_INFO).unwrap();
        for line in reader.lines() {
            let line = line?;
            if let Some(captures) = regex_import.captures(&line) {
                let id_str = captures.get(1).unwrap().as_str();
                let id: i64 = id_str.parse().unwrap();
                id_list.push(id);
                id_list.sort();

            } else if let Some(captures) = regex_info.captures(&line) {
                let id_str = captures.get(1).unwrap().as_str();
                let id: i64 = id_str.parse().unwrap();

                if id_list.binary_search(&id).is_err() {
                    id_list.push(id);
                    id_list.sort();
                }
            }
        }

        Ok(ProblemIndex { id_list })
    }

    fn write(&mut self, filename: &str) -> Result<(), io::Error> {
        self.id_list.sort();

        let mut lines = Vec::<String>::with_capacity(2 * self.id_list.len() + 8);
        for id in &self.id_list {
            lines.push(format!("pub mod p{:04};", id));
        }

        lines.push(String::from(""));
        lines.push(String::from("use crate::common::Problem;"));
        lines.push(String::from(""));
        lines.push(String::from("pub fn all_problems() -> Vec<&'static Problem> {"));
        lines.push(String::from("    vec!["));
        for id in &self.id_list {
            lines.push(format!("        &p{:04}::INFO,", id));
        }
        lines.push(String::from("    ]"));
        lines.push(String::from("}"));
        lines.push(String::from(""));

        let content = lines.join("\n");
        let mut file = fs::File::create(filename)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    fn add(&mut self, id: i64) {
        if self.id_list.binary_search(&id).is_err() {
            self.id_list.push(id);
            self.id_list.sort();
        }
    }

    fn remove(&mut self, id: i64) {
        if let Ok(pos) = self.id_list.binary_search(&id) {
            self.id_list.remove(pos);
        }
    }

    fn contain(&self, id: i64) -> bool {
        self.id_list.binary_search(&id).is_ok()
    }
}

pub trait ProblemManagement {
    fn dir_name(&self) -> String;
    fn mod_filename(&self) -> String;
    fn solution_filename(&self, sln_name: &str) -> String;
    fn index_filename(&self) -> String;

    fn create_problem_directory(&self) -> Result<(), io::Error>;
    fn create_problem_mod(&self) -> Result<(), io::Error>;
    fn create_solution_file(&self, sln_name: &str) -> Result<(), io::Error>;

    fn do_add_actions(
        &mut self,
        callback: Option<FileActionCallback>,
        dry_run: bool,
    ) -> Result<i32, io::Error>;
    fn do_remove_actions(
        &self,
        callback: Option<FileActionCallback>,
        full_delete: bool,
        dry_run: bool,
    ) -> Result<i32, io::Error>;
}

static MODNAME_FIRST: [&str; 1] = ["naive"];
static MODULE_INDEX: &str = "src/problems.rs";

impl ProblemManagement for Problem {
    fn dir_name(&self) -> String {
        format!("src/problems/p{:04}", self.id)
    }

    fn mod_filename(&self) -> String {
        format!("src/problems/p{:04}/mod.rs", self.id)
    }

    fn solution_filename(&self, sln_name: &str) -> String {
        format!("src/problems/p{:04}/{}.rs", self.id, sln_name)
    }

    fn index_filename(&self) -> String {
        MODULE_INDEX.to_string()
    }

    fn create_problem_directory(&self) -> Result<(), io::Error> {
        let problem_dir = self.dir_name();
        fs::create_dir_all(&problem_dir)
    }

    fn create_problem_mod(&self) -> Result<(), io::Error> {
        let problem_mod_filename = self.mod_filename();

        let mut lines = Vec::<String>::new();
        lines.push("use crate::common::{Problem, SolutionInfo};".to_string());
        lines.push("".to_string());

        for sln in &self.solutions {
            if MODNAME_FIRST.contains(&sln.name) {
                lines.push(format!("mod {};", sln.name));
            }
        }

        for sln in &self.solutions {
            if !MODNAME_FIRST.contains(&sln.name) {
                lines.push(format!("mod {};", sln.name));
            }
        }

        let mut info = vec![
            "".to_string(),
            "pub static INFO: std::sync::LazyLock<Problem> = std::sync::LazyLock::new(|| Problem {".to_string(),
            format!("    id: {},", self.id),
            format!("    title: \"{}\",", self.title),
            format!("    answer: {},", self.answer),
            "    extra_time_ms: std::time::Duration::from_millis(0),".to_string(),
            "    solutions: vec![".to_string(),
        ];

        for sln in &self.solutions {
            info.push("        SolutionInfo {".to_string());
            info.push(format!("            name: \"{}\",", sln.name));
            info.push(format!("            entry: {}::solve,", sln.name));
            info.push("        },".to_string());
        }

        info.push("    ],".to_string());
        info.push("});".to_string());
        info.push("".to_string());

        lines.extend(info);

        let content = lines.join("\n");
        let mut file = fs::File::create(&problem_mod_filename)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    fn create_solution_file(&self, sln_name: &str) -> Result<(), io::Error> {
        let sln_filename = self.solution_filename(sln_name);
        let mut file = fs::File::create(&sln_filename)?;
        let content = [
            "// This is a template solution file. You can modify it as needed.",
            "",
            "pub fn solve() -> i64 {",
            "    0",
            "}",
            "",
        ]
        .join("\n");
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn do_add_actions(
        &mut self,
        callback: Option<FileActionCallback>,
        dry_run: bool,
    ) -> Result<i32, io::Error> {
        let mut count = 0;
        let problem_dir = self.dir_name();

        // check problem directory
        let problem_dir_path = Path::new(&problem_dir);
        if !problem_dir_path.exists() {
            count += 1;
            if let Some(cb) = callback {
                cb(&FileAction::MakeDir, &problem_dir);
            }

            if !dry_run {
                self.create_problem_directory()?;
            }
        }

        // check mod.rs
        let problem_mod_filename = self.mod_filename();
        let problem_mod_path = Path::new(&problem_mod_filename);
        let create_mod = !problem_mod_path.exists();
        if create_mod {
            count += 1;
            if let Some(cb) = callback {
                cb(&FileAction::CreateMod, &problem_mod_filename);
            }

            if self.solutions.is_empty() {
                self.solutions.push(SolutionInfo {
                    name: "naive",
                    entry: || 0,
                });
            }

            if !dry_run {
                self.create_problem_mod()?;
            }
        }

        // check solution files
        for sln in &self.solutions {
            let sln_filename = self.solution_filename(sln.name);
            let sln_path = Path::new(&sln_filename);
            if !sln_path.exists() {
                count += 1;
                if let Some(cb) = callback {
                    cb(&FileAction::CreateFile, &sln_filename);
                }
                if !dry_run {
                    self.create_solution_file(sln.name)?;
                }
            }
        }

        let index_filename = self.index_filename();
        let mut index = ProblemIndex::load(&index_filename)?;
        if !index.contain(self.id) {
            count += 1;
            if let Some(cb) = callback {
                cb(&FileAction::UpdateIndex, &index_filename);
            }
            if !dry_run {
                index.add(self.id);
                index.write(&index_filename)?;
            }
        }

        Ok(count)
    }

    fn do_remove_actions(
        &self,
        callback: Option<FileActionCallback>,
        full_delete: bool,
        dry_run: bool,
    ) -> Result<i32, io::Error> {
        let mut count = 0;
        let problem_dir = self.dir_name();

        // check index file
        let index_filename = self.index_filename();
        let mut index = ProblemIndex::load(&index_filename)?;
        if index.contain(self.id) {
            count += 1;
            if let Some(cb) = callback {
                cb(&FileAction::UpdateIndex, &index_filename);
            }
            if !dry_run {
                index.remove(self.id);
                index.write(&index_filename)?;
            }
        }

        if !full_delete {
            return Ok(count);
        }

        // check problem directory
        let problem_dir_path = Path::new(&problem_dir);
        if !problem_dir_path.exists() {
            return Ok(count);
        }

        for entry in fs::read_dir(&problem_dir)?.flatten() {
            let path = entry.path();
            if path.is_file() {
                count += 1;
                if let Some(cb) = callback {
                    cb(&FileAction::Remove, path.to_str().unwrap());
                }
                if !dry_run {
                    fs::remove_file(path)?;
                }
            } else if path.is_dir() {
                count += 1;
                if let Some(cb) = callback {
                    cb(&FileAction::RemoveDir, path.to_str().unwrap());
                }
                if !dry_run {
                    fs::remove_dir_all(path)?;
                }
            }
        }

        count += 1;
        if let Some(cb) = callback {
            cb(&FileAction::RemoveDir, &problem_dir);
        }
        if !dry_run {
            fs::remove_dir_all(&problem_dir)?;
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_import_success() {
        let regex_import = Regex::new(PROBLEM_INDEX_REGEX_IMPORT).unwrap();
        let line = "pub mod p1234;";
        assert!(regex_import.is_match(line));

        let captures = regex_import.captures(line).unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "1234");
    }

    #[test]
    fn test_regex_import_failure() {
        let regex_import = Regex::new(PROBLEM_INDEX_REGEX_IMPORT).unwrap();
        let line = "use crate::common::Problem;";
        assert!(!regex_import.is_match(line));
    }

    #[test]
    fn test_regex_info_success() {
        let regex_info = Regex::new(PROBLEM_INDEX_REGEX_INFO).unwrap();
        let line = "        &p1234::INFO,";
        assert!(regex_info.is_match(line));

        let captures = regex_info.captures(line).unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "1234");
    }

    #[test]
    fn test_regex_info_failure() {
        let regex_info = Regex::new(PROBLEM_INDEX_REGEX_INFO).unwrap();
        let line = "pub mod p1234;";
        assert!(!regex_info.is_match(line));
    }
}
