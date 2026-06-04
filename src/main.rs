mod content;
mod fonts;
mod github;
mod render;
mod stats;

use std::{env, process};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Stats,
    Build,
    Go,
}

fn parse_command() -> Result<Command, String> {
    let command = env::args()
        .nth(1)
        .ok_or_else(|| String::from("missing command"))?;

    match command.as_str() {
        "stats" => Ok(Command::Stats),
        "build" => Ok(Command::Build),
        "go" => Ok(Command::Go),
        _ => Err(format!("unknown command: {command}")),
    }
}

fn print_usage() {
    eprintln!("Usage: cargo run -- <stats|build|go>");
}

fn run_placeholder(command: Command) {
    match command {
        Command::Stats => println!("TODO: implement Rust stats generation."),
        Command::Build => println!("TODO: implement Rust SVG build pipeline."),
        Command::Go => {
            println!("TODO: implement Rust stats generation.");
            println!("TODO: implement Rust SVG build pipeline.");
        }
    }
}

fn main() {
    let command = match parse_command() {
        Ok(command) => command,
        Err(err) => {
            eprintln!("Error: {err}");
            print_usage();
            process::exit(2);
        }
    };

    run_placeholder(command);
}
