# Implementation Details

## Project Structure

```
plz/
├── Cargo.toml       — dependencies and project config
├── src/
│   ├── main.rs      — CLI entry point, confirmation, command execution
│   ├── claude.rs    — Claude HTTP API client
│   ├── claude_cli.rs— Claude CLI backend (preferred when available)
│   └── safety.rs    — Risk assessment and policy logic
├── plan.md          — implementation plan
├── details.md       — this file
└── goal.md          — original requirements
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `clap` | 4 | CLI argument parsing with derive macros |
| `reqwest` | 0.12 (rustls-tls) | Async HTTP client (uses rustls, no OpenSSL needed) |
| `tokio` | 1 (full) | Async runtime |
| `serde` / `serde_json` | 1 | JSON serialization/deserialization |
| `anyhow` | 1 | Ergonomic error handling with context |

**Note:** `reqwest` is configured with `default-features = false, features = ["json", "rustls-tls"]`
to avoid a hard dependency on OpenSSL system libraries.

## Claude API Integration (`src/claude.rs`)

- **Model:** `claude-opus-4-6`
- **Endpoint:** `POST https://api.anthropic.com/v1/messages`
- **Auth:** `x-api-key` header from `ANTHROPIC_API_KEY` env var
- **System prompt:** Instructs Claude to output only raw bash commands, no markdown, no explanations
- **Parsing:** Extracts the first `"text"` content block from the response

## Claude CLI Backend (`src/claude_cli.rs`)

- **Detection:** checks if `claude` executable is in `$PATH` via `which claude`
- **Invocation:** runs `claude -p "<prompt>" --output-format text` as a subprocess
- **Prompt:** reuses the same `SYSTEM_PROMPT` from `claude.rs` with the user description appended
- **Fallback:** if CLI is unavailable or fails, falls back to the HTTP API automatically
- **Error truncation:** CLI stderr is truncated to 512 bytes for error messages

## CLI Flow (`src/main.rs`)

1. `clap` collects all positional arguments with `trailing_var_arg = true`
2. They are joined with spaces into the natural language description
3. Claude API is called asynchronously
4. The proposed command is printed clearly
5. User is prompted `Execute? [y/N]:` — defaults to **No** on empty input
6. If confirmed, command runs via `sh -c <command>` with inherited stdio
7. Exit code is propagated

## Usage

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
cargo build --release
./target/release/plz stop all docker containers
# Asking Claude how to: stop all docker containers
#
# Proposed command:
#   docker stop $(docker ps -q)
#
# Execute? [y/N]: y
```

## Design Decisions

- **No unsafe code** — pure safe Rust throughout
- **Functional style** — uses iterator chains, `?` operator, minimal mutation
- **`sh -c` execution** — allows Claude to use shell features (pipes, subshells, etc.)
- **rustls over native-tls** — avoids OpenSSL system dependency, more portable
- **Default-deny confirmation** — empty enter means No, preventing accidental execution
