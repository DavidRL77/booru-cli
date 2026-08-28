use std;

use booru_rs::{Client, GelbooruClient, Rule34Client};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub user_id : String,
    pub api_key : String
}

pub struct ClientData<T: Client> {
    client: T,
    needs_auth: bool
}

pub enum ClientType {
    Gelbooru(ClientData<GelbooruClient>),
    Rule34(ClientData<Rule34Client>)
}