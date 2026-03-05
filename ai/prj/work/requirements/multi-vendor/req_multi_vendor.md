````markdown
# Goal

Restructure the project so that multiple AI "vendors" (backends) can be used to
generate bash commands. Adding a new vendor should require minimal boilerplate.

## Motivation

Currently the code is tightly coupled to Claude (HTTP API + CLI). OpenAI's
ChatGPT API is equally simple to call, and users may already have an
`OPENAI_API_KEY`. The tool should auto-detect which keys/CLIs are available and
pick the best option.

## Requirements

### 1. Vendor trait abstraction

Define a trait (e.g. `CommandGenerator`) with:
```rust
trait CommandGenerator {
    fn name(&self) -> &str;
    fn is_available(&self) -> bool;
    async fn generate_command(&self, description: &str) -> Result<String>;
}
```

Each vendor implements this trait. The rest of the code works against the trait,
not concrete types.

### 2. Vendor auto-detection & priority

On startup, iterate over registered vendors in priority order and use the first
one where `is_available()` returns true.

Default priority (highest first):
1. **Claude CLI** — `claude` in `$PATH`
2. **Claude HTTP API** — `ANTHROPIC_API_KEY` env var set
3. **OpenAI HTTP API** — `OPENAI_API_KEY` env var set

### 3. OpenAI (ChatGPT) vendor

Add an OpenAI backend:
- **Endpoint:** `POST https://api.openai.com/v1/chat/completions`
- **Auth:** `Authorization: Bearer $OPENAI_API_KEY`
- **Model:** `gpt-4o` (or configurable)
- **System prompt:** same prompt used by Claude — raw bash, no markdown
- **Parsing:** extract `choices[0].message.content`

### 4. Project structure

Suggested layout:
```
src/
├── main.rs
├── safety.rs
├── vendor.rs          — trait definition + auto-detection logic
└── vendors/
    ├── mod.rs
    ├── claude_api.rs  — Claude HTTP API
    ├── claude_cli.rs  — Claude CLI
    └── openai_api.rs  — OpenAI HTTP API
```

### 5. Verbose output

With `-v`, print which vendor was selected and why (e.g.
`"Using Claude CLI (claude found in PATH)"`).

### 6. Error handling

If the selected vendor fails, fall through to the next available vendor before
giving up. Apply existing retry logic (max 3) per vendor attempt.

### 7. No manual vendor selection flag

Detection is automatic based on available keys/CLIs. No `--vendor` flag needed
(keep it simple).

### 8. Update `--help` output

Update the CONFIGURATION section in `--help` to list all supported vendors and
how to set each one up:
- **Claude CLI** — install link, auto-detected from `$PATH`
- **Claude HTTP API** — `export ANTHROPIC_API_KEY="sk-ant-..."`
- **OpenAI HTTP API** — `export OPENAI_API_KEY="sk-..."`

Include the auto-detection priority order so users know which backend will be
picked when multiple are configured.

## Non-goals

- Config file for vendor preferences
- Streaming responses
- Supporting vendors that require OAuth flows

````
