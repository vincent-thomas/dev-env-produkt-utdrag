use anyhow::{anyhow, Result};
use crossterm::style::Stylize;
use std::{
    io::{BufRead, BufReader},
    process::{ChildStdout, Command, Stdio},
};

#[derive(Debug, Copy, Clone)]
pub struct Variable<'a> {
    key: &'a str,
    value: &'a str,
}

pub fn initial_command(args: String, envs: &[Variable]) -> Command {
    let total_args = args.split(' ').collect::<Vec<&str>>();
    let program = total_args.first().expect("No program detected");

    let args = &total_args[1..];

    let mut command = Command::new(program);
    command.args(args);

    for item in envs {
        command.env(item.key, item.value);
    }

    command
}

pub fn inject_command(args: impl Into<String>) {
    let injected_environments = &[Variable {
        key: "KEY_FROM_DATABASE",
        value: "VERY_SECRET_VALUE",
    }];

    let command = initial_command(args.into(), injected_environments);

    let stream_result = stream_command(command, |line| println!("{}", line));

    if let Err(error) = stream_result {
        println!("\n{} {}\n", " ERROR ".on_red().bold(), error);
    }
}

fn stream_command<T>(mut cmd: Command, mut action: T) -> Result<Vec<String>>
where
    T: FnMut(&str),
{
    let spawned_stdout = capture_stdout(&mut cmd)?;
    let stream = BufReader::new(spawned_stdout)
        .lines()
        .filter_map(|value| value.ok())
        .map(|line| {
            action(&line);
            line
        })
        .collect::<Vec<String>>();
    Ok(stream)
}

fn capture_stdout(cmd: &mut Command) -> Result<ChildStdout> {
    let found_process = cmd
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|_| anyhow!("Failed to find command"))?;

    found_process
        .stdout
        .ok_or_else(|| anyhow!("Failed to capture stdout"))
}
