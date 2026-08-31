use booru_rs::{BooruError, Client, Post, Sort, };
use crate::{model::{Credentials, CliRating}};

// Basically a wrapper around information all booru clients share,
// to ease the repetition of creating different clients with the same data.
pub struct ClientConfig<'a> {
    pub name: &'a str,
    pub tags: &'a Vec<String>,
    pub limit: u32,
    pub credentials: Option<&'a Credentials>,
    pub requires_credentials: fn(config: &Self) -> bool,
    pub rating: Option<CliRating>,
    pub sort: Sort
}

impl ClientConfig<'_> {
    pub fn get_client<T>(&self)
    -> booru_rs::Result<T> 
    where 
        T: Client,
        // Make sure we can convert from our custom Rating to the client's
        <T as booru_rs::Client>::Rating: From<CliRating>
    {
        if (self.requires_credentials)(self) && self.credentials.is_none() {
            return Err(BooruError::Unauthorized(
                format!("Credentials required for {}.", self.name).into()
            ).into());
        }

        let mut builder = T::builder()
        .tags(self.tags)?
        .limit(self.limit)
        .sort(self.sort);

        if let Some(c) = self.credentials {
            // Needs to reassign builder since it takes ownership of itself
            builder = builder.set_credentials(&c.api_key, &c.user_id)
        }

        if let Some(r) = self.rating {
            builder = builder.rating(r.into())
        }

        Ok(builder.build())
    }

    pub async fn get_posts<T>(&self) -> booru_rs::Result<Vec<Box< dyn Post>>>
    where
        T: Client,
        <T as booru_rs::Client>::Rating: From<CliRating>,
        // Guarantee that this client's post implements the Post trait,
        // and guarantee that the post lives as long as its box
        <T as Client>::Post: Post + 'static, 
    {
        Ok(self.get_client::<T>()?
        .get()
        .await?
        .into_iter()
        .map(|post| Box::new(post) as _)
        .collect())
    }

}