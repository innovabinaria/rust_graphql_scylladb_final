mod db;
mod graphql;

use axum::{routing::get, Router, Extension};
use std::net::SocketAddr;
use std::sync::Arc;
use scylla::client::session_builder::SessionBuilder;
use graphql::{AppState, build_schema, graphql_handler};
use dotenvy::dotenv;
use std::env;
use log::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenv().ok();
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let keyspace = env::var("KEYSPACE").expect("KEYSPACE not found");

    info!("Connecting to ScyllaDB at {}...", db_url);
    let session = SessionBuilder::new()
        .known_node(db_url)
        .use_keyspace(keyspace, false)
        .build()
        .await?;


    let app_state = AppState { db: Arc::new(session) };
    let schema = build_schema(app_state);

    let app = Router::new()
        .route("/", get(|| async { "GraphQL server running. POST to /graphql" }))
        .route("/graphql", axum::routing::post(graphql_handler))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Server ready in http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app.into_make_service()).await?;
    
    Ok(())

}
