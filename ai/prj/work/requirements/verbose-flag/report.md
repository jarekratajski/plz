````markdown
# Report: Verbose Flag

## Status: Done

## Changes

1. **`src/main.rs`** — added `-v` / `--verbose` flag to `Args` struct
2. **`src/vendor.rs`** — `generate_command_with_fallback()` accepts `verbose` param;
   prints to stderr which vendor is used, skipped, or failed

## Behaviour
- `-v` prints `"Using <vendor>"` or `"<vendor> not available, skipping"` to stderr
- Without `-v`, no diagnostic output
- Works alongside `-s` and `-f` flags

## Tests
- All tests pass, 0 warnings

## Problems
- None

````
