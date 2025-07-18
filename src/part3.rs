use std::collections::HashMap;
use kalosm::language::*;

use crate::helpers::create_session;

pub struct State {}

pub fn initialize_state() -> State {
    return State {};
}

pub async fn chat_with_user(
    model: &Llama,
    message: String,
    username: String,
    state: &mut State
) -> String {
    return String::from("Hello, I am not a bot (yet)!");
}
