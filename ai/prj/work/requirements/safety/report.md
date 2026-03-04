# Report: Safety Modes and Risk Categorization

## Status
Implemented.

## What was done
- Added CLI switches `-s` (safe mode) and `-f` (force mode) as mutually exclusive options.
- Added command risk categorization with 3 levels: `safe`, `moderate`, `dangerous`.
- Implemented policy behavior matrix:
  - default: safe=execute, moderate=confirm, dangerous=reject
  - `-s`: safe=execute, moderate/dangerous=reject
  - `-f`: safe/moderate=execute, dangerous=confirm
- Updated runtime flow to always print proposed command and show computed risk level.
- Added tests for policy matrix and classification examples.

## Validation
- `cargo check` passes.
- `cargo test` passes (8 tests, all green).

## Notes
- Classifier is heuristic-based and conservative by default (unknown patterns are treated as `moderate`).
