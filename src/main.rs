use std::sync::Arc;

use axum::{routing::post, Router};
use clap::Parser;
use hyper::Client;
use tokio::net::TcpListener;

mod chat;
mod embeddings;
mod vss;

#[derive(Debug, Clone, Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
pub struct AppArgs {
    #[arg(long, default_value = "0.0.0.0:8181")]
    pub lister_addr: String,
    #[arg(short, long, default_value = "http://localhost:8080/v1")]
    pub base_url: String,
    #[arg(long, default_value = "embedding")]
    pub embedding_model_name: String,
    #[arg(long, default_value = "http://localhost:8080/v1")]
    pub embedding_base_url: String,
    #[arg(long, default_value = "http://localhost:6663")]
    pub vss_url: String,
    #[arg(short('l'), long, default_value = "3")]
    pub vss_limit: u64,
    #[arg(short('c'), long)]
    pub vss_score_threshold: Option<f32>,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub base_url: String,
    pub vss: Arc<RagState>,
    pub client: Arc<Client<hyper::client::HttpConnector, hyper::Body>>,
}

#[derive(Clone, Debug)]
pub struct RagState {
    pub embedding_model_name: String,
    pub embedding_base_url: String,
    pub client: vss::Client,
    pub limit: u64,
    pub score_threshold: Option<f32>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = AppArgs::parse();

    let client = Client::new();
    let app_state = AppState {
        base_url: args.base_url.trim_end_matches('/').to_string(),
        vss: Arc::new(RagState {
            embedding_model_name: args.embedding_model_name,
            embedding_base_url: args.embedding_base_url.trim_end_matches('/').to_string(),
            client: vss::Client::new_with_url(args.vss_url.trim_end_matches('/').to_string()),
            score_threshold: args.vss_score_threshold,
            limit: args.vss_limit,
        }),
        client: Arc::new(client),
    };

    // build our application with a route
    let app = Router::new()
        .route("/v1/chat/completions", post(chat::chat))
        .route("/v1/embeddings", post(embeddings::embeddings))
        .with_state(app_state);

    // run it
    let addr = &args.lister_addr;
    let tcp_listener = TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", addr);
    axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
