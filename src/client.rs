use booru_rs::{BooruError, Client, GelbooruClient, Post, Rule34Client, Sort, };
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
    pub sort: Sort
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
            sort: client_args.sort.into()
        })
    }

    fn get_client_generic<T>(&self)
    -> booru_rs::Result<T> 
    where 
        T: Client,
        // Make sure we can convert from our custom Rating to the client's
        <T as booru_rs::Client>::Rating: From<CliRating>
    {
        if self.client.requires_auth() && self.credentials.is_none() {
            return Err(BooruError::Unauthorized(
                format!("Credentials required for {}.", self.client).into()
            ).into());
        }

        let mut builder = T::builder()
        .tags(&self.tags)?
        .blacklist_tags(&self.blacklist)
        .limit(self.limit)
        .sort(self.sort);

        if let Some(c) = &self.credentials {
            // Needs to reassign builder since it takes ownership of itself
            builder = builder.set_credentials(&c.api_key, &c.user_id)
        }

        if let Some(r) = self.rating {
            builder = builder.rating(r.into())
        }

        Ok(builder.build())
    }

    async fn get_posts_generic<T>(&self) -> booru_rs::Result<Vec<Box< dyn Post>>>
    where
        T: Client,
        <T as booru_rs::Client>::Rating: From<CliRating>,
        // Guarantee that this client's post implements the Post trait,
        // and guarantee that the post lives as long as its box
        <T as Client>::Post: Post + 'static, 
    {
        Ok(self.get_client_generic::<T>()?
        .get()
        .await?
        .into_iter()
        .map(|post| Box::new(post) as _)
        .collect())
    }
    /// Matches the ClientType to fetch a list of posts from the appropriate client,
    /// mapping it to a dynamic list of posts
    pub async fn get_posts(&self) -> booru_rs::Result<Vec<Box< dyn Post>>> {
    match self.client {
        ClientType::Gelbooru => self.get_posts_generic::<GelbooruClient>().await,
        ClientType::Rule34 => self.get_posts_generic::<Rule34Client>().await

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