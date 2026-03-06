````markdown
# Plan

## Approach

The `CommandGenerator` trait's `generate_command` method currently takes only
`description: &str`. To let vendors know whether to print the request body,
add a `verbose: bool` parameter to the trait method.

## Steps

1. Update `CommandGenerator::generate_command` signature to
   `fn generate_command(&self, description: &str, verbose: bool)`.
2. Update `CommandGeneratorBoxed::generate_command_boxed` signature the same
   way.
3. Update the blanket impl in `vendor.rs`.
4. Update all 4 vendor implementations (claude_cli, claude_api, openai_api,
   copilot_enterprise) to accept `verbose` and print the request body when
   true, using the format specified in the req.
5. Update the call sites in `vendor.rs` (`generate_command_forced`,
   `generate_command_with_fallback`) to pass `verbose` through.
6. Build, test, write report.
````
