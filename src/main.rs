mod model;

use std::{collections::HashMap, env, ffi::OsString, path::PathBuf, println};

use booru_rs::{Client, GelbooruClient, prelude::*};
use clap::Parser;

use crate::model::Credentials;

#[derive(Parser)]
#[command(name="booru-cli")]
#[command(version="0.1-alpha")]
#[command(about="Command line tool to interact with various boorus.")]
struct Cli {

    tags: Vec<String>,
    #[arg(long, short, default_value_t="gelbooru".to_string())]
    client: String,
    #[arg(long, default_value=credentials_path().into_os_string())]
    credentials: PathBuf
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // TODO: Read from actual file
    let credential_map : HashMap<&str, Credentials> = toml::from_str("").unwrap();

    println!("{credential_map:?}");

    // To download a gelbooru image, we'll need to spoof the Referer header in the request,
    // otherwise we'll be redirected to the post
    let booru = GelbooruClient::builder()
    .tags(cli.tags)?
    .limit(1)
    .build();

    let posts = booru.get().await?;
    for post in &posts {
        println!("{}", post.file_url);
    }

    Ok(())
}

fn config_dir() -> PathBuf {
    dirs::config_local_dir()
    .unwrap()
    .join("booru-cli")
}

fn credentials_path() -> PathBuf {
    config_dir()
    .join("credentials.toml")
}