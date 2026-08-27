mod model;

use std::{collections::HashMap, error::Error, fs::{self}, path::PathBuf, println};

use anyhow::Context;
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
async fn main() -> anyhow::Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let credentials = cli.credentials;

    // If the credentials path is the default one, create it
    if credentials == credentials_path(){
        fs::create_dir_all(credentials_path().parent().unwrap())?
    }

    let cred_str = fs::read_to_string(&credentials)
    .with_context(|| format!("Could not find credentials file {}",&credentials.display()))?;

    // Parse the contents of the credentials file
    let credential_map: HashMap<String, Credentials> = toml::from_str(cred_str.as_str())
    .with_context(|| format!("Could not parse file {}", &credentials.display()))?;

    println!("Credentials: {credential_map:?}");

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