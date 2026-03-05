# Implementation Details

## Project Structure

```
plz/
├── Cargo.toml         — dependencies and project config
├── src/
│   ├── main.rs        — CLI entry point, confirmation, command execution
│   ├── vendor.rs      — CommandGenerator trait, auto-detection, fallback chain
│   ├── vendors/
│   │   ├── mod.rs     — re-exports vendor modules
│   │   ├── claude_cli.rs  — Claude CLI backend (highest priority)
│   │   ├── claude_api.rs  — Claude HTTP API backend
│   │   └── openai_api.rs  — OpenAI (ChatGPT) HTTP API backend
│   └── safety.rs      — Risk assessment and policy logic
├── plan.md            — implementation plan
├── details.md         — this file
└── goal.md            — original requirements
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

## Vendor Abstraction (`src/vendor.rs`)

- **Trait:** `CommandGenerator` with `name()`, `is_available()`, `generate_command()`
- **Object-safe wrapper:** `CommandGeneratorBoxed` enables dyn dispatch with async
- **Auto-detection:** `select_vendors()` returns vendors in priority order
- **Fallback chain:** `generate_command_with_fallback()` tries each available vendor in order
- **Shared prompt:** `SYSTEM_PROMPT` lives in `vendor.rs`, used by all backends

## Vendor: Claude CLI (`src/vendors/claude_cli.rs`) — Priority 1

- **Detection:** `which claude` in `$PATH`
- **Invocation:** `claude -p "<prompt>" --output-format text`
- **Error truncation:** stderr capped at 512 bytes

## Vendor: Claude HTTP API (`src/vendors/claude_api.rs`) — Priority 2

- **Model:** `claude-opus-4-6`
- **Endpoint:** `POST https://api.anthropic.com/v1/messages`
- **Auth:** `x-api-key` header from `ANTHROPIC_API_KEY` env var
- **Parsing:** extracts first `"text"` content block

## Vendor: OpenAI HTTP API (`src/vendors/openai_api.rs`) — Priority 3

- **Model:** `gpt-4o`
- **Endpoint:** `POST https://api.openai.com/v1/chat/completions`
- **Auth:** `Authorization: Bearer $OPENAI_API_KEY`
- **Parsing:** extracts `choices[0].message.content`

## CLI Flow (`src/main.rs`)

1. `clap` collects all positional arguments with `trailing_var_arg = true`
2. They are joined with spaces into the natural language description
3. First available vendor is called via the fallback chain
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
