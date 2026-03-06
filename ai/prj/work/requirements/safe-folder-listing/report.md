````markdown
# Report

## Status: Done

## Changes

1. **`src/main.rs`** — replaced the `contains_any`-based safe-command check
   with a new `is_safe_read_command` helper and `SAFE_COMMANDS` constant.
   The helper matches bare commands (`ls`), commands with args (`ls -la`),
   and multi-line scripts (`ls\n...`). Added `dir`, `tree`, `stat`, `file`
   to the list alongside existing safe tools.

2. **`src/safety.rs`** — applied the same refactor (note: this module is
   currently not imported by `main.rs` — there is code duplication that
   should be addressed as tech-debt).

3. **`README.md`** — already updated in prior work to list `dir`, `tree`,
   `stat`, `file` in the Safe examples.

4. **Tests** — added `test_folder_listing_commands_are_safe` in both
   `main.rs` and `safety.rs` covering bare `ls`/`dir`/`tree` and
   `stat`/`file`/`tree` with arguments.

## Problems

- `safety.rs` duplicates all classification logic from `main.rs` and is not
  actually used at runtime. This should be consolidated (tech-debt).
````
