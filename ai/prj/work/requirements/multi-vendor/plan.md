````markdown
# Plan: Multi-Vendor Abstraction

## Steps

1. Create `src/vendor.rs` with `CommandGenerator` trait + `select_vendor()` auto-detection
2. Create `src/vendors/mod.rs` re-exporting all vendor modules
3. Create `src/vendors/claude_cli.rs` — move from `src/claude_cli.rs`, implement trait
4. Create `src/vendors/claude_api.rs` — move from `src/claude.rs`, implement trait
5. Create `src/vendors/openai_api.rs` — new OpenAI backend, implement trait
6. Update `src/main.rs`:
   - Replace `mod claude; mod claude_cli;` with `mod vendor; mod vendors;`
   - Replace `generate_command_with_fallback()` with vendor chain logic
   - Update `--help` CONFIGURATION section
7. Delete old `src/claude.rs` and `src/claude_cli.rs`
8. Move shared `SYSTEM_PROMPT` to `vendor.rs`
9. Build, test, fix warnings
10. Update `details.md`, write `report.md`

````
