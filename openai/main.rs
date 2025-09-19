use async_openai::{
    types::{
        ChatCompletionRequestMessage,
        ChatCompletionRequestUserMessage,
        CreateChatCompletionRequestArgs,
        Role,
    },
    Client,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new();

    // Build a User message properly
    let user_message = ChatCompletionRequestMessage::User(
        ChatCompletionRequestUserMessage {
            role: Role::User,
            content: "Hello! Write me a short Rust poem.".into(),
            name: None,
        }
    );

    // Build the request
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .messages([user_message])
        .build()?;

    // Send request
    let response = client.chat().create(request).await?;

    // Print the response
    if let Some(content) = &response.choices[0].message.content {
        println!("{}", content);
    }

    Ok(())
}
