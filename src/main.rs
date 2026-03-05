mod claude;
mod claude_cli;

use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};
use std::process;

/// plz — run natural language commands in your shell
#[derive(Parser)]
#[command(name = "plz", about = "Turn natural language into shell commands",
    after_help = "\
CONFIGURATION:
  plz uses Claude to generate commands. Two backends are supported:

  1. Claude CLI (preferred)
     Install from: https://docs.anthropic.com/en/docs/claude-cli
     If 'claude' is in your $PATH it will be used automatically.
     No API key needed — the CLI handles authentication.

  2. HTTP API (fallback)
     Get an API key at: https://console.anthropic.com/
     Then set it in your shell:
       export ANTHROPIC_API_KEY=\"sk-ant-...\"
     Add the export to ~/.bashrc or ~/.zshrc for persistence."
)]
struct Args {
    /// Safe mode: only safe commands can run
    #[arg(short = 's', conflicts_with = "force")]
    safe: bool,

    /// Force mode: safe and moderate run automatically, dangerous asks confirmation
    #[arg(short = 'f', conflicts_with = "safe")]
    force: bool,

    /// Verbose mode: report which backend is used and other diagnostics
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Natural language description of what to do
    #[arg(required = true, trailing_var_arg = true)]
    description: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExecutionMode {
    Default,
    Safe,
    Force,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RiskLevel {
    Safe,
    Moderate,
    Dangerous,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PolicyAction {
    Execute,
    Confirm,
    Reject,
}

#[derive(Debug, PartialEq, Eq)]
struct RiskAssessment {
    level: RiskLevel,
    reason: &'static str,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {err:#}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let description = args.description.join(" ");
    let mode = execution_mode_from_args(&args);

    eprintln!("Asking Claude how to: {description}");

    let command = generate_command_with_fallback(&description, args.verbose).await?;
    let risk_assessment = classify_risk(&command);
    let policy_action = decide_policy(mode, risk_assessment.level);

    println!("\nProposed command:\n  {command}\n");
    println!("Risk: {:?} ({})\n", risk_assessment.level, risk_assessment.reason);

    match policy_action {
        PolicyAction::Execute => {
            let exit_status = execute_command(&command)?;
            if !exit_status.success() {
                process::exit(exit_status.code().unwrap_or(1));
            }
        }
        PolicyAction::Confirm => {
            if !confirm("Execute?")? {
                eprintln!("Aborted.");
                return Ok(());
            }

            let exit_status = execute_command(&command)?;
            if !exit_status.success() {
                process::exit(exit_status.code().unwrap_or(1));
            }
        }
        PolicyAction::Reject => {
            eprintln!("Rejected: {:?} command not allowed in this mode.", risk_assessment.level);
            return Ok(());
        }
    }

    Ok(())
}

async fn generate_command_with_fallback(description: &str, verbose: bool) -> Result<String> {
    if claude_cli::is_claude_cli_available() {
        if verbose {
            eprintln!("Using claude CLI");
        }
        match claude_cli::generate_command_via_cli(description) {
            Ok(command) => return Ok(command),
            Err(err) => {
                if verbose {
                    eprintln!("claude CLI failed: {err:#}, falling back to HTTP API");
                }
            }
        }
    } else if verbose {
        eprintln!("claude CLI not found, using HTTP API");
    }

    claude::generate_command(description).await
}

fn execution_mode_from_args(args: &Args) -> ExecutionMode {
    if args.safe {
        ExecutionMode::Safe
    } else if args.force {
        ExecutionMode::Force
    } else {
        ExecutionMode::Default
    }
}

fn decide_policy(mode: ExecutionMode, risk_level: RiskLevel) -> PolicyAction {
    match mode {
        ExecutionMode::Default => match risk_level {
            RiskLevel::Safe => PolicyAction::Execute,
            RiskLevel::Moderate => PolicyAction::Confirm,
            RiskLevel::Dangerous => PolicyAction::Reject,
        },
        ExecutionMode::Safe => match risk_level {
            RiskLevel::Safe => PolicyAction::Execute,
            RiskLevel::Moderate | RiskLevel::Dangerous => PolicyAction::Reject,
        },
        ExecutionMode::Force => match risk_level {
            RiskLevel::Safe | RiskLevel::Moderate => PolicyAction::Execute,
            RiskLevel::Dangerous => PolicyAction::Confirm,
        },
    }
}

fn classify_risk(command: &str) -> RiskAssessment {
    let command_lower = command.to_lowercase();

    if contains_any(&command_lower, &["sudo ", "sudo\n", "sudo\t"]) {
        return RiskAssessment {
            level: RiskLevel::Dangerous,
            reason: "uses sudo",
        };
    }

    if touches_system_or_config(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Dangerous,
            reason: "touches system or user config files",
        };
    }

    if has_high_impact_delete(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Dangerous,
            reason: "high-impact or broad delete operation",
        };
    }

    if has_non_standard_install(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Dangerous,
            reason: "installs non-standard tools",
        };
    }

    if has_external_api_call(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Moderate,
            reason: "calls external network/api",
        };
    }

    if has_known_popular_install(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Moderate,
            reason: "installs popular tooling",
        };
    }

    if has_low_impact_delete(&command_lower) {
        return RiskAssessment {
            level: RiskLevel::Moderate,
            reason: "deletes files",
        };
    }

    if contains_any(
        &command_lower,
        &[
            "docker start",
            "docker stop",
            "docker rm",
            "docker container",
            "docker compose up",
            "docker compose down",
            "docker run",
        ],
    ) {
        return RiskAssessment {
            level: RiskLevel::Safe,
            reason: "docker lifecycle command on developer machine",
        };
    }

    if contains_any(
        &command_lower,
        &[
            "cat ",
            "ls ",
            "find ",
            "grep ",
            "head ",
            "tail ",
            "wc ",
            "echo ",
            "touch ",
            "mkdir ",
            "cp ",
            "mv ",
            "git ",
        ],
    ) {
        return RiskAssessment {
            level: RiskLevel::Safe,
            reason: "read or common local file operation",
        };
    }

    RiskAssessment {
        level: RiskLevel::Moderate,
        reason: "unknown command pattern (conservative default)",
    }
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| text.contains(needle))
}

fn touches_system_or_config(command: &str) -> bool {
    contains_any(
        command,
        &[
            "/etc/",
            "/usr/",
            "/bin/",
            "/sbin/",
            "/var/",
            "/boot/",
            "/root/",
            "~/.config/",
            "$home/.config/",
            ".config/",
        ],
    )
}

fn has_high_impact_delete(command: &str) -> bool {
    contains_any(
        command,
        &[
            "rm -rf /",
            "rm -rf ~",
            "rm -rf *",
            "find / -delete",
            "find . -delete",
            "shred ",
            "wipefs ",
            "mkfs",
        ],
    )
}

fn has_low_impact_delete(command: &str) -> bool {
    contains_any(command, &["rm ", "unlink ", "rmdir ", "trash "])
}

fn has_known_popular_install(command: &str) -> bool {
    contains_any(
        command,
        &[
            "npm install",
            "pnpm add",
            "yarn add",
            "pip install",
            "cargo add",
            "apt install",
            "dnf install",
            "brew install",
        ],
    )
}

fn has_non_standard_install(command: &str) -> bool {
    contains_any(command, &["curl ", "wget "])
        && contains_any(command, &["| sh", "| bash", "bash -c", "sh -c"])
}

fn has_external_api_call(command: &str) -> bool {
    contains_any(command, &["curl ", "wget ", "http://", "https://", "nc ", "ncat ", "telnet "])
}

fn confirm(prompt: &str) -> Result<bool> {
    print!("{prompt} [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(is_confirmation_yes(&input))
}

fn is_confirmation_yes(input: &str) -> bool {
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

fn execute_command(command: &str) -> Result<process::ExitStatus> {
    let status = process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(process::Stdio::inherit())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .status()?;

    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_yes_variants() {
        let yes_inputs = ["y", "Y", "yes", "YES", "Yes"];
        for input in yes_inputs {
            assert!(is_confirmation_yes(input), "{input} should be treated as yes");
        }
    }

    #[test]
    fn test_confirm_no_variants() {
        let no_inputs = ["n", "N", "no", "", "maybe", "quit"];
        for input in no_inputs {
            assert!(!is_confirmation_yes(input), "{input} should be treated as no");
        }
    }

    #[test]
    fn test_policy_matrix_default_mode() {
        assert_eq!(
            decide_policy(ExecutionMode::Default, RiskLevel::Safe),
            PolicyAction::Execute
        );
        assert_eq!(
            decide_policy(ExecutionMode::Default, RiskLevel::Moderate),
            PolicyAction::Confirm
        );
        assert_eq!(
            decide_policy(ExecutionMode::Default, RiskLevel::Dangerous),
            PolicyAction::Reject
        );
    }

    #[test]
    fn test_policy_matrix_safe_mode() {
        assert_eq!(
            decide_policy(ExecutionMode::Safe, RiskLevel::Safe),
            PolicyAction::Execute
        );
        assert_eq!(
            decide_policy(ExecutionMode::Safe, RiskLevel::Moderate),
            PolicyAction::Reject
        );
        assert_eq!(
            decide_policy(ExecutionMode::Safe, RiskLevel::Dangerous),
            PolicyAction::Reject
        );
    }

    #[test]
    fn test_policy_matrix_force_mode() {
        assert_eq!(
            decide_policy(ExecutionMode::Force, RiskLevel::Safe),
            PolicyAction::Execute
        );
        assert_eq!(
            decide_policy(ExecutionMode::Force, RiskLevel::Moderate),
            PolicyAction::Execute
        );
        assert_eq!(
            decide_policy(ExecutionMode::Force, RiskLevel::Dangerous),
            PolicyAction::Confirm
        );
    }

    #[test]
    fn test_classifier_examples() {
        let safe = classify_risk("docker stop $(docker ps -q)");
        assert_eq!(safe.level, RiskLevel::Safe);

        let moderate = classify_risk("curl https://api.github.com/repos/rust-lang/rust");
        assert_eq!(moderate.level, RiskLevel::Moderate);

        let dangerous = classify_risk("sudo rm -rf /tmp/some_dir");
        assert_eq!(dangerous.level, RiskLevel::Dangerous);
    }
}
