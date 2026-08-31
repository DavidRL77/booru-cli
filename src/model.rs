pub mod cli;

use std::fmt::Display;

use booru_rs::{Sort, gelbooru::GelbooruRating, rule34::Rule34Rating};
use serde::{Deserialize};
use clap::ValueEnum;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub user_id : String,
    pub api_key : String
}

#[derive(ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all="lowercase")]
pub enum ClientType {
    Gelbooru,
    Rule34
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

