pub mod model;

use core::time;
use std::{path::PathBuf, format};

use crate::client::{referer_header,referer_url};
use crate::dirs::*;
use anyhow::{Ok, Result};
use booru_rs::{Post, download::Downloader};
use clap::{ArgAction, Parser, Subcommand};
use serde_json::{json};

use crate::cli::model::{CliRating, CliSort, ClientType};

#[derive(Parser, Debug)]
#[command(name="booru-cli")]
#[command(version="0.1-alpha")]
#[command(about="Command line tool to interact with various booru apis.")]
pub struct Cli {
    /// Tags to include, separated by comma or specified multiple times
    #[arg(long, short, value_delimiter=',', global=true)]
    pub tags: Vec<String>,
    /// Tags to exclude, separated by comma or specified multiple times
    #[arg(long, short='T', value_delimiter=',', global=true)]
    pub blacklist: Vec<String>,
    /// Number of posts to return
    #[arg(long, short, default_value_t=1, global=true)]
    pub limit: u32,
    #[arg(value_enum, long, short, default_value_t=ClientType::Gelbooru, global=true)]
    pub client: ClientType,
    #[arg(value_enum, long, short, global=true)]
    pub rating: Option<CliRating>,
    #[arg(value_enum, long, short, default_value_t=CliSort::Id, global=true)]
    pub sort: CliSort,
    /// Path to credentials in toml format
    #[arg(long, default_value=credentials_path().into_os_string(), global=true)]
    pub credentials: PathBuf,

    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Print image url of post to stdout
    Url,
    /// Downloads the image file and prints its path to stdout
    Download {
        #[arg(long, default_value=temp_dir().into_os_string())]
        destination: PathBuf
    },
    Json {
        /// Print json without pretty formatting
        #[arg(long="no-pretty", default_value_t=true, action=ArgAction::SetFalse)]
        pretty: bool
    }
}


impl Commands {
    pub async fn execute(&self, post: &dyn Post, cli: &Cli)
    -> Result<Option<String>>
    {
        match self {
            Commands::Url 
            => self.url(post),
            Commands::Download { destination }
            => self.download(post, cli, destination).await,
            Commands::Json { pretty }
            => self.json(post, *pretty) 
        }
    }

    fn url(&self, post: &dyn Post) 
    -> Result<Option<String>>
    {
        Ok(post.file_url().map(str::to_owned))
    }

    async fn download(&self, post: &dyn Post, cli: &Cli, destination: &PathBuf) 
    -> Result<Option<String>>
    {
        let downloader = Downloader::with_client(
        reqwest::ClientBuilder::new()
            .timeout(time::Duration::from_secs(300))
            .default_headers(
                referer_header(referer_url(cli.client))
            )
            .build()
            .expect("Could not build HTTP client")
        );

        if let Some(url) = post.file_url() {
            let download_result = downloader
            .download_url(url, destination, None).await?;
            
            return Ok(Some(download_result.path.display().to_string()));
        }

        Ok(None)
    }

    fn json(&self, post: &dyn Post, pretty: bool)
    -> anyhow::Result<Option<String>>
    {
        // No way to serialize a trait, so I'll just do it myself
        let value = json!({
            "id": &post.id(),
            "width": &post.width(),
            "height": &post.height(),
            "file_url": &post.file_url(),
            "tags": &post.tags().split(' ').collect::<Vec<_>>(),
            "score": &post.score(),
            "md5": &post.md5(),
            "source": &post.source()
        });

        let string: String = if pretty {
            format!("{:#}", value)
        } else {
            format!("{}", value)
        };

        Ok(Some(string))
    }
}