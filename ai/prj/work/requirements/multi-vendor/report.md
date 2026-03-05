````markdown
# Report: Multi-Vendor Abstraction

## Status: Done

## Changes

1. **`src/vendor.rs`** (new) — `CommandGenerator` trait + `CommandGeneratorBoxed` for dyn dispatch,
   `select_vendors()` priority list, `generate_command_with_fallback()` chain
2. **`src/vendors/mod.rs`** (new) — re-exports vendor modules
3. **`src/vendors/claude_cli.rs`** (new) — moved from `src/claude_cli.rs`, implements `CommandGenerator`
4. **`src/vendors/claude_api.rs`** (new) — moved from `src/claude.rs`, implements `CommandGenerator`
5. **`src/vendors/openai_api.rs`** (new) — OpenAI/ChatGPT backend, implements `CommandGenerator`
6. **`src/main.rs`** — replaced `mod claude; mod claude_cli;` with `mod vendor; mod vendors;`,
   uses `vendor::generate_command_with_fallback()`, updated `--help` with all 3 backends
7. **Deleted** `src/claude.rs`, `src/claude_cli.rs` (replaced by vendors/)
8. **`ai/prj/architecture/details.md`** — updated with new structure

## Auto-detection priority
1. Claude CLI (`claude` in PATH)
2. Claude HTTP API (`ANTHROPIC_API_KEY` set)
3. OpenAI HTTP API (`OPENAI_API_KEY` set)

## Tests
- 16 tests pass, 0 failures, 0 warnings
- New tests: OpenAI response/error parsing, availability check

## Problems
- None

````
