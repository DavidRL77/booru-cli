mod model;
mod error;

use std::{collections::HashMap, fs::{self}, hash::Hash, path::{Path, PathBuf}, println, result, str::FromStr};

use anyhow::Context;
use booru_rs::{Client, GelbooruClient, prelude::*};
use clap::Parser;

use crate::{error::CliError, model::{ClientType, Credentials}};

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
async fn main() -> anyhow::Result<()> {
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

    let client_type = get_client(&cli.client, &cli.tags)?;
    
    let credentials = credential_map.get(&cli.client)
    .with_context(|| format!("No credentials found for {}", cli.client))?;

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

fn get_client(client: &str, tags: &[String]) -> anyhow::Result<ClientType> {
    match client {
        _ => Err(CliError::InvalidArgument(client.to_string()).into())
    }
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