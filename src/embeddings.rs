use axum::{body::Body, extract::State, http::Request, response::Response};

use crate::AppState;

pub async fn embeddings(State(state): State<AppState>, mut req: Request<Body>) -> Response<Body> {
    let uri = format!("{}/embeddings", state.vss.embedding_base_url);
    *req.uri_mut() = uri.parse().unwrap();
    match state.client.request(req).await {
        Ok(resp) => resp,
        Err(e) => Response::builder()
            .status(500)
            .body(Body::from(e.to_string()))
            .unwrap(),
    }
}
