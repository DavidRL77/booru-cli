use std::{collections::HashMap, fmt::Display, path::Path};

use anyhow::Context;
use booru_rs::{Sort, gelbooru::GelbooruRating, rule34::Rule34Rating, safebooru::SafebooruRating};
use serde::{Deserialize};
use clap::ValueEnum;
use crate::dirs;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub user_id : String,
    pub api_key : String
}

impl Credentials {
    pub fn load(path: impl AsRef<Path>, key: &str) -> 
    anyhow::Result<Option<Self>>
    {
        let path = path.as_ref();

        // If the specified path is the same as the default, try to create it
        if path == dirs::credentials_path() {
            std::fs::create_dir_all(dirs::credentials_path().parent().unwrap())?
        }

        let cred_str = match std::fs::read_to_string(path) {
            Ok(value) => value,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => { // Return none only if no file is found
                return Ok(None);
            },
            Err(err) => return Err(err.into())
        };

        // Parse the contents of the credentials file
        let mut credential_map: HashMap<String, Self> = toml::from_str(cred_str.as_str())
        .with_context(|| format!("Could not parse file {}", path.display()))?;

        // Remove the value from the map so we can own and return it
        let parsed_credentials = credential_map.remove(key);

        Ok(parsed_credentials)
    }
}

#[derive(ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all="lowercase")]
pub enum ClientType {
    Safebooru,
    Gelbooru,
    Rule34
}

impl ClientType {
    pub fn requires_auth(&self) -> bool {
        match self {
            ClientType::Safebooru => false,
            ClientType::Gelbooru => true,
            ClientType::Rule34 => true
        }
    }
}

impl Display for ClientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        return write!(f, "{}", name);
    }
}

#[derive(ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all="lowercase")]
pub enum CliRating {
    /// Nothing sexual, safe to watch in public
    Safe,
    /// No explicit sex, but may contain nudity
    Questionable,
    /// Sex
    Explicit
}

impl From<CliRating> for GelbooruRating {
    fn from(value: CliRating) -> Self {
        match value {
            CliRating::Safe => GelbooruRating::General,
            CliRating::Questionable => GelbooruRating::Questionable,
            CliRating::Explicit => GelbooruRating::Explicit
        }
    }
}

impl From<CliRating> for Rule34Rating {
    fn from(value: CliRating) -> Self {
        match value {
            CliRating::Safe => Rule34Rating::Safe,
            CliRating::Questionable => Rule34Rating::Questionable,
            CliRating::Explicit => Rule34Rating::Explicit
        }
    }
}

impl From<CliRating> for SafebooruRating {
    fn from(value: CliRating) -> Self {
        match value {
            CliRating::Safe => SafebooruRating::Safe,
            CliRating::Questionable => SafebooruRating::Questionable,
            CliRating::Explicit => SafebooruRating::Explicit
        }
    }
}

// Need to duplicate booru_rs' sort enum to be able to use it as a cli arg
#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum CliSort {
    /// Sort by post ID.
    Id,
    /// Sort by score/votes.
    Score,
    /// Sort by rating.
    Rating,
    /// Sort by uploader.
    User,
    /// Sort by image height.
    Height,
    /// Sort by image width.
    Width,
    /// Sort by source URL.
    Source,
    /// Sort by last update time.
    Updated,
    /// Random ordering.
    Random,
}

impl From<CliSort> for Sort {
    fn from(value: CliSort) -> Self {
        match value {
            CliSort::Id => Sort::Id,
            CliSort::Score => Sort::Score,
            CliSort::Rating => Sort::Rating,
            CliSort::User => Sort::User,
            CliSort::Height => Sort::Height,
            CliSort::Width => Sort::Width,
            CliSort::Source => Sort::Source,
            CliSort::Updated => Sort::Updated,
            CliSort::Random => Sort::Random,
        }
    }
}

