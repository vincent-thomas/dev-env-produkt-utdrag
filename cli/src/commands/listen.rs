use std::{
    io::{BufRead, BufReader},
    process::{ChildStdout, Command, Stdio},
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Variable {
    key: String,
    value: String,
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

pub fn inject_command(args: String) {
    let injected_environments = &[Variable {
        key: "KEY_FROM_DATABASE".into(),
        value: "VERY_SECRET_VALUE".into(),
    }];

    let command = initial_command(args, injected_environments);

    let _total_logs = stream_command(command, |line| println!("{}", line));
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
    cmd.stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .stdout
        .ok_or_else(|| anyhow!("Failed to capture stdout"))
}
