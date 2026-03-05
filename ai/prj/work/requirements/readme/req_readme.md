````markdown
# Goal

Create a `README.md` at the project root that documents the `plz` tool for users.

## Requirements

### 1. Compilation & Installation section (first)

- Prerequisites: Rust toolchain (`rustup` / `cargo`)
- Clone, build, and install steps:
  ```
  git clone <repo>
  cd plz
  cargo install --path .
  ```
- Verify installation: `plz --help`

### 2. Configuration section

- How to set up AI backends (Claude CLI, Claude API key, OpenAI API key)
- Auto-detection priority order
- `--vendor` flag for forcing a specific backend

### 3. Usage & Options section (longer, detailed)

- Basic usage examples (`plz stop all docker containers`)
- All CLI flags: `-s`, `-f`, `-v`, `--vendor`
- **Safety system** — explain in detail:
  - Three risk levels: Safe, Moderate, Dangerous
  - Three execution modes: Default, Safe (`-s`), Force (`-f`)
  - The policy matrix — which risk levels auto-execute, prompt for
    confirmation, or get rejected in each mode
  - Examples of what falls into each risk category (docker commands = Safe,
    `rm` = Moderate, `sudo` = Dangerous, etc.)
  - Default-deny confirmation (empty enter = No)

### 4. Tone & format

- Keep it practical and scannable (tables, code blocks, short paragraphs)
- No marketing fluff
- Include a one-liner project description at the top

## Non-goals

- Contributing guide
- Changelog
- License section (can be added later)

````
