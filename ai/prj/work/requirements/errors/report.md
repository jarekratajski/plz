# Report: Retry on Command Errors

## Status
Implemented.

## What was done
- Added failure-retry flow: when command execution fails, `plz` now asks the agent for an alternative command.
- Implemented maximum retry count of 3 retries after the initial failed attempt.
- Added retry prompt context containing:
  - original task,
  - previous failed command,
  - error output truncated to first 512 characters.
- Updated command execution to capture output while still printing stdout/stderr to the user.
- Re-applied existing risk classification and policy checks for every regenerated command.

## Validation
- `cargo check` passes.
- `cargo test` passes (12 tests, all green).

## Notes
- Error context prefers `stderr`; falls back to `stdout` when needed.
- If no output is present, retry context uses a fallback message.
