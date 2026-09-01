mod model;
mod client;

use core::time;
use std::{collections::HashMap, fs::{self}, path::PathBuf, println};
use clap::Parser;
use client::ClientConfig;

use anyhow::Context;
use booru_rs::{GelbooruClient, Post, prelude::*};
use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::model::{ClientType, Credentials, cli::{Cli, Commands}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // If the credentials path is the default one, create it
    if cli.credentials == model::cli::credentials_path(){
        tokio::fs::create_dir_all(model::cli::credentials_path().parent().unwrap()).await?
    }

    let cred_str = fs::read_to_string(&cli.credentials)
    .with_context(|| format!("Could not find credentials file {}",&cli.credentials.display()))?;

    // Parse the contents of the credentials file
    let credential_map: HashMap<String, Credentials> = toml::from_str(cred_str.as_str())
    .with_context(|| format!("Could not parse file {}", &cli.credentials.display()))?;
    
    let parsed_credentials = credential_map.get(&cli.client.to_string());

    let config = ClientConfig {
        name: &cli.client.to_string(),
        tags: &cli.tags,
        blacklist: &cli.blacklist,
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

    for post in posts {
        match cli.command {
            Commands::Url => command_url(post),
            Commands::Download {
                ref destination
            } => command_download(post, cli.client, destination).await?,
        }
    }

    Ok(())
}

fn command_url(post: Box<dyn Post>) {
    if let Some(url) = post.file_url() {
            println!("{}", url);
    }
}

async fn command_download(post: Box<dyn Post>, client: ClientType, destination: &PathBuf) 
-> std::result::Result<(), BooruError>
{
    let downloader = Downloader::with_client(
        reqwest::ClientBuilder::new()
        .timeout(time::Duration::from_secs(300))
        .default_headers(
            referer_header(referer_url(client))
        )
        .build()
        .expect("Could not build HTTP client")
    );

    if let Some(url) = post.file_url() {
        let download_result = downloader
        .download_url(url, &destination, None).await?;
        
        println!("{}", download_result.path.display());
    }

    Ok(())
}

fn referer_header(src: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::REFERER, 
        HeaderValue::from_static(src));
    headers
}

fn referer_url(client: ClientType) -> &'static str {
    match client {
        ClientType::Gelbooru => "https://gelbooru.com",
        _ => ""
    }
}