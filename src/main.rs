mod model;
mod error;

use std::{collections::HashMap, debug_assert, fs::{self}, option::Option, path::PathBuf, println};

use anyhow::Context;
use booru_rs::{Client, GelbooruClient, Post, prelude::*};
use clap::Parser;

use crate::{error::CliError, model::Credentials};

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
    
    let credentials = credential_map.get(&cli.client);

    let posts: Vec<Box<dyn Post>> = match cli.client.as_str() {
        "gelbooru" => { 
            configure_client::<GelbooruClient>(cli.tags, cli.limit,
                credentials, true)?
            .get().await?
            .into_iter()
            .map(|post| Box::new(post) as _)
            .collect()
        },
        "rule34" => {
            configure_client::<Rule34Client>(cli.tags, cli.limit,
                credentials, true)?
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



fn configure_client<T>(tags: Vec<String>, limit: u32,
    credentials: Option<&Credentials>, requires_credentials: bool) 
-> anyhow::Result<T> 
where T : Client
{
    if requires_credentials && credentials.is_none() {
        return Err(BooruError::Unauthorized("Credentials required for this client.".into()).into());
    }

    let mut builder = T::builder().tags(tags)?.limit(limit);

    // Needs to reassign builder since it takes ownership of itself
    builder = if let Some(c) = credentials {
        builder.set_credentials(&c.api_key, &c.user_id)
    } else {
        builder
    };

    Ok(builder.build())
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