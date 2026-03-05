````markdown
# Goal

The `--help` output should include a **Configuration** section that tells users
how to set up the tool — specifically how to obtain and configure an Anthropic
API key, and how to install the `claude` CLI as an alternative backend.

## Requirements

1. **Configuration section in help** — extend the `--help` / `-h` output with
   an `after_help` block (clap's `after_help` attribute) containing setup
   instructions.
2. **API key instructions** — explain:
   - Where to get an API key (https://console.anthropic.com/)
   - How to set it: `export ANTHROPIC_API_KEY="sk-ant-..."`
   - Suggest adding it to `~/.bashrc` or `~/.zshrc` for persistence
3. **Claude CLI instructions** — explain:
   - Where to install from (https://docs.anthropic.com/en/docs/claude-cli)
   - That when `claude` is in `$PATH` it will be used automatically (no API
     key needed)
   - The CLI is the preferred backend; HTTP API is the fallback
4. **Keep it concise** — the help text should be short and scannable, not a
   full tutorial

## Non-goals

- Interactive setup wizard
- Auto-detection of missing config on first run (separate concern)

````
