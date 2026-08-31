use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::model::{CliRating, CliSort, ClientType};

#[derive(Parser, Debug)]
#[command(name="booru-cli")]
#[command(version="0.1-alpha")]
#[command(about="Command line tool to interact with various booru apis.")]
pub struct Cli {
    /// Tags specified multiple times, or separated by comma
    #[arg(long, short, value_delimiter=',')]
    pub tags: Vec<String>,
    /// Number of posts to return
    #[arg(long, short, default_value_t=1)]
    pub limit: u32,
    #[arg(value_enum, long, short, default_value_t=ClientType::Gelbooru)]
    pub client: ClientType,
    #[arg(value_enum, long, short)]
    pub rating: Option<CliRating>,
    #[arg(value_enum, long, short, default_value_t=CliSort::Id)]
    pub sort: CliSort,
    /// Path to credentials in toml format
    #[arg(long, default_value=credentials_path().into_os_string())]
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
    }
}

pub fn config_dir() -> PathBuf {
    dirs::config_local_dir()
    .unwrap()
    .join("booru-cli")
}

pub fn credentials_path() -> PathBuf {
    config_dir()
    .join("credentials.toml")
}

pub fn temp_dir() -> PathBuf {
    std::env::temp_dir()
    .join("booru-cli")
}