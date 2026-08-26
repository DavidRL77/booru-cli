use std::{println};

use booru_rs::{Client, GelbooruClient, Post, prelude::*};
use clap::Parser;

#[derive(Parser)]
#[command(name="booru-cli")]
#[command(version="0.1-alpha")]
#[command(about="Command line tool to interact with various boorus.")]
struct Cli {

    tags: Vec<String>,
    #[arg(long)]
    user_id: String,
    #[arg(long)]
    api_key: String
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let booru = Rule34Client::builder()
    .tags(cli.tags)?
    .limit(1)
    .set_credentials(cli.api_key, cli.user_id)
    .build();

    let posts = booru.get().await?;
    for post in &posts {
        println!("{}", post.file_url);
    }

    Ok(())
}
