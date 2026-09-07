pub mod model;

use core::time;
use std::{format, path::PathBuf};

use crate::client::{ClientConfig, referer_header, referer_url};
use crate::dirs::*;
use anyhow::Ok;
use booru_rs::{Post, download::Downloader};
use clap::{ArgAction, Args, Parser, Subcommand};
use serde_json::json;

use crate::cli::model::{CliRating, CliSort, ClientType};

type CommandResult = anyhow::Result<Vec<String>>;

#[derive(Parser, Debug)]
#[command(name = "booru-cli")]
#[command(version = "0.1-alpha")]
#[command(about = "Command line tool to interact with various booru APIs.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Args)]
pub struct ClientArgs {
    /// Tags to include, separated by comma or specified multiple times
    #[arg(long, short, value_delimiter = ',', global = true)]
    pub tags: Vec<String>,
    /// Tags to exclude, separated by comma or specified multiple times
    #[arg(long, short = 'T', value_delimiter = ',', global = true)]
    pub blacklist: Vec<String>,
    /// Number of posts to fetch
    #[arg(long, short, default_value_t = 1, global = true)]
    pub limit: u32,
    #[arg(value_enum, long, short, default_value_t=ClientType::Safebooru, global=true)]
    pub client: ClientType,
    #[arg(value_enum, long, short, global = true)]
    pub rating: Option<CliRating>,
    #[arg(value_enum, long, short, default_value_t=CliSort::Id, global=true)]
    pub sort: CliSort,
    #[arg(long, short, default_value_t = 0, global = true)]
    pub page: u32,
    /// Path to api credentials in toml format
    #[arg(long, default_value=credentials_path().into_os_string(), global=true)]
    pub credentials: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get each post's file url
    Url {
        #[command(flatten)]
        client_args: ClientArgs,
    },
    /// Download each post's file and get its download path
    Download {
        #[command(flatten)]
        client_args: ClientArgs,

        #[arg(long, default_value=temp_dir().into_os_string())]
        destination: PathBuf,

        /// Clear temp folder before downloading
        #[arg(long = "clear-temp")]
        clear: bool,
    },
    /// Get a JSON array of each post in JSON format
    Json {
        #[command(flatten)]
        client_args: ClientArgs,

        /// Print json without pretty formatting
        #[arg(long="no-pretty", default_value_t=true, action=ArgAction::SetFalse)]
        pretty: bool,
    },
    /// Delete this app's temp files (useful for systems that don't clear its own temp folder)
    ClearTemp,
}

impl Commands {
    /// Execute this command's logic and return a list of `String` results to be displayed or used in any way.
    pub async fn execute(self) -> CommandResult {
        match self {
            Commands::Url { client_args } => Self::url(client_args).await,
            Commands::Download {
                client_args,
                destination,
                clear,
            } => Self::download(client_args, &destination, clear).await,
            Commands::Json {
                client_args,
                pretty,
            } => Self::json(client_args, pretty).await,
            Commands::ClearTemp => Self::clear_temp().await,
        }
    }

    /// Return a list of each post's file url (empty ones are skipped)
    async fn url(client_args: ClientArgs) -> CommandResult {
        let posts = Self::get_posts(client_args).await?;

        let mut result: Vec<String> = Vec::new();
        for post in posts {
            if let Some(url) = post.file_url().map(str::to_owned) {
                result.push(url);
            }
        }

        Ok(result)
    }

    /// Download each post's file and return a list of its download paths
    async fn download(
        client_args: ClientArgs,
        destination: &PathBuf,
        clear: bool,
    ) -> CommandResult {
        if clear {
            Self::clear_temp().await?;
        }

        let downloader = Downloader::with_client(
            reqwest::ClientBuilder::new()
                .timeout(time::Duration::from_secs(300))
                .default_headers(referer_header(referer_url(client_args.client)))
                .build()
                .expect("Could not build HTTP client"),
        );

        let posts = Self::get_posts(client_args).await?;

        let mut result: Vec<String> = Vec::new();
        for post in posts {
            if let Some(url) = post.file_url() {
                let download_result = downloader.download_url(url, destination, None).await?;

                result.push(download_result.path.display().to_string());
            }
        }

        Ok(result)
    }

    /// Return a single json array containing every post in json format
    async fn json(client_args: ClientArgs, pretty: bool) -> CommandResult {
        let posts = Self::get_posts(client_args).await?;

        let mut json_posts: Vec<serde_json::Value> = Vec::new();
        for post in posts {
            json_posts.push(Self::post_to_json(post));
        }

        // An array of json values containing the converted posts
        let result = json!(json_posts);
        let string = if pretty {
            format!("{:#}", result)
        } else {
            format!("{}", result)
        };

        Ok(vec![string])
    }

    /// Delete all files in this app's temp directory.
    /// Returns an empty result.
    async fn clear_temp() -> CommandResult {
        let mut iter = tokio::fs::read_dir(temp_dir()).await?;
        while let Some(d) = iter.next_entry().await? {
            let path = d.path();
            if path.is_file() {
                tokio::fs::remove_file(path).await?;
            }
        }

        Ok(Vec::new())
    }

    /// Utility to convert a `Post` trait into a JSON `Value`
    fn post_to_json(post: Box<dyn Post>) -> serde_json::Value {
        // No way to serialize a trait, so I'll just do it myself
        json!({
            "id": &post.id(),
            "width": &post.width(),
            "height": &post.height(),
            "file_url": &post.file_url(),
            "tags": &post.tags().split(' ').collect::<Vec<_>>(),
            "score": &post.score(),
            "md5": &post.md5(),
            "source": &post.source()
        })
    }

    /// Utility function to convert `ClientArgs` -> `ClientConfig` and its posts
    async fn get_posts(client_args: ClientArgs) -> anyhow::Result<Vec<Box<dyn Post>>> {
        // Kinda weird, but I'm using '?' for implicit conversion for the error,
        // which is why I wrap the result in Ok
        Ok(ClientConfig::from_args(client_args)?.get_posts().await?)
    }
}
