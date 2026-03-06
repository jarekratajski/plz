````markdown
# Report

## Status: Done

## Changes

1. **`src/vendor.rs`** — added `verbose: bool` parameter to
   `CommandGenerator::generate_command`, `CommandGeneratorBoxed::generate_command_boxed`,
   the blanket impl, and both `generate_command_forced` /
   `generate_command_with_fallback` call sites.

2. **`src/vendors/claude_cli.rs`** — prints the prompt string between
   `--- Request body ---` / `--------------------` delimiters when verbose.

3. **`src/vendors/claude_api.rs`** — prints the serialised JSON request body
   when verbose.

4. **`src/vendors/openai_api.rs`** — prints the serialised JSON request body
   when verbose.

5. **`src/vendors/copilot_enterprise.rs`** — prints the serialised JSON
   request body when verbose.

## Problems

None. All 20 tests pass, no warnings.
````
