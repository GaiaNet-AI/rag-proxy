use std::vec;

use axum::{body::Body, extract::State, http::Response, Json};
use endpoints::{
    chat::{
        ChatCompletionRequest, ChatCompletionRequestMessage, ChatCompletionUserMessage,
        ChatCompletionUserMessageContent,
    },
    embeddings::{EmbeddingObject, EmbeddingsResponse},
};

use crate::AppState;

pub async fn chat(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(data): Json<ChatCompletionRequest>,
) -> Response<Body> {
    let x_forwarded_host = headers
        .get("x-forwarded-host")
        .map(|v| v.to_str().ok())
        .flatten();

    match chat_impl(state, data, x_forwarded_host).await {
        Ok(resp) => resp,
        Err(e) => Response::builder()
            .status(500)
            .body(Body::from(e.to_string()))
            .unwrap(),
    }
}

fn merge_rag_context(raw_text: &str, context: &[&str]) -> String {
    let context = context.join("\n\n");
    format!(
        "{context}\nAnswer the question based on the pieces of context above. The question is:\n{raw_text}",
    )
}

async fn update_context_by_rag(
    state: &AppState,
    raw_text: &str,
    point: &[f64],
    collection_name: &str,
) -> anyhow::Result<String> {
    let scored_points = state
        .vss
        .client
        .search_points(
            collection_name,
            point,
            state.vss.limit.max(1),
            state.vss.score_threshold,
        )
        .await?;

    if !scored_points.is_empty() {
        let mut context = vec![];
        for point in scored_points.iter() {
            if let Some(payload) = point.payload.as_ref() {
                if let Some(payload) = payload.get("source").map(|s| s.as_str()).flatten() {
                    context.push(payload);
                }
            };
        }

        if context.is_empty() {
            Ok(raw_text.to_string())
        } else {
            Ok(merge_rag_context(raw_text, &context))
        }
    } else {
        Ok(raw_text.to_string())
    }
}

async fn embedding_text(state: &AppState, text: &str) -> anyhow::Result<Vec<EmbeddingObject>> {
    let uri = format!("{}/embeddings", state.vss.embedding_base_url);
    let body = serde_json::json!({
        "model":state.vss.embedding_model_name,
        "input": [text]
    });

    let req = hyper::Request::post(uri)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&body)?))?;

    let resp = state.client.request(req).await?;
    let body = hyper::body::to_bytes(resp.into_body()).await?;
    let embedding_response: EmbeddingsResponse = serde_json::from_slice(&body)?;
    Ok(embedding_response.data)
}

async fn chat_impl(
    state: AppState,
    mut data: ChatCompletionRequest,
    x_forwarded_host: Option<&str>,
) -> anyhow::Result<Response<Body>> {
    let uri = format!("{}/chat/completions", state.base_url);

    let collection_name = x_forwarded_host
        .and_then(|s| s.split_once('.'))
        .and_then(|(s, _)| Some(s))
        .unwrap_or("default");

    if let Some(ChatCompletionRequestMessage::User(user_msg)) = data.messages.last_mut() {
        if let ChatCompletionUserMessageContent::Text(text) = user_msg.content() {
            let embeddings = embedding_text(&state, text).await?;
            if let Some(points) = embeddings.first() {
                let rag_text =
                    update_context_by_rag(&state, text, &points.embedding, collection_name).await?;
                let name = user_msg.name().cloned();
                let content = ChatCompletionUserMessageContent::Text(rag_text);
                *user_msg = ChatCompletionUserMessage::new(content, name)
            }
        }
    }

    let req = hyper::Request::post(uri)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&data)?))?;

    Ok(state.client.request(req).await?)
}
