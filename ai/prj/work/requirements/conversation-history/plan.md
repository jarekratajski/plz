````markdown
# Plan

## Approach

Leverage Claude CLI's native `--session-id <uuid>` flag to maintain
conversation context across invocations. Store session state in a JSON file.

## Design

### Session file: `~/.local/share/plz/session.json`

```json
{
  "session_id": "<uuid>",
  "last_used": "<ISO 8601 timestamp>",
  "last_interaction": {
    "description": "find all TODO comments",
    "command": "grep -rn 'TODO' --include='*.rs'",
    "executed": true,
    "exit_code": 0
  }
}
```

### Claude CLI invocation changes

- **Normal mode**: pass `--session-id <uuid>` to resume session. Prepend
  last interaction context to the prompt so the AI knows the outcome.
- **No-context mode** (`-n`): pass `--no-session-persistence`, omit
  `--session-id`.
- **New/expired session**: generate fresh UUID, omit context prefix.

### Context prefix (prepended to prompt when resuming)

```
[Previous: asked "{description}", generated `{command}`, {executed/not executed}, exit {code}]
```

## Steps

1. Create `src/session.rs` module with:
   - `Session` struct (session_id, last_used, last_interaction)
   - `load_session()` → loads & validates (returns None if expired/missing)
   - `save_session()` → writes session file
   - `session_dir()` → XDG-compliant path
   - `SESSION_EXPIRY` constant (1 hour)
2. Pass `no_context` through `vendor::generate_command` → vendors
3. Update `ClaudeCli::generate_command` to:
   - Load session (if not no_context)
   - Add `--session-id` flag
   - Prepend context prefix if resuming
   - Return session_id for saving after execution
4. Update `main.rs` `run()` to save session after execution
5. Add tests for session load/save/expiry
6. Build & verify
````
