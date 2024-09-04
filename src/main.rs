use std::{collections::HashMap, sync::Arc};

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
    #[arg(long, default_value = "./config.json")]
    pub vss_config: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub base_url: String,
    pub embedding_model_name: String,
    pub embedding_base_url: String,
    pub vss: Arc<VssState>,
    pub client: Arc<Client<hyper::client::HttpConnector, hyper::Body>>,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct VssState {
    pub limit: u64,
    pub score_threshold: Option<f32>,
    pub collections: HashMap<String, String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    let args = AppArgs::parse();

    let s = std::fs::read_to_string(&args.vss_config).expect("failed to read config file");
    let vss: VssState = serde_json::from_str(&s).expect("failed to parse config file");

    let client = Client::new();
    let app_state = AppState {
        base_url: args.base_url.trim_end_matches('/').to_string(),
        embedding_model_name: args.embedding_model_name,
        embedding_base_url: args.embedding_base_url.trim_end_matches('/').to_string(),
        vss: Arc::new(vss),
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
    log::info!("listening on {}", addr);
    axum::Server::from_tcp(tcp_listener.into_std().unwrap())
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
