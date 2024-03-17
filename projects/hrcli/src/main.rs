use std::io::{self, Write};

const APP_NAME: &str = "hrcli";

fn main() {
    loop {
        print!("{APP_NAME}$ ");
        io::stdout().flush().expect("failed to flush message");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to parse input");

        let input = parse(&buffer);
        match input.cmd.as_ref() {
            "echo" => echo(&input),
            "exit" => exit(),
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
