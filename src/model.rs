use booru_rs::{BooruError, Client, Post, };
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Credentials {
    pub user_id : String,
    pub api_key : String
}

pub struct ClientConfig<'a> {
    pub name: &'a str,
    pub tags: Vec<String>,
    pub limit: u32,
    pub credentials: Option<&'a Credentials>,
    pub requires_credentials: fn(config: &Self) -> bool
}

impl ClientConfig<'_> {
    pub fn get_client<T: Client>(&self)
    -> booru_rs::Result<T>
    {
        if (self.requires_credentials)(self) && self.credentials.is_none() {
            return Err(BooruError::Unauthorized(
                format!("Credentials required for {}.", self.name).into()
            ).into());
        }

        let mut builder = T::builder()
        .tags(&self.tags)?
        .limit(self.limit);

        // Needs to reassign builder since it takes ownership of itself
        builder = if let Some(c) = self.credentials {
            builder.set_credentials(&c.api_key, &c.user_id)
        } else {
            builder
        };

        Ok(builder.build())
    }

    pub async fn get_posts<T>(&self) -> booru_rs::Result<Vec<Box< dyn Post>>>
    where
        T: Client,
        <T as Client>::Post: Post + 'static // Guarantee that this client's post implements the Post trait,
        // and guarantee that the post lives as long as the box
    {
        Ok(self.get_client::<T>()?
        .get()
        .await?
        .into_iter()
        .map(|post| Box::new(post) as _)
        .collect())
    }

}