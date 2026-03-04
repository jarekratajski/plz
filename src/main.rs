mod claude;

use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};
use std::process;

/// plz — run natural language commands in your shell
#[derive(Parser)]
#[command(name = "plz", about = "Turn natural language into shell commands")]
struct Args {
    /// Natural language description of what to do
    #[arg(required = true, trailing_var_arg = true)]
    description: Vec<String>,
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

    eprintln!("Asking Claude how to: {description}");

    let command = claude::generate_command(&description).await?;

    println!("\nProposed command:\n  {command}\n");

    if !confirm("Execute?")? {
        eprintln!("Aborted.");
        return Ok(());
    }

    let exit_status = execute_command(&command)?;
    if !exit_status.success() {
        process::exit(exit_status.code().unwrap_or(1));
    }

    Ok(())
}

fn confirm(prompt: &str) -> Result<bool> {
    print!("{prompt} [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(matches!(input.trim().to_lowercase().as_str(), "y" | "yes"))
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
    #[test]
    fn test_confirm_yes_variants() {
        // confirm() itself requires stdin, so we test the parsing logic inline
        let yes_inputs = ["y", "Y", "yes", "YES", "Yes"];
        for input in yes_inputs {
            assert!(
                matches!(input.trim().to_lowercase().as_str(), "y" | "yes"),
                "{input} should be treated as yes"
            );
        }
    }

    #[test]
    fn test_confirm_no_variants() {
        let no_inputs = ["n", "N", "no", "", "maybe", "quit"];
        for input in no_inputs {
            assert!(
                !matches!(input.trim().to_lowercase().as_str(), "y" | "yes"),
                "{input} should be treated as no"
            );
        }
    }
}
