# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Rules and Guidelines

Follow the rules in `ai/rules/general.md`. Key points:

- Before implementing any feature, create `plan.md` in the feature's work folder — this is required, never skip it
- After implementation, write `report.md` summarizing what was done and any issues
- Never modify a `req_*.md` file that already has a `report.md` (it's done); create a new requirement folder instead
- Read `ai/rules/code.md` for coding guidelines
- Project requirements live in `ai/work/requirements/`, tech-debt in `ai/work/tech-debt/`
- Architecture decisions go in `ai/prj/architecture/` (keep short and human-readable)

## Commands

```bash
cargo build --release          # Build optimized binary
cargo build                    # Build debug binary
cargo test                     # Run all tests
cargo test <test_name>         # Run a single test
cargo install --path .         # Install `plz` binary to PATH
cargo clippy                   # Lint
```

## Architecture

**plz** is a Rust CLI tool that translates natural language into shell commands using AI backends, with a safety-first execution model.

### Core Flow

1. Parse CLI args (`main.rs`) → generate command via vendor (`vendor.rs`) → classify risk (`safety.rs`) → decide policy → execute or prompt user

### Modules

- **`main.rs`**: CLI entry point (clap), execution orchestration, confirmation prompts, `sh -c <cmd>` subprocess execution
- **`vendor.rs`**: `CommandGenerator` trait + `CommandGeneratorBoxed` (object-safe for dyn dispatch), `SYSTEM_PROMPT`, vendor auto-detection with priority fallback
- **`safety.rs`**: `classify_risk()` heuristic pattern matching → `RiskLevel`, `decide_policy(mode, risk)` → `PolicyAction`
- **`session.rs`**: Conversation history for Claude CLI backend only; sessions stored at `~/.local/share/plz/session.json` with 1-hour expiry
- **`vendors/`**: Four backends — `claude_cli` (subprocess, has session support), `claude_api` (Anthropic HTTP, stateless), `openai_api` (OpenAI, stateless), `copilot_enterprise` (GitHub Copilot, stateless)

### Safety System

**Risk levels**: Safe → Moderate → Dangerous (classified by pattern matching in `safety.rs`)

**Policy matrix** (mode × risk → action):

|              | Safe    | Moderate | Dangerous |
|--------------|---------|----------|-----------|
| Default      | Execute | Confirm  | Reject    |
| Safe (`-s`)  | Execute | Reject   | Reject    |
| Force (`-f`) | Execute | Execute  | Confirm   |

### Vendor Priority (auto-detection)

1. Claude CLI (`claude` binary available)
2. Claude HTTP API (`ANTHROPIC_API_KEY`)
3. OpenAI API (`OPENAI_API_KEY`)
4. GitHub Copilot Enterprise (`GITHUB_TOKEN`)

Use `--vendor <claude-cli|claude|chatgpt|copilot>` to force a specific backend.

### Extending with a New Vendor

Implement the `CommandGenerator` trait in a new file under `src/vendors/`, then add it to the `select_vendors()` vector in `vendor.rs`.
