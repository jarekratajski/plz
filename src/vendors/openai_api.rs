use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::vendor::{CommandGenerator, SYSTEM_PROMPT};

const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-4o";
const MAX_TOKENS: u32 = 1024;

pub struct OpenAiApi;

#[derive(Serialize)]
struct Request {
    model: &'static str,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct Response {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChoiceMessage,
}

#[derive(Deserialize)]
struct ChoiceMessage {
    content: Option<String>,
}

#[derive(Deserialize)]
struct ApiError {
    error: ApiErrorDetail,
}

#[derive(Deserialize)]
struct ApiErrorDetail {
    message: String,
}

impl CommandGenerator for OpenAiApi {
    fn name(&self) -> &str {
        "OpenAI HTTP API"
    }

    fn vendor_id(&self) -> &str {
        "chatgpt"
    }

    fn is_available(&self) -> bool {
        std::env::var("OPENAI_API_KEY").is_ok()
    }

    async fn generate_command(&self, description: &str) -> Result<String> {
        let api_key = std::env::var("OPENAI_API_KEY")
            .context("OPENAI_API_KEY environment variable not set")?;

        let request = Request {
            model: MODEL,
            max_tokens: MAX_TOKENS,
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: SYSTEM_PROMPT.to_string(),
                },
                Message {
                    role: "user".to_string(),
                    content: description.to_string(),
                },
            ],
        };

        let client = reqwest::Client::new();
        let response = client
            .post(OPENAI_API_URL)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to OpenAI API")?;

        let status = response.status();
        let body = response.text().await.context("Failed to read API response")?;

        if !status.is_success() {
            let api_err: ApiError = serde_json::from_str(&body)
                .unwrap_or(ApiError {
                    error: ApiErrorDetail {
                        message: body.clone(),
                    },
                });
            return Err(anyhow!("OpenAI API error ({}): {}", status, api_err.error.message));
        }

        let response: Response = serde_json::from_str(&body)
            .context("Failed to parse OpenAI API response")?;

        let command = response
            .choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .ok_or_else(|| anyhow!("OpenAI returned no content"))?;

        Ok(command.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_parsing() {
        let json = r#"{
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": "ls -lt | head -1"
                    },
                    "finish_reason": "stop"
                }
            ],
            "usage": {"prompt_tokens": 10, "completion_tokens": 5, "total_tokens": 15}
        }"#;

        let response: Response = serde_json::from_str(json).unwrap();
        let text = response
            .choices
            .into_iter()
            .next()
            .and_then(|c| c.message.content)
            .unwrap();

        assert_eq!(text, "ls -lt | head -1");
    }

    #[test]
    fn test_error_parsing() {
        let json = r#"{"error": {"message": "Incorrect API key", "type": "invalid_request_error", "code": "invalid_api_key"}}"#;
        let err: ApiError = serde_json::from_str(json).unwrap();
        assert_eq!(err.error.message, "Incorrect API key");
    }

    #[test]
    fn test_is_available_depends_on_env() {
        let vendor = OpenAiApi;
        let _available = vendor.is_available();
    }
}
