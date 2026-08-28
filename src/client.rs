use booru_rs::{Client, GelbooruClient, Post, Rule34Client, gelbooru::GelbooruPost};

pub trait ClientWrapper {
    const AUTH: bool;

    fn get_by_id_wrapper(&self, id: u32) -> impl std::future::Future<Output = booru_rs::Result<Box<dyn Post>>> + Send;
    fn get_wrapper(&self) -> impl std::future::Future<Output = booru_rs::Result<Vec<Box<dyn Post>>>> + Send;
}

impl ClientWrapper for GelbooruClient {
    const AUTH: bool = true;

    async fn get_by_id_wrapper(&self, id: u32) -> booru_rs::Result<Box<dyn Post>> {
        self.get_by_id(id)
        .await
        .map(|post| Box::new(post) as _)
    }

    async fn get_wrapper(&self) -> booru_rs::Result<Vec<Box<dyn Post>>> {
        self.get()
        .await
        .map(|posts|
            posts.into_iter().map(|post| Box::new(post) as _).collect()
        )
    }
}

impl ClientWrapper for Rule34Client {
    const AUTH: bool = true;

    async fn get_by_id_wrapper(&self, id: u32) -> booru_rs::Result<Box<dyn Post>> {
        self.get_by_id(id)
        .await
        .map(|post| Box::new(post) as _)
    }

    async fn get_wrapper(&self) -> booru_rs::Result<Vec<Box<dyn Post>>> {
        self.get()
        .await
        .map(|posts|
            posts.into_iter().map(|post| Box::new(post) as _).collect()
        )
    }
}