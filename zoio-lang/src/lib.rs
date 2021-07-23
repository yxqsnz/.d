use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
pub mod stdlib;
pub mod util;
use util::eval;

#[derive(Debug, Clone)]
pub enum Var {
    String(String),
    Number(f64),
    Other(String),
    Nothing,
}
impl Display for Var {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Var::String(s) => s.to_owned(),
                Var::Number(n) => n.to_string(),
                Var::Other(o) => o.to_owned(),
                Var::Nothing => String::from("Nothing"),
            }
        )
    }
}
pub struct Zoio {
    pub imports: Vec<String>,
    pub vars: HashMap<String, Var>,
    pub funcs: HashMap<String, String>,
    pub current_line: u64,
}
impl Zoio {
    pub fn new() -> Self {
        Self {
            imports: vec![],
            vars: HashMap::new(),
            funcs: HashMap::new(),
            current_line: 1,
        }
    }
    fn strfmt(&self, s: &str) -> String {
        let mut result = String::from(s);
        for (name, value) in &self.vars {
            result = result
                .replace(&format!("${{{}}}", name), &format!("{}", value))
                .replace(&format!("${}", name), &format!("{}", value));
        }
        self.strclean(&result)
    }
    fn strclean(&self, s: &str) -> String {
        let mut result = String::from(s);
        if result.starts_with("\"") & result.ends_with("\"") {
            let mut chars = result.chars();
            chars.next();
            chars.next_back();
            result = chars.as_str().to_string();
        }
        result
    }

    pub fn run(&mut self, code: &String) -> Result<String, String> {
        let code_splited = code.split_whitespace().collect::<Vec<_>>();
        let objs = code.split('.').collect::<Vec<_>>();
        let mut result = String::from("Nothing");
        if code_splited.get(0).is_none() {
            return Ok("Nothing".into());
        }
        match objs[0] {
            "io" => {
                if self.imports.contains(&String::from("io")) {
                    let _raw = objs[1].split('(').collect::<Vec<_>>();
                    let func = &_raw[0];
                    let mut args = _raw.clone();
                    args.remove(0);
                    if args.is_empty() {
                        return Err(format!("{} requires at last 1 argument", func));
                    }
                    if let Some(last) = args.last() {
                        let mut chars = last.chars();
                        if let Some(last) = chars.clone().last() {
                            if last.to_string() != *")" {
                                return Err(String::from("unterminated ("));
                            } else {
                                chars.next_back();
                                args.pop();
                                args.push(&chars.as_str());
                            }
                        }
                    } else {
                        return Err(String::from("unterminated ("));
                    }

                    match *func {
                        "Println" => {
                            if self.vars.contains_key(args[0]) {
                                if let Var::String(s) = &self.vars[args[0]] {
                                    args[0] = &s;
                                } else {
                                    return Err(format!(
                                        "Print Functions requires string. but {:?} has provided.",
                                        &self.vars[args[0]]
                                    ));
                                }
                            }

                            let content = args.into_iter().collect::<String>();
                            stdlib::io::println(&self.strclean(&self.strfmt(&content)));
                        }
                        "Flush" => {
                            stdlib::io::flush_stdout();
                        }
                        "Print" => {
                            if self.vars.contains_key(args[0]) {
                                if let Var::String(s) = &self.vars[args[0]] {
                                    args[0] = &s;
                                } else {
                                    return Err(format!(
                                        "Print Functions requires string. but {:?} has provided.",
                                        &self.vars[args[0]]
                                    ));
                                }
                            }

                            let content = args.into_iter().collect::<String>();
                            stdlib::io::print(&self.strclean(&self.strfmt(&content)));
                        }
                        "ReadLine" => {
                            let arg = &args[0].to_string();
                            if self.vars.get(arg).is_none() {
                                return Err(format!("unknown variable: {}", args[0]));
                            }
                            let buf = stdlib::io::read_line();

                            self.vars.insert(arg.to_string(), Var::String(buf));
                        }
                        _ => return Err(format!("undefined function {}", func)),
                    }
                } else {
                    return Err(String::from(
                        "undefined object `io`. Please consider importing io.",
                    ));
                }
            }
            _ => {}
        }
        match code_splited[0] {
            "import" => {
                if code_splited.len() < 2 {
                    return Err(String::from("missing module to import"));
                }
                let lib = code_splited[1];
                match lib {
                    "io" => {
                        self.imports.push(String::from("io"));
                    }
                    _ => {}
                }
            }
            "var" => {
                let var_splited = &code.splitn(2, "=").collect::<Vec<_>>();
                if var_splited.len() < 2 {
                    return Err(String::from("missing variable name or value"));
                }
                let value = var_splited[1].trim().to_string();
                let mut new_value = value.clone();
                if new_value.starts_with("\"") {
                    if code.ends_with("\"") {
                        let mut chars = value.chars();
                        chars.next();
                        chars.next_back();
                        new_value = chars.as_str().to_string();
                    } else {
                        return Err(String::from("unterminated string"));
                    }
                }

                for (var_name, var_value) in &self.vars {
                    if new_value.contains(&format!("{}", var_name)) {
                        if let Var::Number(n) = var_value {
                            new_value = new_value.replace(&format!("{}", var_name), &n.to_string());
                        }
                    }
                }

                let result = if let Ok(n) = value.parse::<f64>() {
                    Var::Number(n)
                } else if value == "Nothing" {
                    Var::Nothing
                } else if let Ok(n) = eval(&value) {
                    Var::Number(n)
                } else {
                    Var::String(self.strfmt(&value))
                };

                self.vars.insert(
                    var_splited[0]
                        .to_string()
                        .trim()
                        .split_whitespace()
                        .collect::<Vec<_>>()[1]
                        .to_string(),
                    result,
                );
            }
            _ => {
                if self.vars.contains_key(&code_splited[0].to_string()) {
                    result = format!("{}", self.vars[&code_splited[0].to_string()]);
                }
            }
        };
        self.current_line += 1;
        Ok(result)
    }
}
