# Report: Separate Tests

## Status
Done.

## Findings
In Rust, inline `#[cfg(test)] mod tests` blocks at the bottom of each source file **is** the standard convention for unit tests. Separating unit tests into different files is against Rust norms. Integration tests go in a top-level `tests/` directory (not applicable here yet).

## What was done
- Extracted safety/risk classification logic from `main.rs` into `src/safety.rs` — each module now has its own focused inline tests.
- `main.rs` went from ~490 lines to ~230 lines.
- Tests are now distributed by domain:
  - `safety.rs`: policy matrix + classifier tests (4 tests)
  - `main.rs`: confirmation, retry, error excerpt tests (6 tests)
  - `claude.rs`: API parsing tests (2 tests)
- Added "Test organization" section to `code.md` documenting the Rust convention as a permanent rule.

## Validation
- `cargo check` passes (no warnings).
- `cargo test` passes (12/12 tests green).
