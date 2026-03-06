````markdown
# Goal

When the `-v` (verbose) flag is active, `plz` should print the full message
body that is sent to the AI agent, so the user can inspect exactly what is
being asked.

## Motivation

The verbose flag already reports which backend is selected. But users
debugging prompt issues or wanting transparency also need to see the actual
request content (system prompt + user message) that is transmitted to the AI.

## Requirements

1. **Print request body** — in verbose mode, before sending the request to
   any vendor, print the full message body to **stderr**.
2. **Format** — wrap the output in clear delimiters:
   ```
   --- Request body ---
   <JSON or text body>
   --------------------
   ```
   For HTTP API vendors (Claude API, OpenAI, Copilot Enterprise), print the
   serialised JSON request body.
   For CLI vendors (Claude CLI), print the prompt string passed to the
   subprocess.
3. **Quiet by default** — without `-v`, nothing extra is printed (existing
   behaviour unchanged).
4. **Output channel** — all verbose output goes to **stderr** so it does not
   interfere with piped stdout.

## Non-goals

- Printing the response body (only the request is logged).
- Redacting API keys from the output (they are in headers, not the body).
````
