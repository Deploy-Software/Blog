use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_warp::Response;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter};

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping<'a>(&self, _ctx: &'a Context<'_>) -> &'a str {
        "Pong!"
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();
    let graphql_post = warp::path("graphql")
        .and(warp::post())
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |(schema, request): (
                Schema<QueryRoot, EmptyMutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
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

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
