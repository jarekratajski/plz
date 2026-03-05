````markdown
# Report: Claude CLI Backend

## Status: Done

## Changes

1. **`src/claude.rs`** — made `SYSTEM_PROMPT` public so both backends share the same prompt
2. **`src/claude_cli.rs`** (new) — CLI backend with:
   - `is_claude_cli_available()` — checks `which claude`
   - `generate_command_via_cli()` — invokes `claude -p` subprocess
   - UTF-8 safe error truncation to 512 bytes
3. **`src/main.rs`** — added `generate_command_with_fallback()`:
   - Tries CLI first when available
   - Falls back to HTTP API on CLI absence or failure
   - Prints which backend is used
4. **`ai/prj/architecture/details.md`** — added CLI backend section

## Tests
- 12 tests pass, 0 failures, 0 warnings
- New tests cover truncation logic and availability check

## Problems
- None

````
