use std::sync::{Arc};
use kalosm::language::Llama;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::State;

#[cfg(any(feature = "part2", feature = "part3"))]
use std::ops::DerefMut;
#[cfg(any(feature = "part2", feature = "part3"))]
use rocket::tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    username: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    message: String,
}

#[cfg(feature = "part1")]
#[post("/", format = "json", data = "<payload>")]
pub async fn chat(
    payload: Json<ChatRequest>,
    model: &State<Arc<Llama>>
) -> Json<ChatResponse> {
    let message = payload.message.clone();

    // Call solution function.
    let message = crate::part1::chat_with_user(model, message).await;

    // Return the result.
    return Json(ChatResponse { message });
}

#[cfg(feature = "part2")]
#[post("/", format = "json", data = "<payload>")]
pub async fn chat(
    payload: Json<ChatRequest>,
    state: &State<Arc<Mutex<crate::part2::State>>>,
    _model: &State<Arc<Llama>>
) -> Json<ChatResponse> {
    let message = payload.message.clone();
    let mut lock = state.lock().await;

    // Call solution function.
    let message = crate::part2::chat_with_user(
        message,
        lock.deref_mut()
    ).await;

    // Return the result.
    return Json(ChatResponse { message });
}

#[cfg(feature = "part3")]
#[post("/", format = "json", data = "<payload>")]
pub async fn chat(
    payload: Json<ChatRequest>,
    state: &State<Arc<Mutex<crate::part3::State>>>,
    model: &State<Arc<Llama>>
) -> Json<ChatResponse> {
    let username = payload.username.clone();
    let message = payload.message.clone();
    let mut lock = state.lock().await;

    // Call solution function.
    let message = crate::part3::chat_with_user(
        model,
        message,
        username,
        lock.deref_mut()
    ).await;

    // Return the result.
    return Json(ChatResponse { message });
}
