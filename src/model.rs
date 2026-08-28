use std::{io, str::FromStr};

use booru_rs::{BooruError, Client, ClientBuilder, GelbooruClient, Rule34Client};
use serde::Deserialize;

use crate::error::CliError;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub user_id : String,
    pub api_key : String
}

pub enum ClientType {
    Gelbooru(ClientBuilder<GelbooruClient>),
    Rule34(ClientBuilder<Rule34Client>)
}

impl FromStr for ClientType {
    type Err = CliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "gelbooru" => Ok(ClientType::Gelbooru(GelbooruClient::builder())),
            "rule34" => Ok(ClientType::Rule34(Rule34Client::builder())),
            _ => Err(CliError::InvalidArgument(String::from(s)))
        }
    }
}