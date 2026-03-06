# plz

Turn natural language into shell commands. Describe what you want in plain
English and `plz` generates the bash command, shows it for review, and runs it
after confirmation.

```
$ plz stop all docker containers
Asking AI how to: stop all docker containers

Proposed command:
  docker stop $(docker ps -q)

Risk: Safe (docker lifecycle command on developer machine)
```

## Installation

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (`cargo` 1.70+)

### Build & install

```bash
git clone https://github.com/AiMiePlz/plz.git
cd plz
cargo install --path .
```

Verify:

```bash
plz --help
```

## Configuration

`plz` auto-detects available AI backends in priority order:

| Priority | Backend | How to enable |
|----------|---------|---------------|
| 1 | Claude CLI | Install from [docs.anthropic.com](https://docs.anthropic.com/en/docs/claude-cli). If `claude` is in your `$PATH`, it is used automatically. No API key needed. |
| 2 | Claude HTTP API | `export ANTHROPIC_API_KEY="sk-ant-..."` — get a key at [console.anthropic.com](https://console.anthropic.com/) |
| 3 | OpenAI (ChatGPT) HTTP API | `export OPENAI_API_KEY="sk-..."` — get a key at [platform.openai.com/api-keys](https://platform.openai.com/api-keys) |
| 4 | GitHub Copilot Enterprise | `export GITHUB_TOKEN="ghp_..."` — requires a GitHub token with Copilot Enterprise access |

Add the `export` to `~/.bashrc` or `~/.zshrc` for persistence.

The first available backend is used. To force a specific one:

```bash
plz --vendor claude-cli list files
plz --vendor claude list files
plz --vendor chatgpt list files
plz --vendor copilot list files
```

## Usage

```bash
plz <description of what you want>
```

### Examples

```bash
plz list the 5 largest files in this directory
plz show disk usage sorted by size
plz find all TODO comments in rust files
plz stop all docker containers
plz compress this folder into a tar.gz
```

## Options

| Flag | Description |
|------|-------------|
| `-s` | **Safe mode** — only Safe commands run; everything else is rejected |
| `-f` | **Force mode** — Safe and Moderate commands run automatically; Dangerous asks for confirmation |
| `-v`, `--verbose` | Print which AI backend is being used and other diagnostics |
| `--vendor <NAME>` | Force a specific backend: `claude-cli`, `claude`, `chatgpt`, or `copilot` |
| `-h`, `--help` | Show help with configuration instructions |

`-s` and `-f` are mutually exclusive.

## Safety system

Every generated command is classified into a **risk level** before execution.
The risk level combined with the **execution mode** determines what happens.

### Risk levels

| Level | Examples | Rationale |
|-------|----------|-----------|
| **Safe** | `ls`, `cat`, `grep`, `git status`, `docker stop` | Read-only or common local operations |
| **Moderate** | `rm file.txt`, `curl https://...`, `npm install` | Deletes files, calls external APIs, or installs packages |
| **Dangerous** | `sudo ...`, `rm -rf /`, `curl ... \| bash`, touching `/etc/` | Elevated privileges, broad destructive operations, or pipe-to-shell installs |

Unknown command patterns default to **Moderate** (conservative).

### Execution modes

| Mode | Flag | Behaviour |
|------|------|-----------|
| **Default** | *(none)* | Normal interactive use |
| **Safe** | `-s` | Lockdown — only safe commands allowed |
| **Force** | `-f` | Hands-off — auto-runs most things |

### Policy matrix

| | Safe | Moderate | Dangerous |
|---|---|---|---|
| **Default** | Auto-execute | Ask confirmation | Reject |
| **Safe** (`-s`) | Auto-execute | Reject | Reject |
| **Force** (`-f`) | Auto-execute | Auto-execute | Ask confirmation |

### Confirmation prompt

When confirmation is required, `plz` shows:

```
Execute? [y/N]:
```

- Type `y` or `yes` to proceed
- Press Enter (empty) or anything else → **No** (default-deny)

### What triggers each risk level

**Dangerous:**
- `sudo` commands
- Paths touching system directories (`/etc/`, `/usr/`, `/var/`, `/boot/`, etc.)
- User config directories (`~/.config/`)
- Broad destructive operations (`rm -rf /`, `rm -rf *`, `shred`, `mkfs`)
- Pipe-to-shell installs (`curl ... | bash`)

**Moderate:**
- File deletion (`rm`, `rmdir`, `unlink`)
- Network calls (`curl`, `wget`, `http://`, `https://`)
- Package installs (`npm install`, `pip install`, `cargo add`, `brew install`, etc.)
- Anything not recognized (conservative default)

**Safe:**
- Read/list operations (`ls`, `dir`, `tree`, `cat`, `find`, `grep`, `head`, `tail`, `wc`, `stat`, `file`)
- Common file operations (`touch`, `mkdir`, `cp`, `mv`)
- Git commands
- Docker lifecycle commands (`docker stop`, `docker run`, `docker compose up`)
