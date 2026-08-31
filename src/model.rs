use core::fmt;

use booru_rs::{Sort, gelbooru::GelbooruRating, rule34::Rule34Rating};
use serde::{Deserialize, Serialize};
use clap::ValueEnum;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub user_id : String,
    pub api_key : String
}

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all="lowercase")]
pub enum ClientType {
    Gelbooru,
    Rule34
}

impl fmt::Display for ClientType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        return write!(f, "{name}");
    }
}

#[derive(Serialize, ValueEnum, Copy, Clone, Debug)]
#[clap(rename_all="lowercase")]
#[serde(rename_all="lowercase")]
pub enum Rating {
    /// Nothing sexual, safe to watch in public
    Safe,
    /// No explicit sex, but may contain nudity
    Questionable,
    /// Sex
    Explicit
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        return write!(f, "{name}");
    }
}

impl From<Rating> for GelbooruRating {
    fn from(value: Rating) -> Self {
        match value {
            Rating::Safe => GelbooruRating::General,
            Rating::Questionable => GelbooruRating::Questionable,
            Rating::Explicit => GelbooruRating::Explicit
        }
    }
}

impl From<Rating> for Rule34Rating {
    fn from(value: Rating) -> Self {
        match value {
            Rating::Safe => Rule34Rating::Safe,
            Rating::Questionable => Rule34Rating::Questionable,
            Rating::Explicit => Rule34Rating::Explicit
        }
    }
}

// Need to duplicate booru_rs' sort enum to be able to use it as a cli arg
#[derive(ValueEnum, Clone, Debug)]
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

impl fmt::Display for CliSort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self).to_lowercase();
        return write!(f, "{name}");
    }
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

