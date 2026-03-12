````markdown
# Goal

Add GitHub Copilot CLI as a supported AI backend. The `gh copilot suggest` command
(part of the GitHub CLI `gh` with the Copilot extension) can generate shell commands
from natural language descriptions.

## Motivation

- Users with GitHub Copilot access can use it without any API key, just the `gh` CLI
  with the Copilot extension installed.
- Similar model to the Claude CLI backend — no HTTP calls, no token management,
  just a subprocess.

## Requirements

### 1. New vendor: `copilot-cli`

Implement a new `CommandGenerator` in `src/vendors/copilot_cli.rs`.

- **Vendor id:** `copilot-cli`
- **Display name:** `GitHub Copilot CLI`
- **Invocation:** subprocess `gh copilot suggest -t shell "<description>"`
- **Availability check:** `which gh` succeeds AND `gh copilot --version` (or similar)
  succeeds — i.e. the Copilot extension is installed.
- **Output:** parse the suggested command from stdout (investigate exact output format
  of `gh copilot suggest` before implementing).
- No session/conversation history support (stateless, like the HTTP vendors).

### 2. Priority

Insert at **priority 2** (after Claude CLI, before Claude HTTP API):

| Priority | Backend |
|----------|---------|
| 1 | Claude CLI |
| **2** | **GitHub Copilot CLI** |
| 3 | Claude HTTP API |
| 4 | OpenAI (ChatGPT) HTTP API |

### 3. `--vendor` flag

Accept `copilot-cli` as a valid `--vendor` value:

```
plz --vendor copilot-cli list files
```

### 4. Help text and README

Update `--help` CONFIGURATION section and `README.md` to document the new backend
(availability condition: `gh` installed with the Copilot extension).

## Non-goals

- GitHub Copilot Enterprise HTTP API (was tried and did not work — see reverted report).
- Conversation history for this backend.
````
