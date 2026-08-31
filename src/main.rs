mod model;
mod error;
mod client;

use core::time;
use std::{collections::HashMap, dbg, fs::{self}, option::Option, path::PathBuf, println};
use client::ClientConfig;

use anyhow::Context;
use booru_rs::{GelbooruClient, Post, prelude::*};
use clap::Parser;
use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::model::{CliSort, ClientType, Credentials, CliRating};

#[derive(Parser, Debug)]
#[command(name="booru-cli")]
#[command(version="0.1-alpha")]
#[command(about="Command line tool to interact with various boorus.")]
struct Cli {

    tags: Vec<String>,
    #[arg(long, short, default_value_t=1)]
    limit: u32,
    #[arg(value_enum, long, short, default_value_t=ClientType::Gelbooru)]
    client: ClientType,
    #[arg(value_enum, long, short)]
    rating: Option<CliRating>,
    #[arg(value_enum, long, short, default_value_t=CliSort::Id)]
    sort: CliSort,
    #[arg(long, default_value=credentials_path().into_os_string())]
    credentials: PathBuf,
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
    
    let parsed_credentials = credential_map.get(&cli.client.to_string());

    let config = ClientConfig {
        name: &cli.client.to_string(),
        tags: cli.tags,
        limit: cli.limit,
        credentials: parsed_credentials,
        requires_credentials: |_| { true }, // Needs better validation
        rating: cli.rating,
        sort: cli.sort.into()
    };

    let posts: Vec<Box<dyn Post>> = match cli.client {
        ClientType::Gelbooru => config.get_posts::<GelbooruClient>().await?,
        ClientType::Rule34 => config.get_posts::<Rule34Client>().await?
    };

    let downloader = Downloader::with_client(
        reqwest::ClientBuilder::new()
        .timeout(time::Duration::from_secs(300))
        .default_headers(
            get_headers()
        )
        .build()
        .expect("Could not build HTTP client")
    );

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

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::REFERER, 
        HeaderValue::from_static("https://gelbooru.com"));
    headers
}