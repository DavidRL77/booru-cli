use booru_rs::{BooruError, Client, Post, Sort, client::{gelbooru, rule34, safebooru}, };
use reqwest::header::{self, HeaderMap, HeaderValue};
use crate::cli::{ClientArgs, model::{CliRating, ClientType, Credentials}};

// Basically a wrapper around information all booru clients share,
// to ease the repetition of creating different clients with the same data.
pub struct ClientConfig {
    pub client: ClientType,
    pub tags: Vec<String>,
    pub blacklist: Vec<String>,
    pub limit: u32,
    pub credentials: Option<Credentials>,
    pub rating: Option<CliRating>,
    pub sort: Sort,
    pub page: u32
}

impl ClientConfig {
    pub fn from_args(client_args: ClientArgs) -> anyhow::Result<Self> {
        Ok(Self {
            client: client_args.client,
            tags: client_args.tags,
            blacklist: client_args.blacklist,
            limit: client_args.limit,
            credentials: Credentials::load(client_args.credentials, &client_args.client.to_string())?,
            rating: client_args.rating,
            sort: client_args.sort.into(),
            page: client_args.page
        })
    }

    async fn get_posts_generic<C>(client: &C, query: C::Query) -> booru_rs::Result<Vec<Box< dyn Post>>>
    where
        C: Client,
        C::Post: 'static
    {
        let page = client.page(query, None).await?;
        Ok(page.posts
        .into_iter()
        .map(|post| Box::new(post) as _)
        .collect())
    }

    fn get_required_credentials(&self) -> booru_rs::Result<&Credentials>{
        match &self.credentials {
            Some(credentials) => Ok(credentials),
            None => Err(BooruError::Unauthorized(
                format!("Credentials required for {}.", self.client)
            ))
        }
    }

    /// Matches the ClientType to fetch a list of posts from the appropriate client,
    /// mapping it to a dynamic list of posts
    pub async fn get_posts(&self) -> booru_rs::Result<Vec<Box< dyn Post>>> {
        match self.client {
            ClientType::Safebooru => {
                let builder = safebooru::Client::builder().build()?;
                let query = safebooru::Query::new()
                .tags(&self.tags)
                .blacklist_tags(&self.blacklist)
                .limit(self.limit)
                .sort(self.sort);

                Self::get_posts_generic(&builder, query).await
            }
            ClientType::Gelbooru => {
                let credentials = self.get_required_credentials()?;
                let client = gelbooru::Client::builder()
                .set_credentials(&credentials.api_key, &credentials.user_id)
                .build()?;
                let query = gelbooru::Query::new()
                .tags(&self.tags)
                .blacklist_tags(&self.blacklist)
                .limit(self.limit)
                .sort(self.sort);

                Self::get_posts_generic(&client, query).await
            }
            ClientType::Rule34 => {
                let credentials = self.get_required_credentials()?;
                let client = rule34::Client::builder()
                .set_credentials(&credentials.api_key, &credentials.user_id)
                .build()?;
                let query = rule34::Query::new()
                .tags(&self.tags)
                .blacklist_tags(&self.blacklist)
                .limit(self.limit)
                .sort(self.sort);

                Self::get_posts_generic(&client, query).await
            }
        }
    }
}

pub fn referer_header(src: &'static str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::REFERER, 
        HeaderValue::from_static(src));
    headers
}

pub fn referer_url(client: ClientType) -> &'static str {
    match client {
        ClientType::Gelbooru => "https://gelbooru.com",
        _ => ""
    }
}