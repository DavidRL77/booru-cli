mod cli;
mod dirs;
mod client;

use crate::dirs::*;
use std::{collections::HashMap, fs::{self}, println};
use clap::Parser;
use client::ClientConfig;

use anyhow::Context;
use booru_rs::{GelbooruClient, Post, prelude::*};

use crate::cli::{Cli, model::{ClientType, Credentials}};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    // If the credentials path is the default one, create it
    if cli.credentials == credentials_path(){
        tokio::fs::create_dir_all(credentials_path().parent().unwrap()).await?
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
        let result= cli.command.execute(post.as_ref(), &cli).await?;

        if let Some(r) = result {
            println!("{}", r);
        }
    }

    Ok(())
}