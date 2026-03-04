# Implementation Plan: `plz` CLI Tool

## Overview
A Rust CLI tool that takes natural language descriptions and uses the Claude API to generate and execute bash commands.

## Steps

### Step 1: Project Setup
- [x] Read goal.md
- [x] Initialize Rust project with `cargo init`
- [x] Set up `Cargo.toml` with dependencies (using rustls-tls to avoid OpenSSL dep)

### Step 2: Dependencies
- `clap` — CLI argument parsing
- `reqwest` — HTTP client for Claude API calls
- `tokio` — Async runtime
- `serde` / `serde_json` — JSON serialization
- `anyhow` — Error handling

### Step 3: Core Modules [DONE]
Structure:
```
src/
  main.rs       — Entry point, CLI orchestration, confirmation, execution
  claude.rs     — Claude API client (raw HTTP via reqwest)
```

### Step 4: CLI Argument Parsing [DONE]
- `clap` with `trailing_var_arg = true` collects all args
- e.g. `plz stop all docker containers` → "stop all docker containers"

### Step 5: Claude API Integration [DONE]
- `claude-opus-4-6` model via reqwest HTTP calls
- API key from `ANTHROPIC_API_KEY` env var
- System prompt: output only raw bash, no markdown, no explanation
- Parses first text content block from response

### Step 6: User Confirmation [DONE]
- Shows: `Proposed command:\n  <cmd>\n\nExecute? [y/N]:`
- Defaults to No on empty input

### Step 7: Command Execution [DONE]
- `sh -c <command>` with inherited stdio
- Exit code propagated

### Step 8: Error Handling [DONE]
- Missing API key → clear message
- API errors → status code + message
- Command failure → propagated exit code

### Step 9: Tests [DONE]
- Unit tests: response parsing, error parsing, confirm yes/no variants
- All 4 tests pass

## API Design
- Request: POST /v1/messages
- Model: `claude-opus-4-6`
- System prompt: "You are a bash command generator. Given a natural language description, output ONLY the bash command or commands needed to accomplish the task. Output only the raw command, no markdown, no explanation, no code fences."
- User message: the natural language input

## Key Decisions
- No Rust Anthropic SDK exists → use `reqwest` for raw HTTP
- Functional style: minimize mutability, use `Result` chains
- No unsafe code
