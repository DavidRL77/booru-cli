use std;

use booru_rs::{Client, GelbooruClient, Rule34Client};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub user_id : String,
    pub api_key : String
}