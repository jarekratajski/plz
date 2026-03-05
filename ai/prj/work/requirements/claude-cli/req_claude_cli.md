````markdown
# Goal

Instead of always calling the Claude HTTP API directly, the `plz` tool should
first check whether the `claude` CLI command is available on the system.
If it is, use the CLI to get the bash command; only fall back to the HTTP API
when the CLI is not found.

## Motivation

- The `claude` CLI handles authentication, model selection, and session
  management on its own — no need for `ANTHROPIC_API_KEY` when it is present.
- Reduces configuration burden for users who already have the CLI installed.
- Keeps the HTTP API path as a fallback so the tool still works everywhere.

## Requirements

1. **Detection** — on startup, check if the `claude` executable is available
   in `$PATH` (e.g. `which claude` or equivalent).
2. **CLI invocation** — when `claude` is available, invoke it as a subprocess
   to obtain the bash command. Pass the same system prompt / instructions used
   for the HTTP API so the output format is identical (raw bash, no markdown,
   no explanations).
   Suggested invocation:
   ```
   claude -p "<system_prompt>\n<user_request>"
   ```
   (use `--output-format text` or equivalent flags if available to get plain
   text output).
3. **Fallback** — when `claude` is not found, fall back to the existing HTTP
   API path (requires `ANTHROPIC_API_KEY`).
4. **Consistent output** — the rest of the flow (display proposed command,
   ask for confirmation, execute) must remain unchanged regardless of which
   backend was used.
5. **Error handling** — if the `claude` CLI invocation fails (non-zero exit,
   empty output, timeout), treat it as an error and apply the existing retry
   logic (up to 3 retries as defined in req_errors).

## Non-goals

- Do not remove or deprecate the HTTP API path.
- Do not add new CLI flags to `plz` for choosing the backend — detection
  should be automatic.

````
