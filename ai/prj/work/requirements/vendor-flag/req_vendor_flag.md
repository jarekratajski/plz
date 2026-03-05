````markdown
# Goal

Add a `--vendor` option that lets the user force a specific AI backend instead
of relying on auto-detection.

## Requirements

1. **New flag** — add `--vendor <NAME>` (no short form) to the CLI args.
   Accepted values: `claude-cli`, `claude`, `chatgpt`.
   The flag is optional; when omitted, auto-detection works as before.
2. **Behaviour** — when `--vendor` is provided:
   - Skip the auto-detection priority chain.
   - Use only the specified vendor.
   - If that vendor is not available (e.g. missing API key or CLI), exit with
     a clear error message explaining what is needed.
3. **Verbose interaction** — with `-v`, print
   `"Vendor forced: <name>"` instead of the auto-detection messages.
4. **Help text** — list the accepted vendor names in the `--help` output
   (via clap's `value_parser` / `PossibleValues`).
5. **Update `--help` CONFIGURATION section** — mention the `--vendor` option
   and the accepted names.

## Vendor name mapping

| `--vendor` value | Backend |
|------------------|---------|
| `claude-cli`     | Claude CLI subprocess |
| `claude`         | Claude HTTP API |
| `chatgpt`        | OpenAI HTTP API |

## Non-goals

- Persisting vendor preference in a config file.
- Env-var override (e.g. `PLZ_VENDOR`).

````
