````markdown
# Goal

Folder/directory listing commands should be classified as **Safe** by the
safety system.

## Motivation

Commands like `ls`, `dir`, `tree`, `stat`, and `file` are read-only
operations that inspect directory contents or file metadata. They cannot
modify, delete, or create anything, so they pose zero risk and should
auto-execute without confirmation.

## Requirements

1. **Classify as Safe** — the following commands (and their common variants)
   must be classified as `RiskLevel::Safe`:
   - `ls` (with or without arguments)
   - `dir` (with or without arguments)
   - `tree` (with or without arguments)
   - `stat`
   - `file`

2. **Bare invocation** — `ls`, `dir`, and `tree` invoked without any
   arguments (bare command) must also match as Safe, not fall through to the
   default Moderate classification.

3. **Existing safe commands unchanged** — all previously safe commands
   (`cat`, `find`, `grep`, `head`, `tail`, `wc`, `echo`, `touch`, `mkdir`,
   `cp`, `mv`, `git`, docker lifecycle) remain Safe.

4. **Update README** — add `dir`, `tree`, `stat`, and `file` to the Safe
   examples list in the safety documentation.

## Non-goals

- Detecting dangerous flag combinations on listing commands (e.g.
  `ls` piped into `rm`) — that is handled by the downstream command
  classification.
````
