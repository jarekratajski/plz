# Plan: Retry on Command Errors

## Scope
When an executed command fails, collect error output, pass the first 512 characters to the agent as retry context, and try alternative command generation up to 3 retries.

## Steps
1. Add retry loop in runtime flow
- Keep original natural-language request.
- On failure, request a new command from the agent with error context.
- Stop after maximum 3 retries.

2. Capture execution output for analysis
- Replace status-only execution with captured `Output`.
- Preserve user-visible stdout/stderr printing.
- Determine failure from exit status.

3. Build retry context payload
- Prefer stderr as error source (fallback to stdout if needed).
- Truncate captured error text to first 512 characters.
- Include previous command and truncated error in retry prompt.

4. Maintain safety policy checks on each retry
- Recompute risk and mode policy for newly generated commands.
- Reject/confirm/execute according to existing mode behavior.

5. Add tests
- Test error truncation helper (max 512 chars).
- Test retry prompt contains required context.

6. Validate
- Run `cargo check` and `cargo test`.
- Add report file after completion.
