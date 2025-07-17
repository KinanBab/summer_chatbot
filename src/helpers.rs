use kalosm::language::*;

// You can use the returned chat session as follows:
// let bot_response = <CHAT SESSION>.add_message(<HUMAN MESSAGE>).await.unwrap();
pub fn create_session(model: &Llama) -> Chat<Llama> {
    return model.chat().with_system_prompt("The assistant will act like a pirate");
}
