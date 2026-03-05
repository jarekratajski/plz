````markdown
# Goal

Add a `-v` (verbose) flag to `plz` that makes the tool report what is happening
during execution.

## Motivation

Currently the tool is mostly silent about its internals.  With `-v`, the user
can see which backend was used (Claude CLI vs HTTP API), timing, and other
diagnostic details — useful for debugging and transparency.

## Requirements

1. **New flag** — add `-v` / `--verbose` boolean flag to the CLI args.
   It must not conflict with existing flags (`-s`, `-f`).
2. **Backend reporting** — in verbose mode, print which backend is being used:
   - `"Using claude CLI"` when the local `claude` command is found and invoked
   - `"claude CLI not found, using HTTP API"` when falling back
   - `"claude CLI failed (<reason>), falling back to HTTP API"` on CLI error
3. **Quiet by default** — without `-v`, behaviour stays exactly as today
   (only the proposed command, risk, and confirmation prompt are shown).
4. **Output channel** — all verbose messages go to **stderr** so they don't
   interfere with piped stdout.

## Non-goals

- Multiple verbosity levels (e.g. `-vv`) — not needed now.
- Logging to a file.

````
