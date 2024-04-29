mod commands;

use clap::{Args, Parser, Subcommand};
use commands::listen::inject_command;

// #[derive(Debug, Parser)]
// enum Testing {
//     test,
// }

#[derive(Parser)]
#[clap(name = "enver")]
enum Cli {
    // #[structopt(subcommand)]
    Listen { command: String },
    // Auth(Testing),
}

fn main() {
    let result = Cli::parse();
    match result {
        Cli::Listen { command } => inject_command(command),
        // Cli::Auth(_) => {}
    };
}
