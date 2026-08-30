use core::fmt;

use booru_rs::{gelbooru::GelbooruRating, rule34::Rule34Rating};
use serde::{Deserialize, Serialize};
use clap::ValueEnum;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub user_id : String,
    pub api_key : String
}

#[derive(Serialize, ValueEnum, Clone, Debug)]
#[clap(rename_all="lowercase")]
#[serde(rename_all="lowercase")]
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
    Safe,
    Questionable,
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