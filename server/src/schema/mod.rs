use crate::records::posts::Post;
use crate::{MutationRoot, QueryRoot};
use async_graphql::{Context, Object, Result};

mod authorization;
mod posts;

#[Object]
impl QueryRoot {
    async fn posts<'a>(&self, ctx: &'a Context<'_>) -> Result<Vec<Post>> {
        posts::get_all(ctx).await
    }

    async fn post<'a>(&self, ctx: &'a Context<'_>, post_id: i32) -> Result<Option<Post>> {
        posts::get(ctx, post_id).await
    }

    async fn ping<'a>(&self, _ctx: &'a Context<'_>) -> &'a str {
        "Pong"
    }
}

#[Object]
impl MutationRoot {
    async fn sign_up<'a>(
        &self,
        ctx: &'a Context<'_>,
        email: String,
        password: String,
    ) -> Result<&'a str> {
        authorization::sign_up(ctx, email, password).await
    }

    async fn sign_in<'a>(
        &self,
        ctx: &'a Context<'_>,
        email: String,
        password: String,
    ) -> Result<String> {
        authorization::sign_in(ctx, email, password).await
    }

    async fn new_post<'a>(
        &self,
        ctx: &'a Context<'_>,
        title: String,
        text: String,
    ) -> Result<Post> {
        posts::new(ctx, &title, &text).await
    }

    async fn update_post<'a>(
        &self,
        ctx: &'a Context<'_>,
        post_id: i32,
        title: Option<String>,
        text: Option<String>,
    ) -> Result<&'a str> {
        posts::update(ctx, post_id, title, text).await
    }
}