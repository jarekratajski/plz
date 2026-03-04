# Plan: Safety Modes and Risk Categorization

## Scope
Implement command risk categorization (`safe`, `moderate`, `dangerous`) and enforce execution policy for default mode, `-s` (safe), and `-f` (force).

## Steps
1. Extend CLI parsing
- Add mutually exclusive flags: `-s` and `-f`.
- Keep existing natural-language prompt parsing unchanged.

2. Add risk model and classifier
- Introduce `enum RiskLevel { Safe, Moderate, Dangerous }`.
- Add deterministic heuristics for:
  - File/system/config modifications
  - Deletes (few/easy-to-recreate vs broader deletion)
  - Install commands (well-known vs non-standard)
  - External API/network calls
  - `sudo` usage
  - Docker lifecycle commands
- Classify generated shell command before execution.

3. Add mode policy evaluator
- Default mode:
  - `safe` => execute directly (always print command)
  - `moderate` => ask confirmation (y/n)
  - `dangerous` => reject (print command and reason)
- `-s` mode:
  - `safe` => execute directly (always print command)
  - `moderate`/`dangerous` => reject
- `-f` mode:
  - `safe`/`moderate` => execute directly (always print command)
  - `dangerous` => ask confirmation (y/n)

4. Integrate into execution flow
- Generate command from AI as today.
- Print proposed command and computed risk level.
- Apply policy outcome (`execute`, `confirm`, `reject`).
- Preserve exit code behavior and clear user-facing messages.

5. Add tests
- Unit tests for classifier heuristics.
- Unit tests for policy matrix across all modes and risk levels.
- Integration-like tests for confirmation path and rejection path.

6. Validate
- Run `cargo test` and `cargo check`.
- Ensure no warnings and no behavior regressions.

## Notes
- Keep implementation conservative: when unsure, classify toward higher risk.
- Keep code changes minimal and localized.
