use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::vendor::{CommandGenerator, SYSTEM_PROMPT};

const COPILOT_API_URL: &str = "https://api.githubcopilot.com/chat/completions";
const MODEL: &str = "gpt-4o";
const MAX_TOKENS: u32 = 1024;

pub struct CopilotEnterprise;

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

impl CommandGenerator for CopilotEnterprise {
    fn name(&self) -> &str {
        "GitHub Copilot Enterprise"
    }

    fn vendor_id(&self) -> &str {
        "copilot"
    }

    fn is_available(&self) -> bool {
        std::env::var("GITHUB_TOKEN")
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }

    async fn generate_command(&self, description: &str, verbose: bool, _no_context: bool) -> Result<String> {
        let token = std::env::var("GITHUB_TOKEN")
            .context("GITHUB_TOKEN environment variable not set")?;

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
        if verbose {
            if let Ok(json) = serde_json::to_string_pretty(&request) {
                eprintln!("--- Request body ---");
                eprintln!("{json}");
                eprintln!("--------------------");
            }
        }
        let client = reqwest::Client::new();
        let response = client
            .post(COPILOT_API_URL)
            .header("Authorization", format!("Bearer {token}"))
            .header("Editor-Version", "plz/1.0.0")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to connect to GitHub Copilot API")?;

        let status = response.status();
        let body = response.text().await.context("Failed to read API response")?;

        if !status.is_success() {
            let api_err: ApiError = serde_json::from_str(&body)
                .unwrap_or(ApiError {
                    error: ApiErrorDetail {
                        message: body.clone(),
                    },
                });
            return Err(anyhow!("GitHub Copilot API error ({}): {}", status, api_err.error.message));
        }

        let response: Response = serde_json::from_str(&body)
            .context("Failed to parse GitHub Copilot API response")?;

        let command = response
            .choices
            .into_iter()
            .next()
            .and_then(|choice| choice.message.content)
            .ok_or_else(|| anyhow!("GitHub Copilot returned no content"))?;

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
        let json = r#"{"error": {"message": "Bad credentials", "type": "unauthorized", "code": "unauthorized"}}"#;
        let err: ApiError = serde_json::from_str(json).unwrap();
        assert_eq!(err.error.message, "Bad credentials");
    }

    #[test]
    fn test_is_available_depends_on_env() {
        let vendor = CopilotEnterprise;
        let _available = vendor.is_available();
    }
}
