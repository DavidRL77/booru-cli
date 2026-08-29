mod model;
mod error;

use std::{collections::HashMap, debug_assert, fs::{self}, option::Option, path::PathBuf, println};

use anyhow::Context;
use booru_rs::{Client, GelbooruClient, Post, prelude::*};
use clap::Parser;

use crate::{error::CliError, model::{ClientConfig, Credentials}};

#[derive(Parser)]
#[command(name="booru-cli")]
#[command(version="0.1-alpha")]
#[command(about="Command line tool to interact with various boorus.")]
struct Cli {

    tags: Vec<String>,
    #[arg(long, short, default_value_t=1)]
    limit: u32,
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
    
    let parsed_credentials = credential_map.get(&cli.client);

    let config = ClientConfig {
        name: &cli.client,
        tags: cli.tags,
        limit: cli.limit,
        credentials: parsed_credentials,
        requires_credentials: |_| { true } // Needs better validation
    };

    let posts: Vec<Box<dyn Post>> = match cli.client.as_str() {
        "gelbooru" => { 
            config.get_client::<GelbooruClient>()?
            .get().await?
            .into_iter()
            .map(|post| Box::new(post) as _)
            .collect()
        },
        "rule34" => {
            config.get_client::<Rule34Client>()?
            .get().await?
            .into_iter()
            .map(|post| Box::new(post) as _)
            .collect()
        }
        _ => return Err(CliError::InvalidArgument(cli.client).into())
    };

    for post in posts {
        if let Some(url) = post.file_url() {
            println!("{}", url);
        }
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