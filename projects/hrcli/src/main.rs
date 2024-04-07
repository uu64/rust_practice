use std::{collections::HashMap, io::{self, Write}};

use regex::Regex;

const APP_NAME: &str = "hrcli";

struct EmployeeRepository {
    repository: HashMap<String, Vec<String>>
}

impl EmployeeRepository {
    fn list_employees(&mut self, department: &str) {
        if department.is_empty() {
            let mut keys: Vec<_> = self.repository.keys().collect();
            keys.sort();
            for k in keys {
                let mut employees = self.repository.get(k).unwrap().clone();
                employees.sort();
                for employee in employees {
                    println!("{}: {}", k, employee);
                }
            }
        } else {
            let mut employees = self.repository.get(department).unwrap().clone();
            employees.sort();
            for employee in employees {
                println!("{}: {}", department, employee);
            }
        }
    }

    fn add_employee(&mut self, department: &str, employee: &str) {
        self.repository.entry(department.to_string())
            .and_modify(|e| {e.push(employee.to_string())})
            .or_insert(vec![employee.to_string()]);
    }
}

fn main() {
    let mut repository = EmployeeRepository {
        repository: HashMap::new(),
    };

    loop {
        print!("{APP_NAME}$ ");
        io::stdout().flush().expect("failed to flush message");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to parse input");

        let input = parse(&buffer);
        match input.cmd.as_ref() {
            "echo" => echo(&input),
            "exit" => exit(),
            "list" => {
                repository.list_employees(input.args.trim())
            },
            "add" => {
                let re = Regex::new(r#""?(?<name>[a-zA-Z0-9 ]+)"?[ ]+to[ ]+"?(?<department>[a-zA-Z0-9 ]+)"?"#).unwrap();
                let Some(caps) = re.captures(input.args.trim()) else {
                    println!("invalid synatax");
                    continue;
                };

                repository.add_employee(&caps["department"], &caps["name"]);
            }
            &_ => {
                println!("unexpected command, continue");
                continue;
            },
        }
    }
}

#[derive(Debug)]
struct CliInput {
    cmd: String,
    args: String,
}

fn parse(s: &str) -> CliInput {
    let input: Vec<&str> = s.trim().splitn(2, ' ').collect();
    return CliInput {
        cmd: input[0].to_string(),
        args: match input.get(1) {
            Some(s) => s.to_string(),
            None => String::from(""),
        },
    }
}

fn echo(input: &CliInput) {
    println!("{} {}", input.cmd, input.args);
}

fn exit() {
    println!("bye");
    std::process::exit(0);
}
