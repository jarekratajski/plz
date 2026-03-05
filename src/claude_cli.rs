use anyhow::{anyhow, Context, Result};
use std::process::Command;

use crate::claude::SYSTEM_PROMPT;

pub fn is_claude_cli_available() -> bool {
    Command::new("which")
        .arg("claude")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn generate_command_via_cli(description: &str) -> Result<String> {
    let prompt = format!("{SYSTEM_PROMPT}\n\n{description}");

    let output = Command::new("claude")
        .arg("-p")
        .arg(&prompt)
        .arg("--output-format")
        .arg("text")
        .output()
        .context("Failed to execute claude CLI")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let truncated_error = truncate_to_char_boundary(&stderr, 512);
        return Err(anyhow!(
            "claude CLI exited with {}: {}",
            output.status,
            truncated_error
        ));
    }

    let command = String::from_utf8(output.stdout)
        .context("claude CLI returned non-UTF-8 output")?;

    let trimmed = command.trim().to_string();
    if trimmed.is_empty() {
        return Err(anyhow!("claude CLI returned empty output"));
    }

    Ok(trimmed)
}

fn truncate_to_char_boundary(text: &str, max_bytes: usize) -> &str {
    if text.len() <= max_bytes {
        return text;
    }
    let mut end = max_bytes;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_to_char_boundary_short_string() {
        let text = "hello";
        assert_eq!(truncate_to_char_boundary(text, 512), "hello");
    }

    #[test]
    fn test_truncate_to_char_boundary_exact() {
        let text = "abcdef";
        assert_eq!(truncate_to_char_boundary(text, 3), "abc");
    }

    #[test]
    fn test_truncate_to_char_boundary_multibyte() {
        let text = "héllo"; // é is 2 bytes, so "hé" = 3 bytes
        // At max_bytes=2 the boundary falls inside é, so we truncate to "h"
        assert_eq!(truncate_to_char_boundary(text, 2), "h");
        // At max_bytes=3 we get the full "hé"
        assert_eq!(truncate_to_char_boundary(text, 3), "hé");
    }

    #[test]
    fn test_is_claude_cli_available_returns_bool() {
        // Just verifies the function runs without panic; actual result depends on system
        let _available = is_claude_cli_available();
    }
}
