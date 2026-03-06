use anyhow::{anyhow, Context, Result};
use std::process::Command;

use crate::session;
use crate::vendor::{CommandGenerator, SYSTEM_PROMPT};

pub struct ClaudeCli;

impl CommandGenerator for ClaudeCli {
    fn name(&self) -> &str {
        "Claude CLI"
    }

    fn vendor_id(&self) -> &str {
        "claude-cli"
    }

    fn is_available(&self) -> bool {
        Command::new("which")
            .arg("claude")
            .output()
            .is_ok_and(|output| output.status.success())
    }

    async fn generate_command(&self, description: &str, verbose: bool, no_context: bool) -> Result<String> {
        let (prompt, session, is_new_session) = if no_context {
            if verbose {
                eprintln!("No-context mode: skipping session");
            }
            (format!("{SYSTEM_PROMPT}\n\n{description}"), None, false)
        } else {
            match session::load_session() {
                Some(existing) => {
                    if verbose {
                        eprintln!("Resuming session: {}", existing.session_id);
                    }
                    let context = existing
                        .last_interaction
                        .as_ref()
                        .map(session::context_prefix)
                        .unwrap_or_default();
                    (
                        format!("{SYSTEM_PROMPT}\n\n{context}{description}"),
                        Some(existing),
                        false,
                    )
                }
                None => {
                    let new_session = session::new_session();
                    if verbose {
                        eprintln!("Starting new session: {}", new_session.session_id);
                    }
                    (
                        format!("{SYSTEM_PROMPT}\n\n{description}"),
                        Some(new_session),
                        true,
                    )
                }
            }
        };

        if verbose {
            eprintln!("--- Request body ---");
            eprintln!("{prompt}");
            eprintln!("--------------------");
        }

        let mut cmd = Command::new("claude");
        cmd.arg("-p")
            .arg(&prompt)
            .arg("--output-format")
            .arg("text");

        if let Some(ref s) = session {
            if is_new_session {
                cmd.arg("--session-id").arg(&s.session_id);
            } else {
                cmd.arg("--resume").arg(&s.session_id);
            }
        } else {
            cmd.arg("--no-session-persistence");
        }

        let output = cmd.output().context("Failed to execute claude CLI")?;

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

        if let Some(mut s) = session {
            s.last_used = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            s.last_interaction = Some(session::Interaction {
                description: description.to_string(),
                command: trimmed.clone(),
                executed: false,
                exit_code: None,
            });
            let _ = session::save_session(&s);
        }

        Ok(trimmed)
    }
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
        assert_eq!(truncate_to_char_boundary(text, 2), "h");
        assert_eq!(truncate_to_char_boundary(text, 3), "hé");
    }

    #[test]
    fn test_is_available_returns_bool() {
        let vendor = ClaudeCli;
        let _available = vendor.is_available();
    }
}
