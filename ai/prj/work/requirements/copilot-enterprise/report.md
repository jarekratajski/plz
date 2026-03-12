````markdown
# Report

## Status: Reverted — did not work

## Summary

The GitHub Copilot Enterprise vendor was implemented and initially reported as done,
but real-world testing showed it does not work. The vendor code has been removed.

## Changes reverted

- Deleted `src/vendors/copilot_enterprise.rs`
- Removed `copilot_enterprise` module from `src/vendors/mod.rs`
- Removed `CopilotEnterprise` from `select_vendors()` in `src/vendor.rs`
- Removed `--vendor copilot` from the value parser in `src/main.rs`
- Removed GitHub Copilot Enterprise from `--help` configuration text
- Removed Copilot row from the configuration table in `README.md`
- Removed `--vendor copilot` example from `README.md`
- Updated Options table and stateless-backends note in `README.md`

## Problems

The vendor did not work in practice. Likely cause: the GitHub Copilot Enterprise
API endpoint (`https://api.githubcopilot.com/chat/completions`) requires specific
enterprise account access that could not be verified, or the API interface differs
from what was implemented.
````
