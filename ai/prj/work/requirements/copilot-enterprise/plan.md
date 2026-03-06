````markdown
# Plan

## Steps

1. Create `src/vendors/copilot_enterprise.rs` implementing `CommandGenerator`
   - Endpoint: `POST https://api.githubcopilot.com/chat/completions`
   - Auth: `Bearer $GITHUB_TOKEN` + `Editor-Version: plz/1.0.0`
   - OpenAI-compatible request/response format
   - Available when `GITHUB_TOKEN` is set and non-empty
2. Register module in `src/vendors/mod.rs`
3. Add to priority list in `src/vendor.rs` at position 4
4. Add `copilot` to `--vendor` value parser in `main.rs`
5. Update help text and README
6. Update fallback error message to mention `GITHUB_TOKEN`
````
