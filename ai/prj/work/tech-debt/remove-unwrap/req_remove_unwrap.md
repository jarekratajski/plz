# Remove unwrap() Usage

## Summary

Audit and remove `unwrap()` calls from production code, replacing them with proper error handling (`?` operator, `anyhow::Context`, or explicit match/if-let). Test code may keep `unwrap()` where appropriate.

## Current State

### Production Code

The production code uses `unwrap_or` variants which won't panic but may silently swallow errors:

- **main.rs:128,140** — `exit_status.code().unwrap_or(1)` — acceptable, OS may not provide exit code
- **copilot_enterprise.rs:62** — `.unwrap_or(false)` — hides env var parse errors
- **copilot_enterprise.rs:106** — `.unwrap_or(ApiError{...})` — hides deserialization errors for error responses
- **claude_api.rs:100** — `.unwrap_or(ApiError{...})` — same pattern
- **openai_api.rs:105** — `.unwrap_or(ApiError{...})` — same pattern
- **session.rs:28-29** — `.unwrap_or_else(...)` on XDG/HOME dir resolution — acceptable fallback
- **session.rs:42** — `.unwrap_or_default()` — silently returns empty on file read/parse failure
- **claude_cli.rs:23** — `.unwrap_or(false)` — hides `which` command check errors
- **claude_cli.rs:42** — `.unwrap_or_default()` — silently returns empty on command output failure
- **claude_cli.rs:108** — `.unwrap_or_default()` — silently returns empty on output extraction

### Test Code (acceptable, no changes needed)

Bare `.unwrap()` calls in `#[cfg(test)]` modules across all vendor files and session.rs — standard Rust test practice.

## Requirements

1. Replace `unwrap_or` / `unwrap_or_default` in production code with proper error propagation (`?` with `anyhow::Context`) where the calling function returns `Result`
2. Where the function does not return `Result`, consider refactoring the signature or using `unwrap_or` with a logged warning (via `eprintln!` in verbose mode)
3. Keep `unwrap_or(1)` for exit codes in main.rs — this is idiomatic
4. Keep `unwrap_or_else` for XDG/HOME fallback in session.rs — this is a reasonable default
5. Keep `unwrap()` in test code — this is standard Rust practice
6. Focus on cases where silently swallowing errors hides real problems (e.g., session.rs:42, claude_cli.rs:42,108)

## Priority

Low — production code already avoids bare `unwrap()`. This is about improving error visibility, not preventing panics.
