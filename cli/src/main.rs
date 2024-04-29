mod commands;

use clier::{
    builder::RCommand,
    run::{ExitCode, Meta, Runnable},
    Clier,
};
use commands::listen::inject_command;

fn inject_command_handler() -> RCommand {
    RCommand::new(
        "run",
        "The command to use to run a command and inject variables",
        |args| {
            inject_command(args.args.after_dashes());
            0
        },
    )
}

fn auth_login() -> RCommand {
    RCommand::new("login", "To login", |_| 0)
}

fn main() -> Result<ExitCode, clier::error::Error> {
    let meta =
        Meta::new("enver", "Enver is a nice thing", "1.2.4").usage("<command> [--flags=value]");

    let app = Clier::parse()
        .meta(&meta)
        .command(inject_command_handler())
        .command(auth_login());

    app.run()
}
