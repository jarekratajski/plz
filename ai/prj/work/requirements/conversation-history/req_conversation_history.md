````markdown
# Goal

When using the Claude CLI backend, maintain conversation history so that
follow-up commands can reference previous interactions. This enables natural
flows like:

```
$ plz find all TODO comments in rust files
$ plz now do the same but for python
$ plz count how many there were total
```

## Motivation

Shell tasks are often iterative — the user refines, repeats, or builds on
previous commands. By leveraging Claude CLI's built-in conversation/session
support, `plz` can understand referential language ("that", "same", "again",
"those files") without the user restating full context.

This is only feasible with Claude CLI because it natively supports
conversation sessions. HTTP API vendors would require storing and resending
full message history on every call, increasing token cost significantly.

## Requirements

### 1. Session management (Claude CLI only)

- Use Claude CLI's `--resume` / `--session-id` flag (or equivalent) to
  maintain a persistent conversation session.
- Store session identifier in `~/.local/share/plz/session_id` (or
  equivalent XDG-compliant path).
- Each invocation resumes the existing session, providing Claude with
  context of previous commands and results.

### 2. Session scope

- A session persists across invocations until explicitly cleared or until
  a configurable timeout/staleness (suggest: session expires after 1 hour
  of inactivity, or on terminal restart — implementation detail).
- If the session file is missing or the session has expired, start a new
  session transparently.

### 3. History content

Each interaction should record:
- The user's natural language description
- The generated command
- Whether it was executed, and if so, the exit code

This gives the AI context like "last time you asked to find TODOs in rust
files and I generated `grep -rn 'TODO' --include='*.rs'` which succeeded".

### 4. No-context flag (`-nc`)

Add a `--no-context` / `-nc` flag that disables conversation history for
a single invocation:

```
plz -nc list files
```

- When `-nc` is used, do not resume any existing session — run as a
  one-shot command with no history, same as current behaviour.
- The existing session is **not** cleared — the next call without `-nc`
  will resume it normally.
- `-nc` has no effect on non-CLI vendors (they already have no context).

### 5. Other vendors unaffected

- HTTP API vendors (Claude API, OpenAI, Copilot Enterprise) continue to
  operate statelessly. Conversation history is a Claude CLI-only feature.
- If `-nc` is passed with a non-CLI vendor, it is silently accepted (no
  error).

### 6. Help text

Update `--help` CONFIGURATION section to mention:
- Conversation history is available with Claude CLI backend
- The `-nc` flag to disable it

### 7. README

Update `README.md`:
- Add `-nc` / `--no-context` to the Options table
- Add a short "Conversation history" section explaining the feature and
  its limitation to Claude CLI

### 8. Verbose output

When `-v` is active:
- Print `"Resuming session: <session_id>"` when resuming
- Print `"Starting new session"` when creating a new one
- Print `"No-context mode: skipping session"` when `-nc` is used

## Non-goals

- Conversation history for HTTP API vendors.
- Persistent cross-terminal sessions (sessions are per-terminal/per-user).
- A command to list or manage past sessions.
- Configurable history depth beyond session expiry.
````
