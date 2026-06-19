use super::problem::{Problem, SolutionItem};

pub fn parse_duration(s: &str) -> Result<u64, String> {
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

#[derive(Debug, PartialEq)]
pub struct SelectionInfo {
    pub name: String,
    pub num: Option<i64>,
}

impl SelectionInfo {
    pub fn from<T: AsRef<str>>(s: T) -> Self {
        let s = s.as_ref().trim().to_lowercase();
        if let Ok(num) = s.parse() {
            SelectionInfo {
                name: s.to_string(),
                num: Some(num),
            }
        } else {
            SelectionInfo {
                name: s.to_string(),
                num: None,
            }
        }
    }
    
    pub fn contains_i64(&self, n: i64) -> bool {
        if let Some(num) = self.num {
            num == n
        } else {
            false
        }
    }

    pub fn contains_str<T: AsRef<str>>(&self, s: T) -> bool {
        s.as_ref().to_lowercase().contains(&self.name)
    }
}

#[derive(Debug, PartialEq)]
pub struct ProblemSelection {
    pub id_title: SelectionInfo,
    pub solutions: Vec<SelectionInfo>,
}

#[derive(Debug)]
pub struct SelectionError {
    pub raw: String,
    pub message: String,
    pub position: (usize, usize),
}

impl std::fmt::Display for SelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start, end) = self.position;
        let spaces = " ".repeat(start);
        let pointers = "^".repeat(end - start);
        writeln!(f, "{}", self.raw)?;
        writeln!(f, "{}{}", spaces, pointers)?;
        writeln!(f, "{}{}", spaces, self.message)
    }
}

impl SelectionError {
    pub fn new<T1: AsRef<str>, T2: AsRef<str>>(raw: T1, position: (usize, usize), message: T2) -> Self {
        SelectionError { 
            raw: raw.as_ref().to_string(),
            message: message.as_ref().to_string(),
            position,
        }
    }
}

fn parse_string_info(s: &[char], start: usize) -> (String, usize) {
    let mut i = start;
    while i < s.len() && s[i].is_alphanumeric() {
        i += 1;
    }
    let info = s[start..i].iter().collect();
    (info, i)
}

fn parse_problem_selection_solutions(s: &[char], start: usize, title: &str) -> Result<ProblemSelection, SelectionError> {
    let mut solutions = Vec::new();
    let mut i = start;
    let mut closed = false;
    while i < s.len() {
        let (sol, next) = parse_string_info(s, i);
        solutions.push(SelectionInfo::from(sol));

        if next <= i {
            let position = (i, i + 1);
            let message = "expected solution name".to_string();
            return Err(SelectionError::new(s.iter().collect::<String>(), position, message));
        }
        
        i = next + 1;
        if next < s.len() {
            let c = s[next];
            if c == ',' {
            } else if c == '}' {
                closed = true;
                break;
            } else {
                let position = (next, next + 1);
                let message = format!("unexpected character '{}'", c);
                return Err(SelectionError::new(s.iter().collect::<String>(), position, message));
            }
        } else {
            let position = (next, next + 1);
            let message = "expected ',' or '}' after solution name".to_string();
            return Err(SelectionError::new(s.iter().collect::<String>(), position, message));
        }
        
    }

    if !closed {
        let position = (i, i + 1);
        let message = "expected '}' to close solution collection".to_string();
        return Err(SelectionError::new(s.iter().collect::<String>(), position, message));
    }

    Ok(ProblemSelection {
        id_title: SelectionInfo::from(title),
        solutions,
    })
}

fn parse_problem_selection_with_solutions(s: &[char], start: usize, title: &str) -> Result<ProblemSelection, SelectionError> {
    if start < s.len() {
        let c = s[start];
        match c {
            '{' => {
               parse_problem_selection_solutions(s, start + 1, title)
            }
            c if c.is_alphanumeric() => {
                let (solution_name, next) = parse_string_info(s, start);
                if next < s.len() {
                    let position = (next, s.len());
                    let content = s[next..].iter().collect::<String>();
                    let message = format!("unexpected content '{}' after solution name", content);
                    Err(SelectionError::new(s.iter().collect::<String>(), position, message))
                } else {
                    Ok(ProblemSelection {
                        id_title: SelectionInfo::from(title),
                        solutions: vec![SelectionInfo::from(solution_name)],
                    })
                }
            }
            _ => {
                let position = (start, start + 1);
                let message = format!("unexpected character '{}'", c);
                Err(SelectionError::new(s.iter().collect::<String>(), position, message))
            }
        }
    } else {
        let position = (start, start + 1);
        let message = "expected solution name or collection after '.'".to_string();
        Err(SelectionError::new(s.iter().collect::<String>(), position, message))
    }
}

fn parse_problem_selection<T: AsRef<str>>(s: T) -> Result<ProblemSelection, SelectionError> {
    let chars: Vec<char> = s.as_ref().chars().collect();

    let (title, next) = parse_string_info(&chars, 0);
    if next < chars.len() {
        let c = chars[next];
        match c {
            '.' => parse_problem_selection_with_solutions(&chars, next + 1, &title),
            _ => {
                let position = (next, next + 1);
                let message = format!("unexpected character '{}'", c);
                Err(SelectionError::new(s, position, message))
            }
        }
    } else {
        Ok(ProblemSelection {
            id_title: SelectionInfo::from(title),
            solutions: Vec::new(),
        })
    }
}

impl ProblemSelection {
    pub fn parse<T: AsRef<str>>(s: T) -> Result<Self, SelectionError> {
        parse_problem_selection(s)
    }

    pub fn check(&self, problem: &Problem) -> bool {
        let check_id = self.id_title.contains_i64(problem.id);
        let check_title = self.id_title.contains_str(&problem.title);
        let check_solutions = if !self.solutions.is_empty() {
            problem.make_solution_items()
                .iter()
                .any(
                    |solution| self.check_solution(solution)
                )
            } else {
                true
            };

        (check_id || check_title) && check_solutions
    }

    pub fn check_solution(&self, solution: &SolutionItem) -> bool {
        if !self.solutions.is_empty() {
            for sel_sol in &self.solutions {
                if sel_sol.contains_str(&solution.solution_name) {
                    return true;
                }
                if sel_sol.contains_i64(solution.index) {
                    return true;
                }
            }

            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::problem::{Problem, SolutionInfo};

    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("100").unwrap(), 100_000);
        assert_eq!(parse_duration("100ms").unwrap(), 100);
        assert_eq!(parse_duration("1s").unwrap(), 1000);
        assert!(parse_duration("abc").is_err());
        assert!(parse_duration("1m").is_err());
    }

    #[test]
    fn test_selection_info() {
        let info = SelectionInfo::from("123");
        assert_eq!(info.name, "123");
        assert_eq!(info.num, Some(123));
        assert!(info.contains_i64(123));
        assert!(!info.contains_i64(456));
        assert!(info.contains_str("123"));
        assert!(info.contains_str("1234"));
        assert!(!info.contains_str("23"));
        assert!(!info.contains_str("45"));
        assert!(info.contains_str(String::from("1234")));
    }

    fn make_problem_info(id: i64, title: &str, solutions: &[&str]) -> Problem {
        Problem::init(id, title)
            .with_solutions(solutions.iter().map(|&s| 
                SolutionInfo::new(s, || 0) // dummy entry
            ).collect())
    }

    #[test]
    fn test_parse_problem_selection_id_only() {
        let text = "42";
        let result = ProblemSelection::parse(text);
        assert!(result.is_ok());

        let selection = result.unwrap();
        assert_eq!(selection.id_title.name, "42");
        assert_eq!(selection.id_title.num, Some(42));
        assert!(selection.solutions.is_empty());

        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[])));
        assert!(!selection.check(&make_problem_info(43, "Lorem Ipsum", &[])));
        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet", "consectetur adipiscing elit", "sed do eiusmod"])));
    }

    #[test]
    fn test_parse_problem_selection_title_only() {
        let text = "re";
        let result = ProblemSelection::parse(text);
        assert!(result.is_ok());

        let selection = result.unwrap();
        assert_eq!(selection.id_title.name, "re");
        assert_eq!(selection.id_title.num, None);
        assert!(selection.solutions.is_empty());

        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[])));
        assert!(selection.check(&make_problem_info(43, "Lorem Ipsum", &[])));
        assert!(!selection.check(&make_problem_info(42, "foobar", &[])));
        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet", "consectetur adipiscing elit", "sed do eiusmod"])));
    }

    #[test]
    fn test_parse_problem_selection_with_id_and_solution_index() {
        let text = "42.2";
        let result = ProblemSelection::parse(text);
        assert!(result.is_ok());

        let selection = result.unwrap();
        assert_eq!(selection.id_title.name, "42");
        assert_eq!(selection.id_title.num, Some(42));
        assert_eq!(selection.solutions, vec![SelectionInfo::from("2")]);

        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet", "consectetur adipiscing elit", "sed do eiusmod"])));
        assert!(!selection.check(&make_problem_info(43, "Lorem Ipsum", &[
            "dolor sit amet", "consectetur adipiscing elit"])));
        assert!(!selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet"])));
        assert!(!selection.check(&make_problem_info(42, "Lorem Ipsum", &[])));
    }

    #[test]
    fn test_parse_problem_selection_with_id_and_solution_name() {
        let text = "42.it";
        let result = ProblemSelection::parse(text);
        assert!(result.is_ok());

        let selection = result.unwrap();
        assert_eq!(selection.id_title.name, "42");
        assert_eq!(selection.id_title.num, Some(42));
        assert_eq!(selection.solutions, vec![SelectionInfo::from("it")]);

        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet", "consectetur adipiscing elit", "sed do eiusmod"])));
        assert!(!selection.check(&make_problem_info(43, "Lorem Ipsum", &[
            "dolor sit amet", "consectetur adipiscing elit"])));
        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet"])));
        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet", "sed do eiusmod"])));
        assert!(!selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "sed do eiusmod"])));
        assert!(!selection.check(&make_problem_info(42, "Lorem Ipsum", &[])));
    }

    #[test]

    fn test_parse_problem_selection_with_title_and_solution_name() {
        let text = "re.it";
        let result = ProblemSelection::parse(text);
        assert!(result.is_ok());

        let selection = result.unwrap();
        assert_eq!(selection.id_title.name, "re");
        assert_eq!(selection.id_title.num, None);
        assert_eq!(selection.solutions, vec![SelectionInfo::from("it")]);

        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet", "consectetur adipiscing elit", "sed do eiusmod"])));
        assert!(!selection.check(&make_problem_info(43, "foobar", &[
            "dolor sit amet", "consectetur adipiscing elit"])));
        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet"])));
        assert!(selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "dolor sit amet", "sed do eiusmod"])));
        assert!(!selection.check(&make_problem_info(42, "Lorem Ipsum", &[
            "sed do eiusmod"])));
        assert!(!selection.check(&make_problem_info(42, "Lorem Ipsum", &[])));
    }
}
