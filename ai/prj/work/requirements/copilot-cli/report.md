````markdown
# Report

## Status: Done

## Changes

1. **`src/vendors/copilot_cli.rs`** — new vendor implementing `CommandGenerator`.
   Runs `gh copilot suggest -t shell "<description>"` as a subprocess.
   Availability requires both `gh` in PATH and the Copilot extension installed
   (`gh copilot --version` succeeds). Output parsed by finding the first non-empty
   line after `Suggestion:`, with a fallback to the first non-empty line of output.
   No session support (stateless).

2. **`src/vendors/mod.rs`** — registered `copilot_cli` module.

3. **`src/vendor.rs`** — inserted `CopilotCli` at priority 2 (after Claude CLI,
   before Claude HTTP API).

4. **`src/main.rs`** — added `copilot-cli` to `--vendor` value parser and help text.

5. **`README.md`** — added GitHub Copilot CLI row to the configuration table,
   `--vendor copilot-cli` usage example, updated Options table and stateless-backends note.

## Problems

Output parsing relies on `gh copilot suggest` printing `Suggestion:` followed by
the command. If the actual output format differs, the fallback (first non-empty line)
kicks in. Needs real-world testing to confirm the parsing is correct.
````
