#[macro_use]
extern crate thiserror;

mod resolver;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use resolver::QueryRoot;
use tokio::net::TcpListener;

pub type BlogSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<BlogSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn notfound_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "not found")
}

#[tokio::main]
async fn main() {
    let server = async {
        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

        let app = Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .fallback(notfound_handler)
            .layer(Extension(schema));

        let addr = TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(addr, app).await.unwrap();
    };

    tokio::join!(server);
}
