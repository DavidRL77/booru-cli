mod cli;
mod client;
mod dirs;

use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let results = cli.command.execute().await?;

    for result in results {
        println!("{}", result);
    }

    Ok(())
}
