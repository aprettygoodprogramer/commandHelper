use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use serde_json::{Value, json};
use std::env;

pub fn generate_command(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    const OPENROUTER_API_URL: &str = "https://openrouter.ai/api/v1/chat/completions";
    const MODEL_NAME: &str = "google/gemini-2.5-flash-preview-05-20";

    let api_key = env::var("OPENROUTER_API_KEY")
        .map_err(|_| "Error: OPENROUTER_API_KEY environment variable not set")?;

    let system_prompt = r#"Generate a Bash command from the provided content.
    Generate only the bash command, with no additional markdown or text.
    Ex: Provided content:("A command to list all files") Your response (ls)"#;

    let request_body = json!({
        "model": MODEL_NAME,
        "messages": [
            { "role": "system", "content": system_prompt },
            { "role": "user", "content": prompt }
        ],
        "stream": false
    });

    let client = Client::new();
    let response = client
        .post(OPENROUTER_API_URL)
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .header(CONTENT_TYPE, "application/json")
        .json(&request_body)
        .send()?;

    let response_body_text = response.error_for_status()?.text()?;

    let json_response: Value = serde_json::from_str(&response_body_text)?;

    let command = json_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Could not find 'content' in API response")?
        .trim()
        .to_string();

    Ok(command)
}
