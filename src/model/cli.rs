use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::credentials_path;
use crate::model::{CliRating, CliSort, ClientType};

#[derive(Parser, Debug)]
#[command(name="booru-cli")]
#[command(version="0.1-alpha")]
#[command(about="Command line tool to interact with various booru apis.")]
pub struct Cli {

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
    Download
}