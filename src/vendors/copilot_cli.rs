use anyhow::{anyhow, Context, Result};
use std::process::Command;

use crate::vendor::CommandGenerator;

pub struct CopilotCli;

impl CommandGenerator for CopilotCli {
    fn name(&self) -> &str {
        "GitHub Copilot CLI"
    }

    fn vendor_id(&self) -> &str {
        "copilot-cli"
    }

    fn is_available(&self) -> bool {
        let gh_found = Command::new("which")
            .arg("gh")
            .output()
            .is_ok_and(|output| output.status.success());

        if !gh_found {
            return false;
        }

        Command::new("gh")
            .args(["copilot", "--version"])
            .output()
            .is_ok_and(|output| output.status.success())
    }

    async fn generate_command(&self, description: &str, verbose: bool, _no_context: bool) -> Result<String> {
        if verbose {
            eprintln!("--- Request ---");
            eprintln!("gh copilot suggest -t shell {:?}", description);
            eprintln!("---------------");
        }

        let output = Command::new("gh")
            .args(["copilot", "suggest", "-t", "shell", description])
            .output()
            .context("Failed to execute gh copilot CLI")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!(
                "gh copilot exited with {}: {}",
                output.status,
                stderr.trim()
            ));
        }

        let stdout = String::from_utf8(output.stdout)
            .context("gh copilot returned non-UTF-8 output")?;

        let command = parse_suggestion(&stdout)
            .ok_or_else(|| anyhow!("gh copilot returned no recognisable suggestion:\n{stdout}"))?;

        Ok(command)
    }
}

/// Extract the suggested command from `gh copilot suggest` output.
///
/// Expected format:
/// ```
/// Suggestion:
///
///   <command>
///
/// ? Select an option ...
/// ```
///
/// If the "Suggestion:" marker is not found, falls back to the first non-empty trimmed line.
fn parse_suggestion(output: &str) -> Option<String> {
    let mut lines = output.lines();

    // Try to find the line after "Suggestion:"
    while let Some(line) = lines.next() {
        if line.trim().eq_ignore_ascii_case("suggestion:") {
            // Skip blank lines and return the first non-empty content line
            for candidate in lines {
                let trimmed = candidate.trim();
                if !trimmed.is_empty() {
                    return Some(trimmed.to_string());
                }
            }
            return None;
        }
    }

    // Fallback: first non-empty trimmed line
    output
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_suggestion_standard_format() {
        let output = "Suggestion:\n\n  ls -la\n\n? Select an option\n";
        assert_eq!(parse_suggestion(output), Some("ls -la".to_string()));
    }

    #[test]
    fn test_parse_suggestion_no_marker_fallback() {
        let output = "\n  find . -name '*.rs'\n";
        assert_eq!(
            parse_suggestion(output),
            Some("find . -name '*.rs'".to_string())
        );
    }

    #[test]
    fn test_parse_suggestion_empty_output() {
        assert_eq!(parse_suggestion(""), None);
        assert_eq!(parse_suggestion("   \n  \n"), None);
    }

    #[test]
    fn test_is_available_returns_bool() {
        let vendor = CopilotCli;
        let _available = vendor.is_available();
    }
}
