````markdown
# Plan

## Analysis

The safety classifier already has `ls `, `dir `, `tree `, `stat `, `file `
(with trailing space) and `ls\n`, `dir\n`, `tree\n` (for multi-line scripts).
The README already lists these commands.

## Gap

Bare commands — e.g. the AI returns just `ls` with no arguments — don't match
any current pattern because `contains_any` does substring matching and none of
`"ls "`, `"ls\n"` are substrings of `"ls"`.

## Steps

1. Refactor the safe-command check in `classify_risk` to use a new helper
   `is_safe_read_command` that checks whether the command starts with a known
   safe tool name (either the full command equals it, or it's followed by a
   space or newline). This covers bare invocation, invocation with args, and
   multi-line scripts.
2. Add tests for bare `ls`, `dir`, `tree` and for `stat`/`file` with args.
3. Verify README already has the updated list (it does from earlier work).
4. Build & run tests.
5. Write report.md.
````
