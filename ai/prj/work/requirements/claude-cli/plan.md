````markdown
# Plan: Claude CLI Backend

## Overview
Add a `claude` CLI backend that is preferred over the HTTP API when available.

## Steps

1. **Refactor `claude.rs`** — make `SYSTEM_PROMPT` public so both backends share it
2. **Create `claude_cli.rs`** — new module with:
   - `is_claude_cli_available()` — checks `which claude`
   - `generate_command_via_cli(description)` — invokes `claude -p` with the system prompt + user request, returns the command string
3. **Update `main.rs`**:
   - Add `mod claude_cli;`
   - In `run()`, try `claude_cli` first; on failure or unavailability, fall back to `claude::generate_command`
   - Print which backend is being used (CLI vs API) for transparency
4. **Tests**:
   - Unit test for CLI availability check (mock via PATH manipulation)
   - Unit test for output parsing from CLI
5. **Build, run `cargo test`, fix warnings**
6. **Update `details.md`** with new module docs
7. **Write `report.md`**

````
