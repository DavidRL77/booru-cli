mod cli;
mod client;
mod dirs;
mod error;

use std::process::{Command, ExitCode};

use clap::Parser;

use crate::{cli::Cli, error::CliError};

#[tokio::main]
async fn main() -> anyhow::Result<ExitCode> {
    let cli = Cli::parse();

    let results = cli.command.execute().await?;

    match cli.open {
        None => {
            println!("{}", results.join("\n"));
            Ok(ExitCode::SUCCESS)
        }
        Some(value) => {
            let mut command = parse_command(value)?;
            command.args(results);

            let status = command.status()?;

            if status.success() {
                Ok(ExitCode::SUCCESS)
            } else {
                Ok(ExitCode::FAILURE)
            }
        }
    }
}

fn parse_command(value: Vec<String>) -> std::result::Result<Command, CliError> {
    if value.is_empty() {
        return Err(CliError::InvalidArgument(
            "program name cannot be empty".into(),
        ));
    }

    let program = &value[0];
    let args = &value[1..];
    if program.is_empty() {
        return Err(CliError::InvalidArgument(
            "program name cannot be empty".into(),
        ));
    }

    let mut command = Command::new(program);

    if !args.is_empty() {
        command.args(args);
    }

    Ok(command)
}
