use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";
const MODEL: &str = "claude-opus-4-6";
const MAX_TOKENS: u32 = 1024;

const SYSTEM_PROMPT: &str = "\
You are a bash command generator. Given a natural language description of a task, \
output ONLY the bash command or script needed to accomplish it. \
Rules:
- Output only raw bash commands, nothing else
- No markdown, no code fences, no explanation
- Use a single line if possible; use newlines only if multiple commands are truly needed
- Prefer safe, commonly available tools
- Never output anything other than the command itself";

#[derive(Serialize)]
struct Request {
    model: &'static str,
    max_tokens: u32,
    system: &'static str,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct Response {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct ApiError {
    error: ApiErrorDetail,
}

#[derive(Deserialize)]
struct ApiErrorDetail {
    message: String,
}

pub async fn generate_command(description: &str) -> Result<String> {
    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .context("ANTHROPIC_API_KEY environment variable not set. Please set it to your Anthropic API key.")?;

    let request = Request {
        model: MODEL,
        max_tokens: MAX_TOKENS,
        system: SYSTEM_PROMPT,
        messages: vec![Message {
            role: "user".to_string(),
            content: description.to_string(),
        }],
    };

    let client = reqwest::Client::new();
    let response = client
        .post(ANTHROPIC_API_URL)
        .header("x-api-key", &api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("content-type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to connect to Claude API")?;

    let status = response.status();
    let body = response.text().await.context("Failed to read API response")?;

    if !status.is_success() {
        let api_err: ApiError = serde_json::from_str(&body)
            .unwrap_or(ApiError {
                error: ApiErrorDetail {
                    message: body.clone(),
                },
            });
        return Err(anyhow!("Claude API error ({}): {}", status, api_err.error.message));
    }

    let response: Response = serde_json::from_str(&body)
        .context("Failed to parse Claude API response")?;

    let command = response
        .content
        .into_iter()
        .find(|block| block.block_type == "text")
        .and_then(|block| block.text)
        .ok_or_else(|| anyhow!("Claude returned no text content"))?;

    Ok(command.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_parsing() {
        let json = r#"{
            "content": [
                {"type": "text", "text": "docker stop $(docker ps -q)"}
            ],
            "id": "msg_123",
            "model": "claude-opus-4-6",
            "role": "assistant",
            "stop_reason": "end_turn",
            "type": "message",
            "usage": {"input_tokens": 10, "output_tokens": 5}
        }"#;

        let response: Response = serde_json::from_str(json).unwrap();
        let text = response
            .content
            .into_iter()
            .find(|b| b.block_type == "text")
            .and_then(|b| b.text)
            .unwrap();

        assert_eq!(text, "docker stop $(docker ps -q)");
    }

    #[test]
    fn test_error_parsing() {
        let json = r#"{"error": {"type": "authentication_error", "message": "Invalid API key"}}"#;
        let err: ApiError = serde_json::from_str(json).unwrap();
        assert_eq!(err.error.message, "Invalid API key");
    }
}
