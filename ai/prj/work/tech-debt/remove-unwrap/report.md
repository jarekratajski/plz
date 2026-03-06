# Report: Remove unwrap() Usage

## Changes Made

### 1. Replaced `.map(...).unwrap_or(false)` with `.is_ok_and(...)` (2 sites)

- **copilot_enterprise.rs** `is_available()`: `.map(|v| !v.is_empty()).unwrap_or(false)` → `.is_ok_and(|v| !v.is_empty())`
- **claude_cli.rs** `is_available()`: `.map(|output| output.status.success()).unwrap_or(false)` → `.is_ok_and(|output| output.status.success())`

More idiomatic — `is_ok_and()` (stable since Rust 1.70) expresses "check if Ok and satisfies predicate" in a single call.

### 2. Replaced `.unwrap_or(ApiError{...})` with `.unwrap_or_else(|_| ApiError{...})` (3 sites)

- **copilot_enterprise.rs** error response parsing
- **claude_api.rs** error response parsing
- **openai_api.rs** error response parsing

`unwrap_or_else` is lazy — the fallback struct is only constructed when deserialization actually fails, avoiding unnecessary allocation on the happy path.

## Kept As-Is (with justification)

| Site | Pattern | Reason |
|---|---|---|
| main.rs:128,140 | `.unwrap_or(1)` | Idiomatic — OS may not provide exit code |
| session.rs:28-29 | `.unwrap_or_else(\|_\| ...)` | Already uses `unwrap_or_else`; correct XDG/HOME fallback |
| session.rs:42, claude_cli.rs:108 | `.unwrap_or_default()` on `duration_since(UNIX_EPOCH)` | Only fails if system clock is before 1970 — returns 0, acceptable |
| claude_cli.rs:42 | `.unwrap_or_default()` on `Option<String>` | Returns empty string when no last interaction — correct behavior |
| All test code | bare `.unwrap()` | Standard Rust test practice — panics produce clear test failures |

## Tests

All 27 tests pass.
