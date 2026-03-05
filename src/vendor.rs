use anyhow::Result;

pub const SYSTEM_PROMPT: &str = "\
You are a bash command generator. Given a natural language description of a task, \
output ONLY the bash command or script needed to accomplish it. \
Rules:
- Output only raw bash commands, nothing else
- No markdown, no code fences, no explanation
- Use a single line if possible; use newlines only if multiple commands are truly needed
- Prefer safe, commonly available tools
- Never output anything other than the command itself";

pub trait CommandGenerator: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn generate_command(&self, description: &str) -> impl std::future::Future<Output = Result<String>> + Send;
}

pub fn select_vendors() -> Vec<Box<dyn CommandGeneratorBoxed>> {
    vec![
        Box::new(crate::vendors::claude_cli::ClaudeCli),
        Box::new(crate::vendors::claude_api::ClaudeApi),
        Box::new(crate::vendors::openai_api::OpenAiApi),
    ]
}

/// Object-safe version of CommandGenerator for use with dyn dispatch.
pub trait CommandGeneratorBoxed: Send + Sync {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn generate_command_boxed<'a>(&'a self, description: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send + 'a>>;
}

impl<T: CommandGenerator> CommandGeneratorBoxed for T {
    fn name(&self) -> &str {
        CommandGenerator::name(self)
    }

    fn is_available(&self) -> bool {
        CommandGenerator::is_available(self)
    }

    fn generate_command_boxed<'a>(&'a self, description: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String>> + Send + 'a>> {
        Box::pin(CommandGenerator::generate_command(self, description))
    }
}

pub async fn generate_command_with_fallback(description: &str, verbose: bool) -> Result<String> {
    let vendors = select_vendors();
    let mut last_error = None;

    for vendor in &vendors {
        if !vendor.is_available() {
            if verbose {
                eprintln!("{} not available, skipping", vendor.name());
            }
            continue;
        }

        if verbose {
            eprintln!("Using {}", vendor.name());
        }

        match vendor.generate_command_boxed(description).await {
            Ok(command) => return Ok(command),
            Err(err) => {
                if verbose {
                    eprintln!("{} failed: {err:#}, trying next vendor", vendor.name());
                }
                last_error = Some(err);
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        anyhow::anyhow!(
            "No AI vendor available. Set ANTHROPIC_API_KEY or OPENAI_API_KEY, \
             or install the claude CLI. Run plz --help for details."
        )
    }))
}
