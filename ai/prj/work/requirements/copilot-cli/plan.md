````markdown
# Plan

## Steps

1. Create `src/vendors/copilot_cli.rs`
   - Availability: `which gh` succeeds AND `gh copilot --version` succeeds
   - Invocation: `gh copilot suggest -t shell "<description>"`
   - Output parsing: `gh copilot suggest` prints `Suggestion:\n\n  <command>` on stdout;
     extract the first non-empty trimmed line after the "Suggestion:" line.
     Fall back to trimming the full output if the pattern is not found.
   - No session support (stateless)

2. Register module in `src/vendors/mod.rs`

3. Insert at priority 2 in `src/vendor.rs` `select_vendors()`

4. Add `copilot-cli` to `--vendor` value parser in `src/main.rs` and help text

5. Update `README.md`

6. Write `report.md`
````
