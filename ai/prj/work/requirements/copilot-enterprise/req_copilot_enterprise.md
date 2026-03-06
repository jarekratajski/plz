````markdown
# Goal

Add GitHub Copilot Enterprise as a supported AI backend. When the user provides
a GitHub token (with Copilot Enterprise access), `plz` can use GitHub's Copilot
Chat API to generate bash commands.

## Motivation

- Many organisations already have GitHub Copilot Enterprise licences — their
  developers shouldn't need a separate API key from another provider.
- Expands the set of available backends, giving users more flexibility.
- The Copilot Chat completions API follows the OpenAI-compatible format, making
  integration straightforward.

## Requirements

### 1. New vendor: `copilot`

Implement a new `CommandGenerator` for GitHub Copilot Enterprise.

- **Vendor id:** `copilot`
- **Display name:** `GitHub Copilot Enterprise`
- **Endpoint:** `POST https://api.githubcopilot.com/chat/completions`
- **Auth:** `Authorization: Bearer $GITHUB_TOKEN` and
  `Editor-Version: plz/1.0.0` header.
- **Model:** `gpt-4o` (Copilot's default model).
- **System prompt:** reuse the existing `SYSTEM_PROMPT` constant.
- **Request format:** OpenAI-compatible chat completions (same structure as
  the existing OpenAI vendor).
- **Response parsing:** extract `choices[0].message.content`, same as OpenAI.

### 2. Availability detection

The vendor is available when the `GITHUB_TOKEN` environment variable is set
and non-empty.

### 3. Priority

Insert the new backend at **priority 4** (after OpenAI, before giving up):

| Priority | Backend |
|----------|---------|
| 1 | Claude CLI |
| 2 | Claude HTTP API |
| 3 | OpenAI (ChatGPT) HTTP API |
| **4** | **GitHub Copilot Enterprise** |

### 4. `--vendor` flag

Accept `copilot` as a valid `--vendor` value:

```
plz --vendor copilot list files
```

### 5. Help text

Update the `--help` `CONFIGURATION` section to include:

```
  4. GitHub Copilot Enterprise
     Requires a GitHub token with Copilot Enterprise access:
       export GITHUB_TOKEN="ghp_..."
```

And append to the `--vendor` list:

```
    --vendor copilot      GitHub Copilot Enterprise
```

### 6. README

Update `README.md`:

- Add a row to the **Configuration** priority table for Copilot Enterprise.
- Add `copilot` to the `--vendor` usage examples.
- Add a row to the **Options** table vendor description.

### 7. Error handling

- If the API returns an error, parse and display the error message (reuse the
  same pattern as the OpenAI vendor).
- Follow existing retry logic (up to 3 retries as defined in req_errors).

### 8. Token guidance

When Copilot Enterprise is forced via `--vendor copilot` but `GITHUB_TOKEN` is
not set, exit with a clear message:

```
GitHub Copilot Enterprise is not available. Run plz --help for setup instructions.
```

## Non-goals

- Copilot Individual (non-Enterprise) support (different auth flow).
- Persisting token in a config file — environment variable only.
- OAuth device flow — user must provide the token directly.
````
