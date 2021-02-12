use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_warp::Response;
use sqlx::postgres::PgPool;
use std::convert::Infallible;
use std::env;
use warp::{http::Response as HttpResponse, Filter};

mod records;
mod schema;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct AuthToken(String);
struct QueryRoot;
struct MutationRoot;

pub async fn db_connection() -> Result<PgPool> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
    Ok(PgPool::connect(&database_url).await?)
}

#[tokio::main]
async fn main() {
    let pg_pool: PgPool = db_connection().await.expect("Database connection failed.");
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Database migrations failed");

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pg_pool)
        .finish();

    let graphql_post = warp::path("graphql")
        .and(warp::post())
        .and(warp::header::optional("token"))
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |token,
             (schema, mut request): (
                Schema<QueryRoot, MutationRoot, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                if let Some(token) = token {
                    request = request.data(AuthToken(token));
                }
                let resp = schema.execute(request).await;
                Ok::<_, Infallible>(Response::from(resp))
            },
        );

    let graphql_playground = warp::path("playground").and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
    });

    let static_files = warp::path("static").and(warp::fs::dir("./static"));

    let catch_all = warp::any().map(move || {
        let body = include_str!("index.html");
        warp::reply::html(body)
    });

    let routes = static_files
        .or(graphql_post)
        .or(graphql_playground)
        .or(catch_all);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
