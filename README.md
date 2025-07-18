# Chatbot Homework - Summer@Brown [Introduction to Computer Science with the Rust Programming Language](https://catalog.precollege.brown.edu/detail/CECS0935)

This homework will show you how you can build a chatbot agent using Facebook/Meta's Llama Large Language Model (LLM).

## Instructions

### Download and Initialize

First, download and compile the code provided in this repo.

On GitHub, click on the code button on the top right of the central panel, then click `download zip`. After downloading, extract the zip archive to your desktop folder. Alternatively, you can clone this repo using `git`.

After you download and extract the code, open the resulting folder in VSCode, then open the terminal inside VSCode and execute the following command
```
cargo run --release --features part1
```

This command will download the Llama LLM model and compile all the dependencies. It will take a while, up to 15 minutes on slow computers. Do not interrupt it.

You will know that the code is compiled and ready when you see a message about launching `rocket` on address `127.0.0.1:3000`. When you see that message, open `index.html` using your favorite web browser, type in any username you like in the textbox, and begin chatting with the LLM. You will notice that the chat functionality outputs a fixed message and that the chatbot agent is not configured yet. Your task is to implement it.

### Part 1: Simple chatbot

You need to implement the `chat_with_user` function located in `src/part1.rs`.

The function is given two parameters: `model`, which is an instance of the Llama LLM, and `message` the string message that users type into the chatbot webpage. Your job is use the model to get the chatbot's response, and return it from the function.

Your code must perform the following steps:
1. Use the `create_session` function, which is provided to you, to create a chat session with the LLM. The session stores the conversation history with the LLM, and is responsible for prompting it and acquiring responses from it. 
```rust
let chat_session = create_session(model);
```
2. Pass the chat message to the bot and get the bot's response via the chat session. You can do this using:
```rust
let bot_response = chat_message.add_message(message).await.unwrap();
```
Look at this line carefully, it contains a call to the `add_message` function, which is provided by the chat session library. It also uses `.await`, which instructs the language to wait until the LLM finishes computing the response. Finally, it invokes `.unwrap()`, which extracts the returned bot response from the containing enum (we have seen `unwrap` before with the `Option` enum!).
3. Finally, return the bot response.

After you successfully complete this part, you can run `cargo run --release --features part1`, and you will be able to chat with LLM when you open `index.html` in your browser.

Note: the chatbot will take a little while to compute its response, up to 3-4 minutes on slow computers.

Try to tell the LLM your name, and then afterwards, ask it to say your name back. Do you notice anything weird? Does the chatbot remember the conversation history?

### Part 2: Chatbot with conversation history

In this part, you will extend your earlier solution to keep track of the conversation history. You need to implement the `chat_with_user` and `initialize_state` functions in `src/part2.rs`.

The provided code will invoke `initalize_state` when you first start the chat bot. It provides the function with the Llama model as a parameter, and it expects the function to return an instance of type `State`.

Your code inside `initialize_state` should invoke `create_session`, and then store the resulting `chat_session` inside an instance of `State` and return it. You will need to update the definition of the `State` struct (line 5) to include a chat session field of an appropriate type.

In part 2, the `chat_with_user` function is given two parameters, the message from the user, as well as a reference to the state that is returned by `initialize_state`. If you implement `initialize_state` correctly, the state should have a `chat_session` stored inside it. Thus, you can directly pass the chat message to it to get the bot's response:
```rust
let bot_response = state.chat_message.add_message(message).await.unwrap();
```

You can run your code to test it using `cargo run --release --features part2`. If your implementation is correct, your chatbot should remember previous messages in the conversation.

Try to have a conversation with the LLM, then, open `index.html` again, and log in with a different username. Can you get the LLM to spill secret details from the previous conversation from another username? Is this good or bad behavior?

### Part 3: Chatbot with a different conversation history for each logged in user

In this last part, you will implement new versions of `initialize_state` and of `chat_with_user` located in `src/part3.rs`.

The idea is to store multipe `chat_sessions` in the `State`. Indeed, you need to store one `chat_session` for each `username`. You will need to:
1. Modify the definition of the `State` struct (line 6) to add a field that can store many `chat_sessions`, one per username.
2. Create an empty instance of `State` and return it inside the body of `initialize_state`.
3. In `chat_with_user`, first, check if a `chat_session` exists inside `state` for the given `username`. There are two cases:
   1. A `chat_session` exists: retrieve it, and pass the message to it to retrieve the bot's response (`using .add_message(...).await.unwrap()`).
   2. No `chat_session` exists: create a new one using the given `model`, insert it into the state, and then pass the message to it and retrieve the bot's resposne (similar to (i)).

Hint: consider having a HashMap in your state mapping `usernames` to `chat_sessions`.

You can run your code using `cargo run --release --features part3`. If your implementation is correct, you should be able to log in using different usernames in different tabs in your browser, and have separate conversations with the chatbot in each.


### Submission

After you are done, submit the contents of `src/part1.rs`, `src/part2.rs`, and `src/part3.rs` via the Google form linked in Canvas.
