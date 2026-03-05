````markdown
# Report: Vendor Flag

## Status: Done

## Changes

1. **`src/main.rs`** — added `--vendor <NAME>` optional arg with
   `value_parser = ["claude-cli", "claude", "chatgpt"]`; passed to
   `vendor::generate_command()`; updated CONFIGURATION help section
2. **`src/vendor.rs`** — added `vendor_id()` to `CommandGenerator` and
   `CommandGeneratorBoxed` traits; added `generate_command()` entry point
   that delegates to forced or fallback path; added `generate_command_forced()`
3. **`src/vendors/claude_cli.rs`** — added `vendor_id() -> "claude-cli"`
4. **`src/vendors/claude_api.rs`** — added `vendor_id() -> "claude"`
5. **`src/vendors/openai_api.rs`** — added `vendor_id() -> "chatgpt"`

## Behaviour
- `--vendor claude-cli` / `--vendor claude` / `--vendor chatgpt` forces that backend
- If the forced vendor is unavailable, exits with a clear error
- With `-v`, prints `"Vendor forced: <name>"` instead of auto-detection messages
- Without `--vendor`, auto-detection works as before
- Invalid vendor values are rejected by clap before reaching application code

## Tests
- 16 tests pass, 0 failures, 0 warnings

## Problems
- None

````
