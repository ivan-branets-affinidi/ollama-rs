use std::path::PathBuf;

use ollama_rs::{
    coordinator::Coordinator,
    generation::{
        chat::ChatMessage,
        options::GenerationOptions,
        parameters::{FormatType, JsonSchema, JsonStructure},
    },
    Ollama,
};

use serde::Deserialize;

/// Get the weather for a given city.
///
/// * city - City to get the weather for.
#[ollama_rs::function]
async fn get_weather(city: String) -> Result<String, Box<dyn std::error::Error + Sync + Send>> {
    println!("Get weather function called for {city}");
    Ok(
        reqwest::get(format!("https://wttr.in/{city}?format=%C+%t+%w+%P"))
            .await?
            .text()
            .await?,
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let ollama = Ollama::default();

    let history = vec![];
    let tools = ollama_rs::tool_group![get_weather];

    let format = FormatType::StructuredJson(JsonStructure::new::<Weather>());

    let mut coordinator =
        Coordinator::new_with_tools(ollama, "llama3.2".to_string(), history, tools)
            .format(format)
            .options(GenerationOptions::default().temperature(0.0));

    let user_messages = vec!["What's the weather in Nanaimo?"];

    for user_message in user_messages {
        println!("User: {user_message}");

        let user_message = ChatMessage::user(user_message.to_owned());
        let resp = coordinator.chat(vec![user_message]).await?;
        println!("Assistant: {}", resp.message.content);
    }

    Ok(())
}

#[allow(dead_code)]
#[derive(JsonSchema, Deserialize, Debug)]
struct Weather {
    city: String,
    temperature_units: String,
    temperature: f32,
    wind_units: String,
    wind: f32,
    pressure_units: String,
    pressure: f32,
}
