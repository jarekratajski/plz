````markdown
# Report

## Status: Done

## Changes

1. **`src/vendors/copilot_enterprise.rs`** — new vendor implementing
   `CommandGenerator`. Calls `POST https://api.githubcopilot.com/chat/completions`
   with `Authorization: Bearer $GITHUB_TOKEN` and `Editor-Version: plz/1.0.0`.
   Uses OpenAI-compatible request/response format. Available when `GITHUB_TOKEN`
   is set and non-empty. Includes unit tests for response parsing, error
   parsing, and availability check.

2. **`src/vendors/mod.rs`** — registered `copilot_enterprise` module.

3. **`src/vendor.rs`** — added `CopilotEnterprise` at priority 4 in
   `select_vendors()`. Updated no-vendor-available error message to mention
   `GITHUB_TOKEN`.

4. **`src/main.rs`** — added `copilot` to `--vendor` value parser. Added
   GitHub Copilot Enterprise section to `--help` CONFIGURATION text.

5. **`README.md`** — added Copilot Enterprise row to configuration table,
   `--vendor copilot` usage example, and updated Options table.

## Problems

None. All tests pass, no warnings.
````
