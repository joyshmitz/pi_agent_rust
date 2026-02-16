<p align="center">
  <img src="pi_agent_rust_illustration.webp" alt="Pi Agent Rust" width="600"/>
</p>

<h1 align="center">pi_agent_rust</h1>

<p align="center">
  <strong>pi_agent_rust — High-performance AI coding agent CLI written in Rust</strong>
</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> •
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#commands">Commands</a> •
  <a href="#configuration">Configuration</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/rust-2024%20edition-orange?logo=rust" alt="Rust 2024">
  <img src="https://img.shields.io/badge/license-MIT-blue" alt="License: MIT">
  <img src="https://img.shields.io/badge/unsafe-forbidden-brightgreen" alt="No Unsafe Code">
  <a href="https://github.com/Dicklesworthstone/pi_agent_rust/actions/workflows/ci.yml">
    <img src="https://github.com/Dicklesworthstone/pi_agent_rust/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI">
  </a>
  <a href="https://github.com/Dicklesworthstone/pi_agent_rust/actions/workflows/bench.yml">
    <img src="https://github.com/Dicklesworthstone/pi_agent_rust/actions/workflows/bench.yml/badge.svg?branch=main" alt="Bench">
  </a>
</p>

---

## The Problem

You want an AI coding assistant in your terminal, but existing tools are:
- **Slow to start**: Node.js/Python runtimes add 500ms+ before you can type
- **Memory hungry**: Electron apps or heavy runtimes eat gigabytes
- **Unreliable**: Streaming breaks, sessions corrupt, tools fail silently
- **Hard to extend**: Closed ecosystems or complex plugin systems

## The Solution

**pi_agent_rust** is a from-scratch Rust port of [Pi Agent](https://github.com/badlogic/pi) by [Mario Zechner](https://github.com/badlogic) (made with his blessing!). Single binary, instant startup, rock-solid streaming, and 7 battle-tested built-in tools.

Rather than a direct line-by-line translation, this port builds on two purpose-built Rust libraries:
- **[asupersync](https://github.com/Dicklesworthstone/asupersync)**: A structured concurrency async runtime with built-in HTTP, TLS, and SQLite
- **[rich_rust](https://github.com/Dicklesworthstone/rich_rust)**: A Rust port of [Rich](https://github.com/Textualize/rich) by [Will McGugan](https://github.com/willmcgugan), providing beautiful terminal output with markup syntax

```bash
# Start a session
pi "Help me refactor this function to use async/await"

# Continue a previous session
pi --continue

# Single-shot mode (no session)
pi -p "What does this error mean?" < error.log
```

## Why Pi?

| Feature | Pi (Rust) | Typical TS/Python CLI |
|---------|-----------|----------------------|
| **Startup** | <100ms | 500ms-2s |
| **Binary size** | ~15MB | 100MB+ (with runtime) |
| **Memory (idle)** | <50MB | 200MB+ |
| **Streaming** | Native SSE parser | Library-dependent |
| **Tool execution** | Process tree management | Basic subprocess |
| **Sessions** | JSONL with branching | Varies |
| **Unsafe code** | Forbidden | N/A |

---

## Foundation Libraries

### asupersync

[asupersync](https://github.com/Dicklesworthstone/asupersync) is a structured concurrency async runtime designed for applications that need predictable resource cleanup. Key features used by pi_agent_rust:

- **Capability-based context (`Cx`)**: Async functions receive an explicit context that controls what they can do (HTTP, filesystem, time). This makes testing deterministic.
- **HTTP client with TLS**: Built-in HTTP API with rustls, avoiding OpenSSL dependency hell
- **Structured cancellation**: When a parent task cancels, all child tasks cancel cleanly. No orphaned futures.

`pi_agent_rust` runs on `asupersync` end-to-end today (runtime + HTTP/TLS + cancellation). Provider streaming uses a minimal HTTP client (`src/http/client.rs`) feeding a custom SSE parser (`src/sse.rs`).

### rich_rust

[rich_rust](https://github.com/Dicklesworthstone/rich_rust) is a Rust port of Will McGugan's [Rich](https://github.com/Textualize/rich) Python library. It provides:

- **Markup syntax**: `[bold red]error[/]` renders as bold red text
- **Tables**: ASCII/Unicode table rendering with alignment and borders
- **Panels**: Boxed content with titles
- **Progress bars**: Animated progress indicators
- **Markdown**: Terminal-rendered markdown with syntax highlighting
- **Themes**: Consistent color schemes across components

The terminal UI uses rich_rust for all output formatting, providing the same visual quality as Rich-based Python tools.

---

## Quick Start

### 1. Install

```bash
# Install latest release binary
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/pi_agent_rust/main/install.sh?$(date +%s)" | bash
```

If you already have the original TypeScript `pi` installed, the installer asks
whether to make Rust Pi canonical as `pi` and automatically create `legacy-pi`
for the old command.

### 2. Configure API Key

```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

### 3. Run

```bash
# Interactive mode
pi

# With an initial message
pi "Explain this codebase structure"

# Read files as context
pi @src/main.rs "What does this do?"
```

---

## Features

### Streaming Responses

Real-time token streaming with extended thinking support:

```
pi "Write a quicksort implementation"
```

Watch the response appear token-by-token, with thinking blocks shown inline.

### 7 Built-in Tools

| Tool | Description | Example |
|------|-------------|---------|
| `read` | Read file contents, supports images | Read src/main.rs |
| `write` | Create or overwrite files | Write a new config file |
| `edit` | Surgical string replacement | Fix the typo on line 42 |
| `bash` | Execute shell commands with timeout | Run the test suite |
| `grep` | Search file contents with context | Find all TODO comments |
| `find` | Discover files by pattern | Find all *.rs files |
| `ls` | List directory contents | What's in src/? |

All tools include:
- Automatic truncation for large outputs (2000 lines / 50KB)
- Detailed metadata in responses
- Process tree cleanup for bash (no orphaned processes)

### Session Management

Sessions persist as JSONL files with full conversation history:

```bash
# Continue most recent session
pi --continue

# Open specific session
pi --session ~/.pi/agent/sessions/--home-user-project--/2024-01-15T10-30-00.jsonl

# Ephemeral (no persistence)
pi --no-session
```

Sessions support:
- Tree structure for conversation branching
- Model/thinking level change tracking
- Automatic compaction for long conversations

### Extended Thinking

Enable deep reasoning for complex problems:

```bash
pi --thinking high "Design a distributed rate limiter"
```

Thinking levels: `off`, `minimal`, `low`, `medium`, `high`, `xhigh`

### Customization (Skills & Prompt Templates)

- **Skills**: Drop `SKILL.md` under `~/.pi/agent/skills/` or `.pi/skills/` and invoke with `/skill:name`.
- **Prompt templates**: Markdown files under `~/.pi/agent/prompts/` or `.pi/prompts/`; invoke via `/<template> [args]`.
- **Packages**: Share bundles with `pi install npm:@org/pi-packages` (skills, prompts, themes, extensions).

### Autocomplete

Pi provides context-aware autocomplete in the interactive editor:

- **`@` file references**: Type `@` followed by a path fragment to attach file contents. The completion engine indexes project files (respecting `.gitignore`) via the `ignore` crate's `WalkBuilder`, capping at 5,000 entries.
- **`/` slash commands**: Built-in commands (`/help`, `/model`, `/tree`, `/clear`, `/compact`, `/exit`) and user-defined prompt templates and skills all appear as completions.
- **Fuzzy scoring**: Prefix matches rank above substring matches. Results are sorted by match quality, then by kind (commands > templates > skills > files > paths).
- **Background refresh**: A background thread re-indexes the project file tree every 30 seconds, so completions stay current without blocking the input loop.

### Three Execution Modes

Pi runs in three modes, each suited to different workflows:

| Mode | Invocation | Use Case |
|------|-----------|----------|
| **Interactive** | `pi` (default) | Full TUI with streaming, tools, session branching, autocomplete |
| **Print** | `pi -p "..."` | Single response to stdout, no TUI, scriptable |
| **RPC** | `pi --mode rpc` | Headless JSON protocol over stdin/stdout for IDE integrations |

**Interactive mode** provides the full experience: a multi-line text editor with history, scrollable conversation viewport, model selector (`Ctrl+P`), session branch navigator (`/tree`), and real-time token/cost tracking.

**Print mode** sends one message, streams the response to stdout, and exits. Useful for shell scripts and one-off queries.

**RPC mode** exposes a line-delimited JSON protocol for programmatic control. Clients send commands (`prompt`, `steer`, `follow-up`, `abort`, `get-state`, `compact`) and receive streaming events. This is how IDE extensions and custom frontends integrate with Pi. See [RPC Protocol](#rpc-protocol) for the wire format.

### Extensions

Pi runs legacy JS/TS extensions **without Node or Bun**, using an embedded
QuickJS runtime with capability-gated host connectors:

- **187 of 223 extensions pass** automated conformance tests unmodified
- **100% pass rate** for simple single-file extensions; **98.4%** for official upstream
- **Sub-100ms cold load** (P95), **sub-1ms warm load** (P99)
- Node API shims for `fs`, `path`, `os`, `crypto`, `child_process`, `url`, and more
- Capability-based security: extensions call explicit connectors (`tool/exec/http/session/ui`) with audit logging

Extensions can register tools, slash commands, event hooks, flags, providers,
and shortcuts. See [EXTENSIONS.md](EXTENSIONS.md) for the full architecture
and [docs/extension-catalog.json](docs/extension-catalog.json) for the
223-entry catalog with per-extension conformance status and perf budgets.

## Extension Validation Pipeline

This project validates extension compatibility with a two-track pipeline:

- **Vendored corpus (223)**: deterministic conformance, compatibility matrix, and scenario suites.
- **Unvendored corpus (777)**: source acquisition and onboarding prioritization.

### Why this exists

- Catch runtime/API regressions in QuickJS host shims and capability policy.
- Keep extension support measurable instead of anecdotal.
- Produce a prioritized queue for onboarding unvendored candidates into vendored conformance.

### Pipeline components

1. **Fetch unvendored source corpus**
   - Binary: `ext_unvendored_fetch_run`
   - Typical command:
     - `cargo run --bin ext_unvendored_fetch_run -- run-all --workers 8 --no-probe`
   - Purpose:
     - Clones GitHub repos and unpacks npm tarballs into `.tmp-codex-unvendored-cache/`
     - Produces machine-readable acquisition status for all unvendored candidates
   - Artifacts:
     - `tests/ext_conformance/reports/pipeline/unvendored_fetch_probe_report.json`
     - `tests/ext_conformance/reports/pipeline/unvendored_fetch_probe_events.jsonl`

2. **Run end-to-end validation orchestration**
   - Binary: `ext_full_validation`
   - Typical command:
     - `cargo run --bin ext_full_validation --`
   - Stages (in order):
     1. `refresh_onboarding_queue` (runs `ext_onboarding_queue`)
     2. `conformance_shard_0..N` (runs `ext_conformance_generated` sharded matrix)
     3. `conformance_failure_dossiers`
     4. `provider_compat_matrix`
     5. `scenario_conformance_suite`
     6. `auto_repair_full_corpus`
     7. `differential_suite` (optional, enabled via `--run-diff`; npm diff via `--run-npm-diff`)
   - Artifacts:
     - `tests/ext_conformance/reports/pipeline/full_validation_report.json`
     - `tests/ext_conformance/reports/pipeline/full_validation_report.md`
     - Plus stage-specific reports under `tests/ext_conformance/reports/**`

3. **Aggregate and triage**
   - `full_validation_report.json` combines:
     - Stage-level pass/fail (`stageSummary`, `stageResults`)
     - Corpus counts (`corpus`)
     - Vendored conformance totals (`conformance`)
     - Provider matrix totals (`providerCompat`)
     - Scenario totals (`scenario`)
     - Review queue + verdict classification (`reviewQueue`, `verdictCounts`)
   - Important interpretation rule:
     - `not_tested_unvendored` indicates unvendored candidates not yet in vendored conformance; this is inventory status, not a vendored regression.

### Recommended run environment

These runs compile many crates and can be disk-heavy. Point Cargo artifacts and temp files to a large volume:

```bash
export CARGO_TARGET_DIR="/data/projects/pi_agent_rust/.cargo-target/${USER:-agent}"
export TMPDIR="/data/projects/pi_agent_rust/.tmp/${USER:-agent}"
mkdir -p "$CARGO_TARGET_DIR" "$TMPDIR"
```

Then run:

```bash
cargo run --bin ext_unvendored_fetch_run -- run-all --workers 8 --no-probe
cargo run --bin ext_full_validation --
```

### Latest run snapshot (2026-02-14)

- `ext_unvendored_fetch_run` (777 unvendored):
  - `fetched=761`, `cached=3`, `fetchFailed=13` (764 local sources available)
- `ext_full_validation`:
  - Stage summary: `passed=8`, `failed=1`, `skipped=1`
  - Vendored conformance: `195/223` pass (`87.4%`)
  - Provider compatibility cells: `750/750` pass (`100.0%`) after self-contained extension artifact fixes.
  - Scenario suite: `25/25` pass
  - Auto-repair sweep: `220` clean pass, `2` repaired pass (`monorepo_escape`), `1` still failing (`community/nicobailon-interview-tool`, missing `form/index.html` asset).
  - Reality check: provider compatibility is fully green, but the full extension pipeline is not yet 100% green.

---

## Installation

### Curl Installer (Recommended)

```bash
# Latest release
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/pi_agent_rust/main/install.sh?$(date +%s)" | bash

# Non-interactive + auto PATH update
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/pi_agent_rust/main/install.sh?$(date +%s)" | bash -s -- --yes --easy-mode

# Pin a release tag
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/pi_agent_rust/main/install.sh?$(date +%s)" | bash -s -- --version v0.1.0

# Install from explicit artifact URL + checksum URL
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/pi_agent_rust/main/install.sh?$(date +%s)" | \
  bash -s -- \
    --artifact-url "https://github.com/Dicklesworthstone/pi_agent_rust/releases/download/v0.1.0/pi-linux-amd64.tar.xz" \
    --checksum-url "https://github.com/Dicklesworthstone/pi_agent_rust/releases/download/v0.1.0/SHA256SUMS"

# Skip completion setup (CI/non-interactive minimal install)
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/pi_agent_rust/main/install.sh?$(date +%s)" | \
  bash -s -- --yes --no-completions
```

The installer is idempotent and supports a migration path from TypeScript Pi:
- Detect existing TS `pi` command
- Prompt to install Rust Pi as canonical `pi`
- Preserve old CLI behind `legacy-pi`
- Record state for clean uninstall/restore

Notable installer flags:
- `--offline`: skip network preflight checks (install still needs network unless using local artifacts/source cache)
- `--artifact-url`: force a specific release artifact URL
- `--checksum` / `--checksum-url`: override checksum source for explicit artifacts
- `--sigstore-bundle-url`: override Sigstore bundle URL used by `cosign verify-blob`
- `--completions auto|off|bash|zsh|fish`: force shell completion install target (`off` is equivalent to `--no-completions`)
- `--no-completions`: disable completion installation
- `--no-agent-skills`: skip automatic installation of the `pi-agent-rust` skill into `~/.claude/skills/` and `~/.codex/skills/`
- `--no-verify`: skip checksum + signature verification (testing only)
- `--artifact-url` without `--version` uses a synthetic tag for release mode only; if the artifact download fails, install exits instead of attempting source fallback

By default, the installer also installs a `pi-agent-rust` skill for both Claude Code and Codex CLI:
- Claude Code: `~/.claude/skills/pi-agent-rust/SKILL.md`
- Codex CLI: `~/.codex/skills/pi-agent-rust/SKILL.md` (or `$CODEX_HOME/skills/pi-agent-rust/SKILL.md` if `CODEX_HOME` is set)

Installer regression harness (options + checksum + signature + completions):

```bash
bash tests/installer_regression.sh
```

### Distribution Compatibility Contract (Packaging/Invocation Scope)

For drop-in adoption, packaging and invocation compatibility follows this contract:

- This section covers packaging/invocation behavior only; strict functional drop-in replacement messaging is governed by the release certification gates in `docs/dropin-certification-contract.json`.

- Canonical executable name is `pi` across release assets and installer-managed installs.
- Existing TypeScript `pi` installs can be migrated in place; the prior command is preserved as `legacy-pi`.
- If you keep TypeScript `pi` as canonical (`--keep-existing-pi`), Rust Pi is installed as `pi-rust`.
- Version-pinned installs are supported via `install.sh --version vX.Y.Z` for deterministic rollouts.
- Every GitHub release ships platform binaries plus `SHA256SUMS` for integrity validation.

Representative smoke checks:

```bash
# Canonical command should exist and execute
command -v pi
pi --version
pi --help >/dev/null

# If a TS migration was performed, legacy command remains available
command -v legacy-pi && legacy-pi --version
```

### From Source

Requires Rust nightly (2024 edition features):

```bash
# Install Rust nightly
rustup install nightly
rustup default nightly

# Clone and build
git clone https://github.com/Dicklesworthstone/pi_agent_rust.git
cd pi_agent_rust
cargo build --release

# Binary is at target/release/pi
./target/release/pi --version
```

### Dependencies

Pi has minimal runtime dependencies:
- `fd`: Required for the `find` tool (install via `apt install fd-find` or `brew install fd`)
- `rg`: Optional, improves grep performance (install via `apt install ripgrep` or `brew install ripgrep`)

### Uninstall

```bash
curl -fsSL "https://raw.githubusercontent.com/Dicklesworthstone/pi_agent_rust/main/uninstall.sh" | bash
```

By default, uninstall removes installer-managed Rust binaries/aliases and
restores a migrated TypeScript `pi` if one was preserved.

---

## Commands

### Basic Usage

```bash
pi [OPTIONS] [MESSAGE]...

# Examples
pi                              # Start interactive session
pi "Hello"                      # Start with message
pi @file.rs "Explain this"      # Include file as context
pi -p "Quick question"          # Print mode (no session)
```

Interactive file references:
- Type `@relative/path` in the editor to attach a file’s contents (autocomplete inserts the `@` form).

### Options

| Option | Description |
|--------|-------------|
| `-c, --continue` | Continue most recent session |
| `-s, --session <PATH>` | Open specific session file |
| `--no-session` | Don't persist conversation |
| `-p, --print` | Single response, no interaction |
| `--model <MODEL>` | Model to use (default: claude-sonnet-4-20250514) |
| `--thinking <LEVEL>` | Thinking level: off/minimal/low/medium/high/xhigh |
| `--tools <TOOLS>` | Comma-separated tool list |
| `--api-key <KEY>` | API key (or use ANTHROPIC_API_KEY) |
| `--list-models [PATTERN]` | List available models (optional fuzzy filter) |
| `--export <PATH>` | Export session file to HTML |

### Subcommands

```bash
# Package management
pi install <source> [-l|--local]    # Install a package source and add to settings
pi remove <source> [-l|--local]     # Remove a package source from settings
pi update [source]                 # Update all (or one) non-pinned packages
pi list                            # List user + project packages from settings

# Configuration
pi config                          # Show settings paths + precedence
```

---

## Configuration

Pi reads configuration from `~/.pi/agent/settings.json`:

```json
{
  "default_provider": "anthropic",
  "default_model": "claude-sonnet-4-20250514",
  "default_thinking_level": "medium",

  "compaction": {
    "enabled": true,
    "reserve_tokens": 8192,
    "keep_recent_tokens": 20000
  },

  "retry": {
    "enabled": true,
    "max_retries": 3,
    "base_delay_ms": 1000,
    "max_delay_ms": 30000
  },

  "images": {
    "auto_resize": true,
    "block_images": false
  },

  "terminal": {
    "show_images": true,
    "clear_on_shrink": false
  },

  "shell_path": "/bin/bash",
  "shell_command_prefix": "set -e"
}
```

### Configuration Precedence

Settings are resolved in priority order (first match wins):

1. **CLI flags** (`--model`, `--thinking`, `--provider`, etc.)
2. **Environment variables** (`ANTHROPIC_API_KEY`, `PI_CONFIG_PATH`, etc.)
3. **Project settings** (`.pi/settings.json` in the working directory)
4. **Global settings** (`~/.pi/agent/settings.json`)
5. **Built-in defaults**

This means a CLI flag always overrides a `settings.json` value, and a project-level setting overrides the global one.

### Resource Resolution

Skills, prompt templates, themes, and extensions follow the same resolution order:

1. CLI-specified paths (`--skill`, `--prompt-template`, `--theme`, `-e`)
2. Project directory (`.pi/skills/`, `.pi/prompts/`, `.pi/themes/`, `.pi/extensions/`)
3. Global directory (`~/.pi/agent/skills/`, `~/.pi/agent/prompts/`, etc.)
4. Installed packages (`~/.pi/agent/packages/`)

When multiple resources share the same name, the first occurrence wins. Collisions are logged as diagnostics.

**Prompt template expansion** supports positional arguments: `$1`, `$2`, `$@` (all args), and slice syntax `${@:start}`, `${@:start:length}`. For example, a template invoked as `/review src/main.rs --strict` receives `src/main.rs` as `$1` and `--strict` as `$2`.

### Environment Variables

| Variable | Description |
|----------|-------------|
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `OPENAI_API_KEY` | OpenAI API key |
| `GOOGLE_API_KEY` | Google Gemini API key |
| `AZURE_OPENAI_API_KEY` | Azure OpenAI API key |
| `COHERE_API_KEY` | Cohere API key |
| `GROQ_API_KEY` | Groq API key (OpenAI-compatible) |
| `DEEPINFRA_API_KEY` | DeepInfra API key (OpenAI-compatible) |
| `CEREBRAS_API_KEY` | Cerebras API key (OpenAI-compatible) |
| `OPENROUTER_API_KEY` | OpenRouter API key (OpenAI-compatible) |
| `MISTRAL_API_KEY` | Mistral API key (OpenAI-compatible) |
| `MOONSHOT_API_KEY` | Moonshot/Kimi API key (OpenAI-compatible) |
| `DASHSCOPE_API_KEY` | DashScope/Qwen API key (OpenAI-compatible) |
| `DEEPSEEK_API_KEY` | DeepSeek API key (OpenAI-compatible) |
| `FIREWORKS_API_KEY` | Fireworks API key (OpenAI-compatible) |
| `TOGETHER_API_KEY` | Together API key (OpenAI-compatible) |
| `PERPLEXITY_API_KEY` | Perplexity API key (OpenAI-compatible) |
| `XAI_API_KEY` | xAI API key (OpenAI-compatible) |
| `PI_CONFIG_PATH` | Custom config file path |
| `PI_CODING_AGENT_DIR` | Override the global config directory |
| `PI_PACKAGE_DIR` | Override the packages directory |
| `PI_SESSIONS_DIR` | Custom sessions directory |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                           CLI (clap)                            │
│  • Argument parsing    • @file expansion    • Subcommands       │
└─────────────────────────────────┬───────────────────────────────┘
                                  │
┌─────────────────────────────────▼───────────────────────────────┐
│                          Agent Loop                             │
│  • Message history     • Tool iteration    • Event callbacks    │
└────────┬──────────────────────┬──────────────────────┬──────────┘
         │                      │                      │
┌────────▼────────┐  ┌─────────▼──────────┐  ┌───────▼──────────┐
│ Provider Layer  │  │  Tool Registry     │  │  Extension Mgr   │
│ • Anthropic     │  │  • read  • bash    │  │  • QuickJS RT    │
│ • OpenAI (Chat/ │  │  • write • grep    │  │  • Capability    │
│   Responses)    │  │  • edit  • find    │  │    policy        │
│ • Gemini/Cohere │  │  • ls              │  │  • Node shims    │
│ • Azure/Bedrock │  │  • ext-registered  │  │  • Event hooks   │
│ • Vertex/Copilot│  │                    │  │  • Runtime risk  │
│ • GitLab/Ext    │  │                    │  │    controller    │
└────────┬────────┘  └─────────┬──────────┘  └───────┬──────────┘
         │                     │                      │
┌────────▼─────────────────────▼──────────────────────▼──────────┐
│                     Session Persistence                         │
│  • JSONL format (v3)   • Tree structure   • Session index/cache  │
│  • Per-project dirs    • Optional SQLite backend                │
└─────────────────────────────────────────────────────────────────┘
```

Current native providers in `src/providers/` are `anthropic`, `openai`, `openai_responses`, `gemini`, `cohere`, `azure`, `bedrock`, `vertex`, `copilot`, and `gitlab`, with extension-provided `streamSimple` providers routed through the same agent loop.

### Key Design Decisions

1. **No unsafe code**: `#![forbid(unsafe_code)]` enforced project-wide
2. **Streaming-first**: Custom SSE parser, no blocking on responses
3. **Process tree management**: `sysinfo` crate ensures no orphaned processes
4. **Structured errors**: `thiserror` with specific error types per component
5. **Size-optimized release**: LTO + strip + opt-level=z for lean binaries

### asupersync Context vs TypeScript Pi (pi-mono)

This Rust port preserves Pi's user experience, but intentionally changes the runtime substrate. The original TypeScript Pi (`pi-mono`, `packages/coding-agent`) is built on Node.js + package-level abstractions. `pi_agent_rust` moves those same behaviors onto `asupersync` primitives so lifecycle guarantees are explicit in the runtime model.

| Concern | TypeScript Pi (pi-mono baseline) | pi_agent_rust + asupersync |
|---------|----------------------------------|----------------------------|
| **Runtime model** | Node event loop + Promise/AbortSignal conventions | `RuntimeBuilder` + explicit reactor and runtime handle |
| **Async ownership** | Task lifetimes coordinated by framework/library code | Structured task ownership and explicit cross-thread channels (TUI/RPC bridging) |
| **Cancellation semantics** | Primarily API- and tool-layer conventions | Runtime-aware cancellation checks + bounded timeout handling in tools |
| **I/O capability shape** | Ambient Node APIs + extension layer policies | Capability-scoped context (`AgentCx` over `asupersync::Cx`) and explicit hostcall policy |
| **HTTP streaming** | Provider/client dependent | Purpose-built asupersync HTTP/TLS client feeding custom SSE parser |
| **Deterministic test hooks** | Conventional async test setup | asupersync test/runtime hooks used widely in unit/integration tests |

Why this is useful in practice:
- **More predictable failure behavior** during aborts/timeouts because cancellation is checked in explicit loop boundaries and tool runners.
- **Cleaner resource lifetimes** because the runtime, timers, and I/O paths all share one concurrency substrate.
- **Less hidden coupling** because the main invariants live in Rust types/algorithms rather than spread across framework conventions.

### Runtime Invariants (and Why They Matter)

These are the concrete invariants we rely on in this implementation:

1. **Turn-scoped agent lifecycle**
   - The main loop emits `AgentStart`, `TurnStart`, `TurnEnd`, and `AgentEnd` in a stable order.
   - Tool recursion is bounded by `max_tool_iterations` (default `50`) to avoid unbounded self-tool loops.
   - Benefit: stable event ordering for TUI/RPC consumers and predictable termination behavior.

2. **Abort and timeout behavior is explicit**
   - Agent abort checks happen at turn boundaries and around tool execution.
   - `bash` timeout follows a clear escalation path: terminate process tree, grace period, then hard kill.
   - Benefit: fewer "hung" sessions and reduced orphan-process risk during aggressive tool use.

3. **Session writes are crash-resilient**
   - JSONL saves write to a temp file and persist atomically.
   - Session indexing uses SQLite WAL + lock file coordination for concurrent instances.
   - Benefit: better durability and resume reliability under multi-process usage.

4. **Compaction is threshold-driven and boundary-aware**
   - Trigger: estimated context tokens exceed `context_window - reserve_tokens`.
   - Cut-point logic prefers user-turn boundaries and preserves recent context budget.
   - Benefit: compaction recovers context without collapsing near-term task continuity.

5. **Capability policy is fail-closed and precedence-defined**
   - Resolution order: per-extension deny -> global deny -> per-extension allow -> default caps -> mode fallback.
   - Benefit: policy outcomes are explainable, deterministic, and auditable.

6. **Streaming parser tolerates real network chunking**
   - SSE parser handles CR/LF variants, multi-line `data:` fields, partial UTF-8 tails, and end-of-stream flush.
   - Benefit: incremental rendering remains robust across providers and network fragmentation.

### Design Principles Carried From asupersync Into Pi

The following `asupersync` principles are reflected directly in `pi_agent_rust` architecture:

- **Single async substrate**: runtime, timers, fs, and HTTP/TLS all run on one coherent foundation.
- **Explicit context threading**: `AgentCx` wraps `asupersync::Cx` at subsystem boundaries (agent/tools/session/rpc).
- **Bounded operations over best-effort cleanup**: timeout paths and compaction thresholds are parameterized and enforceable.
- **Determinism hooks for tests**: timer-driver aware sleeps and asupersync test helpers reduce nondeterministic flakiness.

Compared to the original TypeScript implementation, this shifts more correctness responsibility into the runtime and core algorithms themselves, instead of relying primarily on ecosystem conventions.

### Additional Major Deltas (Original pi-mono vs Rust Port)

This is a second comparison pass focused on high-impact architectural deltas and rationale.

| Area | Original pi-mono (`packages/coding-agent`) | `pi_agent_rust` | Why this divergence exists |
|------|---------------------------------------------|------------------|----------------------------|
| **Distribution model** | npm package (`npm install -g @mariozechner/pi-coding-agent`) | Single Rust binary (`pi`) | Remove Node runtime dependency and improve startup/deployment portability |
| **Execution surfaces** | Interactive + print + JSON mode + RPC + SDK | Interactive + print + JSON mode + RPC (SDK parity tracked in `bd-c9usa`) | Strict drop-in parity remains the target and is release-gated; JSON/RPC/SDK parity claims are conditioned on certification artifacts |
| **Default built-in tool posture** | Defaults to `read/write/edit/bash` (others available) | Seven built-ins treated as first-class (`read/write/edit/bash/grep/find/ls`) | Keep common code-navigation and shell workflows available without extra configuration |
| **Extension trust model** | Extension/package model documented as full system access | Embedded runtime with capability-gated hostcalls and policy profiles | Reduce ambient authority and make extension behavior auditable/deny-by-default |
| **Session architecture emphasis** | JSONL tree session model and branch navigation | JSONL v3 tree + explicit session index (SQLite sidecar) + optional SQLite session backend | Faster resume/lookups at scale and safer multi-instance coordination |
| **Streaming transport stack** | Node runtime networking stack | Purpose-built HTTP/TLS client + custom SSE parser on asupersync | Tighter control over chunking, parsing, and failure handling in long streams |
| **Cancellation/timeout mechanics** | Platform/event-loop cancellation conventions | Explicit abort signaling, bounded tool iterations, process-tree termination | Minimize hangs/orphans and make stop behavior deterministic under load |
| **Runtime context model** | Framework-level conventions and extension APIs | Explicit `AgentCx`/`asupersync::Cx` capability-scoped context threading | Make effect boundaries and testability first-class architectural constraints |

Practical consequence of these deltas:
- Extension/package workflows are compatible across both implementations.
- The non-negotiable goal is strict drop-in replacement for pi-mono across all use cases.
- Strict drop-in replacement language is allowed only when `docs/dropin-certification-contract.json` gates pass and `docs/dropin-certification-verdict.json` reports `overall_verdict = CERTIFIED`.
- `docs/parity-certification.json` is an informational snapshot and does not override release-gate policy for strict replacement claims.

### Algorithmic Mechanics: pi-mono Baseline vs Rust Implementation

This section compares concrete implementation mechanics for equivalent high-level behavior.

| Algorithm | pi-mono baseline mechanism | Rust implementation mechanism | Why the Rust variant exists |
|-----------|-----------------------------|-------------------------------|-----------------------------|
| **Session context rebuild after compaction** | `buildSessionContext()` emits compaction summary, then messages from `firstKeptEntryId` (pre-compaction path), then post-compaction entries | `to_messages_for_current_path()` uses the same ordering and adds a fallback if `first_kept_entry_id` is missing | Avoid silent context loss when compaction anchors are orphaned/corrupted |
| **JSONL persistence** | Incremental append (`appendFileSync`) plus full rewrite (`writeFileSync`) for migrations/rewrites | Save via temp file + atomic persist/replace | Keep on-disk session state crash-resilient during save operations |
| **Session discovery/resume** | Directory/file scan and mtime sorting of JSONL files | SQLite session index sidecar + WAL + lock file + staleness-triggered full reindex | Bound resume lookup cost and coordinate concurrent processes |
| **Compaction token accounting** | Uses assistant usage (`totalTokens` else `input+output+cacheRead+cacheWrite`) plus heuristic trailing estimates | Uses assistant usage (`total_tokens` else `input+output`) plus heuristic trailing estimates; fixed image token estimate | Keep accounting stable across providers with uneven cache-token reporting while staying conservative |
| **Cut-point + split-turn handling** | Valid cut points exclude tool results; split turns are summarized as history + turn-prefix context | Same cut-point class and split-turn strategy, implemented in Rust entry/message model | Preserve tool-call/result adjacency and turn coherence under budget pressure |
| **Bash timeout/process cleanup** | Timeout/abort kills process tree (`killProcessTree`) and returns tail-truncated output | Timeout escalation (`TERM` then grace then `KILL`) + process-tree walk + shell exit trap + tail truncation | Enforce bounded cleanup and reduce descendant-process leaks from background jobs |
| **Streaming event decoding** | Transport semantics are exposed (`sse`/`websocket`/`auto`); parser details are runtime-internal | Explicit SSE parser with BOM stripping, CR/LF normalization, UTF-8 tail buffering, and flush-on-end | Make byte-to-event behavior deterministic and provider-SDK-independent |

### Feature Superset Highlights (Beyond pi-mono Baseline)

The sections above compare mechanics. This section calls out concrete features present in this Rust port that are not part of the pi-mono baseline implementation model.

| Rust-port feature | Why it is useful/compelling |
|-------------------|-----------------------------|
| **`pi doctor` diagnostics command** (`text`/`json`/`markdown`, `--only`, `--fix`, extension compatibility checks) | Gives actionable environment + compatibility diagnostics, supports CI gating (non-zero on failures), and can auto-fix safe issues like missing dirs/permissions |
| **Capability-gated extension policy profiles** (`safe` / `balanced` / `permissive`) with per-extension overrides | Lets operators run shared extensions with explicit capability boundaries instead of ambient full-system access |
| **Secret-aware extension env filtering** (`pi.env()` blocklist for keys/tokens/secrets) | Reduces accidental credential exposure from extension code paths |
| **Runtime risk controller for extension hostcalls** (configurable, fail-closed by default) | Adds another enforcement layer beyond static policy for suspicious runtime behavior in extension call flows |
| **Extension preflight compatibility analysis** (policy-aware extension checks) | Surfaces likely incompatibilities and security-sensitive patterns before runtime execution |
| **Node/Bun-compatible extension runtime without Node/Bun dependency** (embedded QuickJS + shims) | Runs legacy extension workflows in a single native binary deployment model |
| **Extension compatibility scanner + conformance harness** | Makes extension support measurable and auditable instead of anecdotal |
| **SQLite session index sidecar** (WAL + lock + stale reindex path) | Gives fast session resume/list operations at scale without scanning every JSONL file on each query |
| **Optional SQLite session storage backend** (`sqlite-sessions` feature) | Supports deployments that want database-backed session persistence in addition to JSONL |
| **Crash-resilient session save path** (temp file + atomic persist) | Improves session-file durability during writes and reduces partial-write failure modes |
| **Unified hostcall dispatcher with typed taxonomy mapping** (`timeout` / `denied` / `io` / `invalid_request` / `internal`) | Produces consistent extension/runtime error semantics and easier client handling |
| **Structured auth diagnostics with stable machine codes** | Improves troubleshooting and operational visibility without leaking sensitive credential material |

---

## Deep Dive: Core Algorithms

### SSE Streaming Parser

The SSE (Server-Sent Events) parser is a custom implementation that handles Anthropic's streaming response format. Unlike library-based approaches, the parser operates as a state machine that processes bytes incrementally:

```
Bytes → Line Accumulator → Event Parser → Typed StreamEvent
```

**Key characteristics:**

| Property | Implementation |
|----------|----------------|
| **Buffering** | Zero-copy where possible; lines accumulated only when incomplete |
| **Event types** | 12 distinct variants: MessageStart, ContentBlockStart, ContentBlockDelta, ContentBlockStop, MessageDelta, MessageStop, Ping, Error, and thinking-specific events |
| **Error recovery** | Malformed events logged but don't crash the stream |
| **Memory** | Fixed-size rolling buffer prevents unbounded growth |

The parser handles edge cases like:
- Multi-line `data:` fields (concatenated with newlines)
- Events split across TCP packet boundaries
- The `event:` field appearing before or after `data:`
- CRLF and LF line endings interchangeably

### Truncation Algorithm

Large outputs from tools (file reads, command output, grep results) must be truncated to avoid exhausting the LLM's context window. The truncation algorithm preserves usefulness while staying within limits:

```
┌─────────────────────────────────────────┐
│           Original Content              │
│         (potentially huge)              │
└─────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────┐
│  HEAD: First N/2 lines                  │
│  ─────────────────────────              │
│  [... X lines truncated ...]            │
│  ─────────────────────────              │
│  TAIL: Last N/2 lines                   │
└─────────────────────────────────────────┘
```

**Constants:**

| Limit | Value | Rationale |
|-------|-------|-----------|
| `MAX_LINES` | 2000 | Balances context usage vs. completeness |
| `MAX_BYTES` | 50KB | Prevents binary file accidents |
| `GREP_MAX_LINE_LENGTH` | 500 chars | Truncates minified code |

The algorithm:
1. Splits content into lines
2. If line count exceeds `MAX_LINES`, takes first 1000 and last 1000
3. Inserts a marker showing how many lines were omitted
4. If byte count still exceeds `MAX_BYTES`, applies byte-level truncation
5. Returns metadata indicating truncation occurred, enabling the LLM to request specific ranges

### Process Tree Management

The `bash` tool must handle runaway processes, infinite loops, and fork bombs without leaving orphans. The implementation uses the `sysinfo` crate to walk the process tree:

```rust
// Pseudocode for process cleanup
fn kill_process_tree(root_pid: Pid) {
    let system = System::new();
    let children = find_all_descendants(root_pid, &system);

    // Kill children first (deepest first), then parent
    for child in children.iter().rev() {
        kill(child, SIGKILL);
    }
    kill(root_pid, SIGKILL);
}
```

**Timeout behavior:**

1. Command starts with configurable timeout (default 120s)
2. Output streams to a rolling buffer in real-time
3. On timeout: SIGTERM sent, 5s grace period, then SIGKILL
4. Process tree walked and all descendants killed
5. Exit code set to indicate timeout vs. normal termination

To avoid orphaned background jobs (e.g. `cmd &`), the bash script installs an `EXIT` trap that waits for any remaining child processes and then exits with the original command's status.

This prevents the common failure mode where killing a shell leaves its children running.

### Session Tree Structure

Sessions use a tree structure rather than a flat list, enabling conversation branching (useful when exploring different approaches):

```
                    ┌─────────┐
                    │ Message │ (root)
                    │   #1    │
                    └────┬────┘
                         │
                    ┌────▼────┐
                    │ Message │
                    │   #2    │
                    └────┬────┘
                         │
              ┌──────────┼──────────┐
              │                     │
         ┌────▼────┐          ┌────▼────┐
         │ Message │          │ Message │ (branch)
         │   #3    │          │   #3b   │
         └────┬────┘          └────┬────┘
              │                    │
         ┌────▼────┐          ┌────▼────┐
         │ Message │          │ Message │
         │   #4    │          │   #4b   │
         └─────────┘          └─────────┘
```

**JSONL format (v3):**

Each line is a self-contained JSON object with a `type` discriminator:

```json
{"type":"session","version":3,"cwd":"/project","created":"2024-01-15T10:30:00Z"}
{"type":"message","id":"a1b2c3d4","parent":"root","role":"user","content":[...]}
{"type":"message","id":"e5f6g7h8","parent":"a1b2c3d4","role":"assistant","content":[...]}
{"type":"model_change","id":"i9j0k1l2","parent":"e5f6g7h8","model":"claude-sonnet-4-20250514"}
```

The `parent` field creates the tree. Replaying a session walks the tree from root to the current leaf. Branching creates a new message with a different `parent` than the previous continuation.

### Provider Abstraction

The `Provider` trait abstracts over different LLM backends:

```rust
#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    fn models(&self) -> &[Model];

    async fn stream(
        &self,
        context: &Context,
        options: &StreamOptions,
    ) -> Result<impl Stream<Item = Result<StreamEvent>>>;
}
```

**Context structure:**

```rust
pub struct Context {
    pub system: Option<String>,      // System prompt
    pub messages: Vec<Message>,       // Conversation history
    pub tools: Vec<ToolDef>,          // Available tools with JSON schemas
}
```

**StreamOptions:**

```rust
pub struct StreamOptions {
    pub model: String,
    pub max_tokens: u32,
    pub temperature: Option<f32>,
    pub thinking: Option<ThinkingConfig>,  // Extended thinking settings
    pub stop_sequences: Vec<String>,
}
```

This design allows adding new providers (OpenAI, Gemini) without modifying the agent loop. Each provider translates the common types to its wire format and emits a unified `StreamEvent` stream.

### Compaction Algorithm

Long conversations eventually exceed the model's context window. Pi's compaction algorithm reclaims space by summarizing older messages while preserving recent context.

The algorithm runs automatically after each agent turn when estimated token usage exceeds `context_window - reserve_tokens`:

```
┌──────────────────────────────────────────────────────────────┐
│                     Full Conversation                         │
│  msg1 → msg2 → msg3 → ... → msgN-5 → msgN-4 → ... → msgN   │
│  ├──── older messages ─────┤ ├─── recent messages ──────────┤ │
│                                                              │
│  Step 1: Find cut point at a valid turn boundary             │
│  Step 2: LLM summarizes msgs 1..N-5 into compact paragraph  │
│  Step 3: Store Compaction entry in session JSONL             │
│  Step 4: Next agent call uses [summary] + msgs N-4..N       │
└──────────────────────────────────────────────────────────────┘
```

**Token estimation** uses a conservative `chars ÷ 4` heuristic for text and a flat 1,200 tokens per image. When an assistant message includes a `usage` field from the API, that measured value takes precedence over the heuristic.

**Cut point selection** prefers boundaries between complete user-assistant turns. If the budget forces a mid-turn cut, the algorithm includes prefix messages from the split turn so the model retains context about what was being discussed at the boundary.

**File operation tracking** extracts `read`, `write`, and `edit` tool calls from the messages being summarized. The compaction prompt includes these paths so the summary preserves awareness of which files were examined or modified:

```
<read-files>
src/main.rs
src/config.rs
</read-files>

<modified-files>
src/auth.rs
</modified-files>
```

**Configurable parameters:**

| Parameter | Default | Purpose |
|-----------|---------|---------|
| `reserve_tokens` | 8% of context window | Safety margin for response generation |
| `keep_recent_tokens` | 10% of context window | Minimum recent context preserved |

Compaction can also be triggered manually with `/compact` in interactive mode or the `compact` RPC command.

### Multi-Provider Routing & Model Registry

Pi routes model requests through a provider factory that resolves the correct backend implementation from a `(provider, model, api)` tuple.

**Resolution flow:**

```
User specifies --provider openai --model gpt-4o
               │
               ▼
  ┌──────────────────────────┐
  │  Provider Metadata Table  │  Maps "openai" → canonical ID,
  │                           │  determines API type (Completions
  │                           │  vs Responses vs custom)
  └────────────┬──────────────┘
               │
  ┌────────────▼──────────────┐
  │  URL Normalization         │  Appends /chat/completions,
  │                            │  /responses, or /chat depending
  │                            │  on detected API type
  └────────────┬──────────────┘
               │
  ┌────────────▼──────────────┐
  │  Compat Config             │  Applies per-model overrides:
  │                            │  system_role_name, max_tokens
  │                            │  field name, feature flags
  └────────────┬──────────────┘
               │
  ┌────────────▼──────────────┐
  │  Provider Instance         │  Anthropic | OpenAI | Gemini
  │                            │  Cohere | Azure | Bedrock | ...
  └───────────────────────────┘
```

**`models.json` overrides**: Users can define custom providers in `~/.pi/agent/models.json` or `.pi/models.json`. Each entry specifies a model ID, base URL, API type, and optional compat flags, letting you route to self-hosted models, proxies, or providers that Pi does not natively support.

**Compat config** handles the differences between OpenAI-compatible APIs:

| Override | Example | Purpose |
|----------|---------|---------|
| `system_role_name` | `"developer"` | o1 models use "developer" instead of "system" |
| `max_tokens_field` | `"max_completion_tokens"` | Some models require a different field name |
| `supports_tools` | `false` | Suppress tool definitions for models that reject them |
| `supports_streaming` | `false` | Fall back to non-streaming for incompatible endpoints |
| `custom_headers` | `{"X-Custom": "val"}` | Per-provider header injection |

**Fuzzy matching**: When a provider name doesn't match any known provider, Pi computes edit distance against all registered names and suggests the closest match in the error message.

### Extension Hostcall Protocol

Extensions run in an embedded QuickJS runtime (`rquickjs` crate) and communicate with Pi through a structured hostcall protocol. This is the mechanism that lets JavaScript code invoke Pi's built-in tools, make HTTP requests, and interact with the session, all without direct OS access.

**Execution model:**

```
┌─────────────────── QuickJS VM ───────────────────┐
│                                                   │
│  extension.js calls:                              │
│    pi.tool("read", {path: "src/main.rs"})         │
│      │                                            │
│      ▼                                            │
│    enqueue HostcallRequest {                      │
│      call_id: "hc-0042",                          │
│      kind: Tool { name: "read" },                 │
│      payload: { path: "src/main.rs" },            │
│    }                                              │
│      │                                            │
│      ▼                                            │
│    return Promise (resolve/reject stored in map)  │
│                                                   │
└────────────────────────┬──────────────────────────┘
                         │
    drain_hostcall_requests()
                         │
                         ▼
┌─────────────── ExtensionDispatcher ──────────────┐
│                                                   │
│  1. Check capability policy:                      │
│     read tool → requires "read" capability        │
│     → Policy says: Allow / Deny / Prompt          │
│                                                   │
│  2. If allowed → dispatch to ToolRegistry         │
│     → Execute read tool                           │
│     → Get ToolOutput                              │
│                                                   │
│  3. complete_hostcall("hc-0042", Ok(result))      │
│     → Resolves the Promise in QuickJS             │
│                                                   │
│  4. runtime.tick()                                │
│     → Drains Promise .then() chains               │
│     → Extension JS continues execution            │
│                                                   │
└───────────────────────────────────────────────────┘
```

**Capability mapping**: Each hostcall kind maps to a required capability:

| Hostcall | Required Capability | Dangerous? |
|----------|-------------------|------------|
| `pi.tool("read", ...)` | `read` | No |
| `pi.tool("write", ...)` | `write` | No |
| `pi.tool("bash", ...)` | `exec` | Yes |
| `pi.http(request)` | `http` | No |
| `pi.exec(cmd, args)` | `exec` | Yes |
| `pi.env(key)` | `env` | Yes |
| `pi.session(op, ...)` | `session` | No |
| `pi.ui(op, ...)` | `ui` | No |
| `pi.log(entry)` | `log` | No (always allowed) |

**Deduplication**: Each hostcall's parameters are canonicalized (object keys sorted, structure normalized) and SHA-256 hashed. Identical requests within a short window can be deduplicated to avoid redundant tool executions.

**Auto-repair pipeline**: When an extension fails to load or produces runtime errors, Pi's repair system can automatically fix common issues:

| Repair Mode | Behavior |
|-------------|----------|
| `Off` | No repairs |
| `Suggest` | Log suggestions, don't apply |
| `AutoSafe` (default) | Apply provably safe fixes (missing file paths, asset references) |
| `AutoStrict` | Apply aggressive heuristic fixes (pattern-based transforms) |

**Compatibility scanner**: Before loading, Pi statically analyzes extension source code for imports, `require()` calls, and forbidden patterns (`eval`, `Function()`, `process.binding`, `dlopen`). The scan produces a capability evidence ledger that informs policy decisions.

**Environment variable filtering**: Extensions calling `pi.env()` hit a blocklist that denies access to API keys, credentials, tokens, and private keys. The filter blocks exact matches (`ANTHROPIC_API_KEY`, `AWS_SECRET_ACCESS_KEY`), suffix patterns (`*_API_KEY`, `*_SECRET`, `*_TOKEN`), and prefix patterns (`AWS_SECRET_*`, `AWS_SESSION_*`). Only `PI_*` variables are unconditionally allowed.

### Interactive TUI Architecture

The interactive mode uses the **Elm Architecture** (Model-Update-View) via the `charmed_rust` library family, which is a Rust port of Go's [Bubble Tea](https://github.com/charmbracelet/bubbletea) framework.

**Component stack:**

```
┌────────────────────────────────────────────────────┐
│                 Terminal (crossterm)                │
│  Raw mode │ Alt screen │ Keyboard/Mouse events      │
└──────────────────────┬─────────────────────────────┘
                       │
┌──────────────────────▼─────────────────────────────┐
│             bubbletea Program Loop                  │
│  Init() → Update(Msg) → View() → render cycle      │
└──────────────────────┬─────────────────────────────┘
                       │
┌──────────────────────▼─────────────────────────────┐
│                  PiApp (Model)                      │
│                                                     │
│  ┌─────────────┐ ┌──────────────┐ ┌─────────────┐  │
│  │  TextArea    │ │  Viewport    │ │  Spinner     │  │
│  │  (editor)    │ │  (convo)     │ │  (status)    │  │
│  └─────────────┘ └──────────────┘ └─────────────┘  │
│                                                     │
│  ┌─────────────────────────────────────────────┐    │
│  │           Overlay Stack                      │    │
│  │  Model Selector │ Session Picker │ /tree     │    │
│  │  Settings UI    │ Theme Picker   │ Branches  │    │
│  │  Capability Prompt (extension UI)            │    │
│  └─────────────────────────────────────────────┘    │
└──────────────────────┬─────────────────────────────┘
                       │
              async channels (mpsc)
                       │
┌──────────────────────▼─────────────────────────────┐
│             Agent Async Task                        │
│  Runs on asupersync runtime                         │
│  Streams provider responses                         │
│  Executes tools                                     │
│  Sends PiMsg events back to TUI thread              │
└────────────────────────────────────────────────────┘
```

**The async/sync bridge**: The agent runs on the `asupersync` async runtime in a separate thread. It communicates with the bubbletea UI thread through `mpsc` channels. Each streaming event (text delta, tool start, tool update, agent done) becomes a `PiMsg` variant delivered to `PiApp::update()`, keeping the UI responsive during API streaming and tool execution.

**Viewport scrolling**: The conversation viewport tracks whether the user is at the bottom. When new content arrives and the user hasn't scrolled up, the viewport auto-follows the stream tail. Scrolling up disables auto-follow; pressing `End` or typing a new message re-enables it.

**Overlay system**: Modal UIs (model selector, session picker, branch navigator, extension capability prompts) stack on top of the main conversation view. Each overlay captures keyboard input until dismissed. Only the topmost active overlay receives events.

**Slash commands** available in the interactive editor:

| Command | Action |
|---------|--------|
| `/help` | Show available commands and keybindings |
| `/model` or `Ctrl+P` | Open model selector with fuzzy search |
| `/tree` | Browse and fork the conversation tree |
| `/clear` | Clear conversation and start fresh |
| `/compact` | Trigger manual compaction |
| `/thinking <level>` | Change thinking level mid-conversation |
| `/share` | Export session to GitHub Gist |
| `/exit` or `Ctrl+C` | Exit Pi |

### RPC Protocol

The RPC mode (`pi --mode rpc`) exposes a line-delimited JSON protocol over stdin/stdout for programmatic integration. Each line is a self-contained JSON object.

**Client → Pi (stdin):**

```json
{"type": "prompt", "message": "Explain this function", "id": "req-001"}
{"type": "steer", "message": "Focus on error handling"}
{"type": "follow-up", "message": "Now add tests"}
{"type": "abort"}
{"type": "get-state"}
{"type": "compact", "reserve": 8192, "keepRecent": 20000}
```

**Pi → Client (stdout):**

```json
{"type": "event", "event": "agent_start", "data": {"session_id": "..."}}
{"type": "event", "event": "text_delta", "data": {"text": "The function"}}
{"type": "event", "event": "tool_start", "data": {"tool": "read", "input": {}}}
{"type": "event", "event": "tool_end", "data": {"tool": "read", "output": {}}}
{"type": "event", "event": "agent_done", "data": {"usage": {}}}
{"type": "response", "id": "req-001", "data": {"status": "ok"}}
```

**I/O architecture**: Two dedicated threads handle stdin reading and stdout writing, bridged to the async agent runtime via channels. The stdin thread retries on transient errors to prevent dropped input. The stdout thread flushes after every line to prevent buffering delays.

**Message queuing**: While the agent is streaming a response, incoming messages are routed to one of two queues:

| Queue | Behavior | Use Case |
|-------|----------|----------|
| **Steering** | Interrupts current response; processed on next turn | Course corrections |
| **Follow-up** | Queued until current response completes | Sequential instructions |

Queue modes (`All` or `OneAtATime`) control whether multiple queued messages are batched into a single turn or processed individually.

**Extension UI over RPC**: When an extension requests user input (capability prompt, selection dialog), Pi emits an `extensionUiRequest` event. The client renders the prompt in its own UI and responds with an `extensionUiResponse` message. IDE extensions can then present native UI for capability decisions instead of falling back to terminal prompts.

### Session Indexing

Session resume (`pi -c` or `pi -r`) needs to find the most recent session for the current project without scanning every JSONL file on disk. Pi maintains a SQLite index (`session-index.sqlite`) that provides constant-time lookups.

**Schema:**

```sql
CREATE TABLE sessions (
    path            TEXT PRIMARY KEY,
    id              TEXT NOT NULL,
    cwd             TEXT NOT NULL,
    timestamp       TEXT NOT NULL,
    message_count   INTEGER NOT NULL,
    last_modified   INTEGER NOT NULL,
    size_bytes      INTEGER NOT NULL,
    name            TEXT
);
```

**Update lifecycle:**

1. After saving a session JSONL file, Pi upserts its metadata into the index
2. `pi -c` queries `WHERE cwd = ? ORDER BY last_modified DESC LIMIT 1`
3. `pi -r` queries the same table and presents a picker sorted by recency

**Concurrency**: A file-based lock (`session-index.lock`) serializes writes from concurrent Pi instances. Reads use WAL mode for non-blocking access.

**Staleness-based reindexing**: If the index is older than a configurable threshold, Pi runs a full re-scan of the sessions directory to catch files created by other instances or manual edits. The re-scan keeps the index accurate without a centralized daemon.

### Authentication & Credential Management

Beyond simple API keys, Pi supports OAuth, AWS credential chains, and service key exchange. Credentials are stored in `~/.pi/agent/auth.json` with file-locked access to prevent corruption from concurrent instances.

| Mechanism | Providers | Details |
|-----------|-----------|---------|
| **API Key** | Anthropic, OpenAI, Gemini, Cohere, 12+ others | Static key via env var or settings |
| **OAuth** | Anthropic (console), GitHub Copilot, GitLab | Browser-based flow with automatic token refresh |
| **AWS Credentials** | Bedrock | Access key + secret + optional session token; region-aware |
| **Service Key** | SAP AI Core | Client ID/secret exchange for bearer token |
| **Bearer Token** | Custom providers | Static token in auth storage |

**OAuth token lifecycle:**

1. User runs `pi` with an OAuth-configured provider
2. Pi checks `auth.json` for an existing token
3. If missing: opens browser to authorization URL, user authenticates, Pi receives authorization code, exchanges it for access + refresh tokens, stores both with expiry timestamp
4. If expired but refresh token valid: exchanges refresh token for new access token, updates `auth.json`
5. Bearer token attached to API requests

**Credential status reporting**: `pi config` shows the status of each configured provider's credentials: `Missing`, `ApiKey`, `OAuthValid` (with time until expiry), `OAuthExpired` (with time since expiry), `AwsCredentials`, or `BearerToken`.

**Diagnostic codes**: Auth failures produce specific diagnostic codes (`MissingApiKey`, `InvalidApiKey`, `QuotaExceeded`, `OAuthTokenRefreshFailed`, `MissingAzureDeployment`, `MissingRegion`, etc.) with context-specific error hints rather than generic messages.

---

## Tool Details

### read

Read file contents (optionally images):

```
Input: { "path": "src/main.rs", "offset": 10, "limit": 50 }
```

- Supports images (jpg, png, gif, webp) with optional auto-resize
- Truncates at 2000 lines or 50KB
- Returns continuation hint if truncated

### bash

Execute shell commands with timeout and output capture:

```
Input: { "command": "cargo test", "timeout": 120 }
```

- Default 120s timeout, configurable per-call
- Set `timeout: 0` to disable the default timeout
- Process tree cleanup on timeout (kills children)
- Rolling buffer for real-time output
- Full output saved to temp file if truncated

### edit

Surgical string replacement:

```
Input: { "path": "src/lib.rs", "old": "fn foo()", "new": "fn bar()" }
```

- Exact string matching (no regex)
- Fails if old string not found or ambiguous
- Returns diff preview

### grep

Search file contents:

```
Input: { "pattern": "TODO", "path": "src/", "context": 2, "limit": 100 }
```

- Regex patterns supported
- Context lines before/after matches
- Respects .gitignore

### find

Discover files by pattern:

```
Input: { "pattern": "*.rs", "path": "src/", "limit": 1000 }
```

- Glob patterns via `fd`
- Sorted by modification time
- Respects .gitignore

### ls

List directory contents:

```
Input: { "path": "src/", "limit": 500 }
```

- Alphabetically sorted
- Directories marked with trailing `/`
- Truncates at limit

---

## Performance Engineering

### Why Rust Matters for CLI Tools

CLI tools have different performance requirements than servers or GUI applications. The critical metric is **time-to-first-interaction**: how quickly can the user start typing after invoking the command?

| Phase | TypeScript/Node.js | Rust |
|-------|-------------------|------|
| Process spawn | ~10ms | ~10ms |
| Runtime initialization | 200-500ms | 0ms (no runtime) |
| Module loading | 100-300ms | 0ms (static linking) |
| JIT warmup | 50-100ms | 0ms (AOT compiled) |
| **Total** | **360-910ms** | **~10ms** |

This difference compounds with usage frequency. A developer invoking `pi` 50 times per day saves 15-45 minutes per week in startup latency alone.

### Binary Size Optimization

The release profile aggressively optimizes for size:

```toml
[profile.release]
opt-level = "z"      # Size optimization (not speed)
lto = true           # Link-time optimization across all crates
codegen-units = 1    # Single codegen unit (slower compile, better optimization)
panic = "abort"      # No unwinding machinery
strip = true         # Remove symbol tables
```

**Size breakdown (approximate):**

| Component | Contribution |
|-----------|-------------|
| Core binary logic | ~3MB |
| HTTP + TLS (rustls) | ~5MB |
| serde + JSON | ~1MB |
| clap (CLI) | ~1MB |
| Other dependencies | ~3MB |
| **Total (stripped)** | **~13-15MB** |

Compare to Node.js: the `node` binary alone is 80MB+, before any application code.

### Benchmark Evidence vs Shipping Artifacts

Pi separates performance evidence artifacts from distributable release artifacts:

- **Benchmark evidence artifacts** (PERF-3X and certification inputs) are produced by
  `scripts/perf/orchestrate.sh` and `scripts/bench_extension_workloads.sh`, and must carry
  run provenance (`correlation_id`) plus profile labels (for example `build_profile=perf`).
- **Shipping artifacts** (end-user binaries) are built with Cargo `release` profile and
  distributed via GitHub Releases + installer paths.

Policy implication: release/size artifacts alone are not valid evidence for global performance
claims. Performance claims must cite benchmark evidence bundles with reproducible provenance.
See `docs/testing-policy.md` and `docs/releasing.md` for normative policy details.

### Allocator Strategy for Benchmarks

Shipping builds default to the platform allocator. For allocator experiments, Pi
supports `jemalloc` as an explicit build-time option:

```bash
# System allocator baseline + jemalloc variant in one repeatable run
BENCH_ALLOCATORS_CSV=system,jemalloc \
  ./scripts/bench_extension_workloads.sh
```

The benchmark harness records both requested and effective allocator metadata in
its JSONL output (`allocator_requested`, `allocator_effective`,
`allocator_fallback_reason`) via `PI_BENCH_ALLOCATOR`.

- `system`: build and run without allocator feature overrides
- `jemalloc`: build with `--features jemalloc`
- `auto`: prefer `jemalloc`, fall back to `system` if build fails

If `jemalloc` is requested but unavailable for the current build, the run fails
closed to the compiled allocator and includes a fallback reason in artifacts.

### Memory Usage

Rust's ownership model enables predictable memory usage without garbage collection pauses:

| State | Memory |
|-------|--------|
| Startup (idle) | ~15MB |
| Active session (small) | ~25MB |
| Large file in context | ~30-50MB |
| Streaming response | +0MB (streamed, not buffered) |

The absence of a GC means no surprise latency spikes during streaming output.

### Streaming Architecture

Responses stream token-by-token from the API to the terminal with minimal buffering:

```
API Server → TCP → SSE Parser → Event Handler → Terminal
     │                              │
     └──────── no buffering ────────┘
```

Each token appears on screen within milliseconds of leaving Anthropic's servers. The SSE parser processes events as they arrive rather than waiting for complete responses.

### TUI Rendering Performance

The interactive TUI targets 60fps rendering with several optimization layers:

**Frame timing telemetry**: Every render cycle is instrumented. Slow frames (>16ms) are tracked and classifiable by phase: viewport sync, message encoding, markdown rendering. This data feeds the internal performance monitor.

**Message render cache**: Markdown-to-ANSI conversion involves syntax highlighting, table layout, and link detection. Pi caches the rendered output per message and invalidates only on theme change or terminal resize. During streaming, only the actively-changing message is re-rendered; all prior messages hit the cache.

**Pre-allocated render buffers**: The `RenderBuffers` struct is reused across render cycles. Rather than allocating new `String` buffers each frame, Pi writes into pre-sized buffers and clears them before reuse, eliminating thousands of small allocations per second during streaming.

**Memory pressure monitoring**: A `MemoryMonitor` samples process heap size and classifies it into three tiers:

| Tier | Threshold | Action |
|------|-----------|--------|
| **Normal** | <80% of budget | No action |
| **Pressure** | 80-95% of budget | Collapse tool output displays, hide thinking blocks |
| **Critical** | >95% of budget | Truncate older messages, force compaction |

Progressive degradation keeps Pi responsive during long sessions with accumulated tool output.

---

## Troubleshooting

For a more complete guide, see [docs/troubleshooting.md](docs/troubleshooting.md).

### "fd not found"

The `find` tool requires `fd`:

```bash
# Ubuntu/Debian
apt install fd-find

# macOS
brew install fd

# The binary might be named fdfind
ln -s $(which fdfind) ~/.local/bin/fd
```

### "API key not set"

```bash
export ANTHROPIC_API_KEY="sk-ant-..."

# Or in settings.json
{ "apiKey": "sk-ant-..." }

# Or per-command
pi --api-key "sk-ant-..." "Hello"
```

### "Session corrupted"

Sessions are append-only JSONL. If corruption occurs:

```bash
# Start fresh
pi --no-session

# Or delete the problematic session
rm ~/.pi/agent/sessions/--home-user-project--/corrupted-session.jsonl
```

### "Streaming hangs"

Check your network connection. Pi uses SSE which requires stable connections:

```bash
# Test with curl
curl -N https://api.anthropic.com/v1/messages
```

### "Tool output truncated"

This is intentional to prevent context overflow. Use offset/limit:

```bash
# In the conversation
"Read lines 2000-4000 of that file"
```

---

## Limitations

Pi is honest about what it doesn't do:

| Limitation | Workaround |
|------------|------------|
| **Not all provider APIs** | Built-in support includes Anthropic, OpenAI (Chat + Responses), Gemini, Cohere, Azure OpenAI, Bedrock, Vertex AI, GitHub Copilot, and GitLab Duo; some ecosystem-specific APIs are still TBD |
| **No web browsing** | Use bash with curl |
| **No GUI** | Terminal-only by design |
| **Some extensions need npm stubs** | 5 npm packages not yet shimmed; see EXTENSIONS.md §8.1 |
| **English-centric** | Works but not optimized for other languages |
| **Nightly Rust required** | Uses 2024 edition features |

---

## Design Philosophy

### Specification-First Porting

This port follows a "specification extraction" methodology rather than line-by-line translation:

1. **Extract behavior**: Study the TypeScript implementation to understand *what* it does, not *how*
2. **Document the spec**: Write down expected behaviors, edge cases, and invariants
3. **Implement from spec**: Write idiomatic Rust that satisfies the spec
4. **Conformance testing**: Verify behavior matches via fixture-based tests

This approach yields better code than mechanical translation. TypeScript idioms (callbacks, promises, class hierarchies) don't map cleanly to Rust (ownership, traits, enums). Fighting the language produces worse results than embracing it.

### Conformance Testing

The test suite includes fixture-based conformance tests that validate tool behavior:

```json
{
  "version": "1.0",
  "tool": "edit",
  "cases": [
    {
      "name": "edit_simple_replace",
      "setup": [
        {"type": "create_file", "path": "test.txt", "content": "Hello, World!"}
      ],
      "input": {
        "path": "test.txt",
        "oldText": "World",
        "newText": "Rust"
      },
      "expected": {
        "content_contains": ["Successfully replaced"],
        "details": {"oldLength": 5, "newLength": 4}
      }
    }
  ]
}
```

Each fixture specifies:
- **Setup**: Files/directories to create before the test
- **Input**: Tool parameters
- **Expected**: Output content patterns, exact field matches, or error conditions

The Rust implementation can be validated against the TypeScript original without coupling to implementation details.

### Extension System

Pi supports legacy JS/TS extensions via an embedded QuickJS runtime. Unlike
traditional plugin systems, extensions run in a **sandboxed, capability-gated**
environment with no ambient OS access:

1. **No Node/Bun required**: QuickJS + Pi-provided shims for common Node APIs
2. **Capability-based security**: each host connector call is policy-checked and logged
3. **Conformance-tested**: 187 of 223 known extensions pass automated tests
4. **Sub-100ms load times**: extensions load in <100ms (P95) with no JIT warmup

Policy preset quick-start:

```bash
# Inspect current effective policy
pi --explain-extension-policy

# Switch profile for one command (safe | balanced | permissive)
pi --extension-policy balanced --explain-extension-policy

# Legacy alias is still accepted:
pi --extension-policy standard --explain-extension-policy

# Narrow dangerous-capability opt-in (preferred over permissive)
PI_EXTENSION_ALLOW_DANGEROUS=1 pi --extension-policy balanced --explain-extension-policy
```

Operator rollout playbook (safe local + CI adoption):

```bash
# 1) Baseline: verify defaults are fail-closed (`safe`)
pi --explain-extension-policy

# 2) Staging: use balanced prompting, dangerous caps still denied by default
pi --extension-policy balanced --explain-extension-policy

# 3) Narrow opt-in for dangerous capabilities (preferred path)
PI_EXTENSION_ALLOW_DANGEROUS=1 pi --extension-policy balanced --explain-extension-policy

# 4) Emergency troubleshooting only (short-lived)
pi --extension-policy permissive --explain-extension-policy
```

`settings.json` baseline for local/dev:

```json
{
  "extensionPolicy": {
    "profile": "balanced",
    "allowDangerous": false
  }
}
```

CI guidance:

```bash
# CI default: keep dangerous capabilities disabled
pi --extension-policy safe --explain-extension-policy

# CI opt-in job (only where required), keep explicit and auditable
PI_EXTENSION_ALLOW_DANGEROUS=1 pi --extension-policy balanced --explain-extension-policy
```

Rollback rule: remove `PI_EXTENSION_ALLOW_DANGEROUS`, set `extensionPolicy.profile`
back to `safe`, and re-run `pi --explain-extension-policy` to confirm deny decisions.

See [EXTENSIONS.md](EXTENSIONS.md) for the full architecture, runtime contract,
and conformance results.

### Unsafe Forbidden

The `#![forbid(unsafe_code)]` directive is project-wide and non-negotiable. Rationale:

- **Attack surface**: Pi executes user-provided shell commands and reads arbitrary files
- **Memory bugs = security bugs**: Buffer overflows or use-after-free in this context could be exploitable
- **Performance irrelevant**: The bottleneck is network latency to the API, not CPU cycles
- **Dependencies audited**: All dependencies either use no unsafe or are well-audited (e.g., `rustls`)

The safe Rust subset provides all necessary functionality without compromising security.

---

## FAQ

**Q: What's the relationship to the original Pi Agent?**
A: This is an authorized Rust port of [Pi Agent](https://github.com/badlogic/pi) by [Mario Zechner](https://github.com/badlogic), created with his blessing. The architecture differs significantly from the TypeScript original: it uses [asupersync](https://github.com/Dicklesworthstone/asupersync) for structured concurrency and [rich_rust](https://github.com/Dicklesworthstone/rich_rust) (a port of Will McGugan's [Rich](https://github.com/Textualize/rich) library) for terminal rendering. The goal is idiomatic Rust while preserving Pi Agent's UX.

**Q: Why rewrite in Rust?**
A: Startup time matters when you're in a terminal all day. Rust gives us <100ms startup vs 500ms+ for Node.js. Plus, no runtime dependencies to manage.

**Q: Can I use providers beyond Anthropic (OpenAI/Gemini/Cohere/Azure/Bedrock/Vertex/Copilot/GitLab)?**
A: Yes. Native providers include OpenAI (Chat + Responses), Gemini, Cohere, Azure OpenAI, Amazon Bedrock, Vertex AI, GitHub Copilot, and GitLab Duo. Set the provider-specific credentials (for example `OPENAI_API_KEY`, `GOOGLE_API_KEY`, `COHERE_API_KEY`, `AZURE_OPENAI_API_KEY`, AWS credentials for Bedrock, `GITHUB_COPILOT_API_KEY`/`GITHUB_TOKEN`, or `GITLAB_TOKEN`/`GITLAB_API_KEY`) and select via `--provider`/`--model`.

**Q: How do sessions work?**
A: Each session is a JSONL file with message entries. Sessions are per-project (based on working directory) and support branching via parent references.

**Q: Why is unsafe forbidden?**
A: Memory safety is non-negotiable for a tool that executes arbitrary commands. The performance cost is negligible for this use case.

**Q: How do I extend Pi?**
A: Pi has a full extension system. Drop a `.ts` or `.js` extension file into your project and it runs in an embedded QuickJS runtime with capability-gated host access. Extensions can register tools, slash commands, event hooks, flags, and custom providers. See [EXTENSIONS.md](EXTENSIONS.md) for details. For built-in tool changes, implement the `Tool` trait in `src/tools.rs`.

**Q: Why isn't X feature included?**
A: Pi focuses on core coding assistance. Features like web browsing, image generation, etc. are out of scope. Use specialized tools for those.

**Q: How does compaction work?**
A: When a conversation exceeds the model's context window, Pi summarizes older messages using the LLM itself, storing the summary as a session entry. Recent messages are kept verbatim. The cut point is chosen at a turn boundary, and the summary includes a record of which files were read or modified so the model retains that awareness. Compaction runs automatically after each agent turn when needed, or manually via `/compact`.

**Q: Can I add a custom provider that Pi doesn't support natively?**
A: Yes. Create a `models.json` file in `~/.pi/agent/` or `.pi/` with entries specifying the model ID, base URL, and API type (usually `openai-completions` for OpenAI-compatible endpoints). Pi's compat config system handles field name differences and feature flag overrides. Extensions can also register entirely custom providers.

**Q: How does Pi decide which session to resume?**
A: Pi maintains a SQLite index of all session files. When you run `pi -c`, it queries the index for the most recently modified session whose working directory matches your current project. This avoids scanning the filesystem on every resume.

**Q: What happens if an extension tries to access something dangerous?**
A: Every hostcall from an extension is checked against the active capability policy before execution. Dangerous capabilities (`exec`, `env`) are denied by default under the `safe` policy, require a user prompt under `balanced`, and are allowed under `permissive`. Denied calls return an error to the extension's Promise. Pi also blocks extensions from reading sensitive environment variables (API keys, credentials, tokens) regardless of policy.

**Q: Does Pi work with self-hosted or proxied LLMs?**
A: Yes. Point any provider at a custom base URL via `models.json` or `--base-url`. Pi normalizes the URL path per API type and applies compat config overrides for any field name or feature differences. This works with vLLM, Ollama, LiteLLM, and similar OpenAI-compatible servers.

---

## Comparison

| Feature | Pi | Claude Code | Aider | Cursor |
|---------|-----|-------------|-------|--------|
| **Language** | Rust | TypeScript | Python | Electron |
| **Startup** | <100ms | ~1s | ~2s | ~5s |
| **Memory** | <50MB | ~200MB | ~150MB | ~500MB |
| **Providers** | Anthropic + OpenAI/Responses + Gemini/Cohere + Azure/Bedrock/Vertex + Copilot/GitLab | Anthropic | Many | Many |
| **Tools** | 7 built-in | Many | File-focused | IDE-integrated |
| **Sessions** | JSONL tree | Proprietary | Git-based | Proprietary |
| **Open source** | Yes | Yes | Yes | No |

---

## Development

### Building

```bash
cargo build           # Debug build
cargo build --release # Release build (optimized)
cargo test           # Run tests
cargo clippy         # Lint check
```

### Testing

```bash
# Unified verification runner (recommended for deterministic evidence artifacts)
./scripts/e2e/run_all.sh --profile focused
./scripts/e2e/run_all.sh --profile ci
./scripts/e2e/run_all.sh --rerun-from tests/e2e_results/<timestamp>/summary.json --skip-unit

# Multi-agent safety: with CODEX_THREAD_ID set, run_all defaults
# CARGO_TARGET_DIR to target/agents/<CODEX_THREAD_ID> unless overridden.
# Set CARGO_TARGET_DIR explicitly if you want a custom shared or isolated target.

# All tests
cargo test

# Specific module
cargo test tools::tests
cargo test sse::tests

# Conformance tests
cargo test conformance
```

### Release & Publishing

Releases are tag-driven and must align with `Cargo.toml` versions.

- Tag format: `vX.Y.Z` (pre-releases like `vX.Y.Z-rc.N` are allowed but skip crates.io publish).
- The tag version **must** match `package.version` in `Cargo.toml`.
- Publish order for dependencies: `asupersync` → `rich_rust` → `charmed-*` (lipgloss, bubbletea, bubbles, glamour) → `pi_agent_rust`.
- `.github/workflows/publish.yml` handles crates.io publish when `CARGO_REGISTRY_TOKEN` is set.

### Coverage

Coverage uses `cargo-llvm-cov`:

```bash
# One-time install
cargo install cargo-llvm-cov --locked
rustup component add llvm-tools-preview

# Summary (fastest)
cargo llvm-cov --all-targets --workspace --summary-only

# LCOV report (for CI/artifacts)
CI=true VCR_MODE=playback VCR_CASSETTE_DIR=tests/fixtures/vcr \
  cargo llvm-cov --all-targets --workspace --lcov --output-path lcov.info

# HTML report (defaults to target/llvm-cov/html)
cargo llvm-cov --all-targets --workspace --html
```

### Project Structure

Selected core modules (non-exhaustive):

```
src/
├── main.rs                # CLI entry point
├── lib.rs                 # Library exports
├── app.rs                 # Startup/model selection helpers
├── agent.rs               # Agent loop + event orchestration
├── agent_cx.rs            # asupersync capability context wiring
├── cli.rs                 # Argument parsing
├── config.rs              # Configuration
├── auth.rs                # API key/OAuth/AWS credential storage
├── model.rs               # Message/content/stream event types
├── provider.rs            # Provider trait
├── provider_metadata.rs   # Canonical provider IDs + routing defaults
├── models.rs              # Model registry + models.json overrides
├── providers/
│   ├── anthropic.rs        # Anthropic Messages API
│   ├── openai.rs           # OpenAI Chat Completions
│   ├── openai_responses.rs # OpenAI Responses API
│   ├── gemini.rs           # Gemini API
│   ├── cohere.rs           # Cohere Chat API
│   ├── azure.rs            # Azure OpenAI
│   ├── bedrock.rs          # Amazon Bedrock Converse
│   ├── vertex.rs           # Google Vertex AI
│   ├── copilot.rs          # GitHub Copilot backend
│   ├── gitlab.rs           # GitLab Duo backend
│   └── mod.rs              # Provider factory + extension bridge
├── tools.rs                # Built-in tool implementations
├── sse.rs                  # Streaming SSE parser
├── http/
│   ├── client.rs           # asupersync-backed HTTP client
│   ├── sse.rs              # HTTP SSE helpers
│   └── mod.rs
├── session.rs              # JSONL session persistence/tree ops
├── session_index.rs        # SQLite session metadata index/cache
├── session_sqlite.rs       # Optional sqlite-sessions backend
├── compaction.rs           # Context compaction algorithm
├── interactive.rs          # Interactive TUI app loop/state
├── interactive/            # Bubble Tea-style TUI submodules
├── rpc.rs                  # RPC/stdio mode
├── extensions.rs           # Extension protocol + policy + security
├── extensions_js.rs        # QuickJS runtime bridge + hostcalls
├── extension_dispatcher.rs # Hostcall/tool dispatch plumbing
├── extension_preflight.rs  # Extension compatibility scanner
├── extension_validation.rs # Extension validation pipeline glue
├── resources.rs            # Skills/prompt/theme/extension loading
└── tui.rs                  # Terminal UI rendering helpers
```

---

## Documentation Index

Each entry below includes the document name, purpose, bottom-line takeaway, and direct link.

| Category | What it covers | Jump |
|---|---|---|
| Extension Ecosystem | extension architecture, corpus, catalogs, conformance, compatibility | [Go](#1-extension-ecosystem) |
| Core Ops and UX | governance, CI/QA ops, keybindings, prompts, packaging, model/config UX, drop-in migration/certification | [Go](#2-core-ops-and-ux) |
| Provider Subsystem | provider audits, setup contracts, parity and remediation artifacts | [Go](#3-provider-subsystem) |
| QA, Schemas, Security, Platform | release/QA runbooks, schemas, security baselines, platform guides | [Go](#4-qa-schemas-security-and-platform) |

### 1. Extension Ecosystem

- `docs/BRANCH_PROTECTION.md` - Purpose: define branch protection policy and enforcement rules. Bottom line: CI and review gates are mandatory for mainline integrity. Link: [View](docs/BRANCH_PROTECTION.md)
- `docs/EXTENSION_CANDIDATES.md` - Purpose: catalog candidate extensions for inclusion/testing. Bottom line: start extension triage from this longlist. Link: [View](docs/EXTENSION_CANDIDATES.md)
- `docs/EXTENSION_CAPTURE_SCENARIOS.md` - Purpose: define extension capture and replay scenarios. Bottom line: use these scenarios for deterministic extension behavior capture. Link: [View](docs/EXTENSION_CAPTURE_SCENARIOS.md)
- `docs/EXTENSION_POPULARITY_CRITERIA.md` - Purpose: specify how extension popularity is scored. Bottom line: popularity decisions follow explicit ranking criteria. Link: [View](docs/EXTENSION_POPULARITY_CRITERIA.md)
- `docs/EXTENSION_REFRESH_CHECKLIST.md` - Purpose: operational checklist for extension corpus refreshes. Bottom line: follow this to update corpus safely and repeatably. Link: [View](docs/EXTENSION_REFRESH_CHECKLIST.md)
- `docs/EXTENSION_SAMPLE.md` - Purpose: human-readable sample extension contract. Bottom line: reference this when authoring new extension packages. Link: [View](docs/EXTENSION_SAMPLE.md)
- `docs/EXTENSION_SAMPLING_MATRIX.md` - Purpose: define extension sampling strategy and strata. Bottom line: test coverage comes from this sampling matrix. Link: [View](docs/EXTENSION_SAMPLING_MATRIX.md)
- `docs/LEGACY_EXTENSION_RUNNER.md` - Purpose: explain legacy extension runner behavior and constraints. Bottom line: use for compatibility context, not new runtime design. Link: [View](docs/LEGACY_EXTENSION_RUNNER.md)
- `docs/PIJS_PROOF_REPORT.md` - Purpose: provide formal evidence for PiJS runtime properties. Bottom line: PiJS eliminates ambient authority by design. Link: [View](docs/PIJS_PROOF_REPORT.md)
- `docs/TEST_COVERAGE_MATRIX.md` - Purpose: map modules to test suites and coverage status. Bottom line: this is the coverage gap dashboard. Link: [View](docs/TEST_COVERAGE_MATRIX.md)
- `docs/capability-prompts.md` - Purpose: document capability prompt UX and policy semantics. Bottom line: prompts are policy artifacts, not ad hoc UI. Link: [View](docs/capability-prompts.md)
- `docs/ci-operator-runbook.md` - Purpose: CI operations runbook for maintainers. Bottom line: use this for incident response in CI pipelines. Link: [View](docs/ci-operator-runbook.md)
- `docs/conformance-operator-playbook.md` - Purpose: operating guide for conformance campaigns. Bottom line: run conformance with this playbook for reproducible results. Link: [View](docs/conformance-operator-playbook.md)
- `docs/coverage-baseline-map.json` - Purpose: machine-readable mapping of coverage baselines. Bottom line: automation should use this as baseline source of truth. Link: [View](docs/coverage-baseline-map.json)
- `docs/development.md` - Purpose: contributor-facing development workflow reference. Bottom line: this is the canonical local dev setup guide. Link: [View](docs/development.md)
- `docs/e2e_scenario_matrix.json` - Purpose: machine-readable matrix of E2E scenarios. Bottom line: E2E scope and expected flows are defined here. Link: [View](docs/e2e_scenario_matrix.json)
- `docs/evidence-contract-schema.json` - Purpose: schema for evidence artifacts and logs. Bottom line: evidence producers must conform to this contract. Link: [View](docs/evidence-contract-schema.json)
- `docs/ext-compat.md` - Purpose: explain extension compatibility posture and boundaries. Bottom line: this is the quick compatibility reference. Link: [View](docs/ext-compat.md)
- `docs/extension-api-matrix.json` - Purpose: API surface matrix for extension host/runtime calls. Bottom line: use this to see supported extension APIs at a glance. Link: [View](docs/extension-api-matrix.json)
- `docs/extension-architecture.md` - Purpose: architecture deep dive for extension runtime internals. Bottom line: this is the primary extension technical design doc. Link: [View](docs/extension-architecture.md)
- `docs/extension-artifact-provenance.json` - Purpose: provenance metadata for extension artifacts. Bottom line: artifact trust and lineage are tracked here. Link: [View](docs/extension-artifact-provenance.json)
- `docs/extension-candidate-pool.json` - Purpose: machine-readable candidate pool for extension selection. Bottom line: ingestion/selection jobs should read from this pool. Link: [View](docs/extension-candidate-pool.json)
- `docs/extension-catalog.json` - Purpose: master extension catalog and status metadata. Bottom line: this is the canonical extension inventory. Link: [View](docs/extension-catalog.json)
- `docs/extension-catalog.schema.json` - Purpose: schema for validating extension catalog documents. Bottom line: catalog updates must pass this schema. Link: [View](docs/extension-catalog.schema.json)
- `docs/extension-code-search-inventory.json` - Purpose: inventory of code-search findings across extension repos. Bottom line: use this as raw search evidence. Link: [View](docs/extension-code-search-inventory.json)
- `docs/extension-code-search-summary.json` - Purpose: summarized results from extension code-search inventory. Bottom line: top code-search findings are condensed here. Link: [View](docs/extension-code-search-summary.json)
- `docs/extension-collections.json` - Purpose: grouped extension collections by theme/scope. Bottom line: collection-level planning starts here. Link: [View](docs/extension-collections.json)
- `docs/extension-compatibility-matrix.md` - Purpose: compatibility matrix for extension runtime support levels. Bottom line: check this before claiming compatibility gaps. Link: [View](docs/extension-compatibility-matrix.md)
- `docs/extension-conformance-matrix.json` - Purpose: machine-readable extension conformance outcomes. Bottom line: this is the conformance truth table for automation. Link: [View](docs/extension-conformance-matrix.json)
- `docs/extension-conformance-test-plan.json` - Purpose: test planning artifact for extension conformance. Bottom line: conformance execution should follow this plan. Link: [View](docs/extension-conformance-test-plan.json)
- `docs/extension-curated-list-summary.json` - Purpose: summary of curated extension subsets. Bottom line: use for quick curated corpus decisions. Link: [View](docs/extension-curated-list-summary.json)
- `docs/extension-entry-scan.json` - Purpose: scan results for extension entrypoint detection. Bottom line: extension entry assumptions are validated here. Link: [View](docs/extension-entry-scan.json)
- `docs/extension-inclusion-list.json` - Purpose: explicit inclusion list for extension sets. Bottom line: this controls what is in-scope for runs. Link: [View](docs/extension-inclusion-list.json)
- `docs/extension-individual-enumeration.json` - Purpose: per-extension enumeration details and metadata. Bottom line: drill into individual extension records here. Link: [View](docs/extension-individual-enumeration.json)
- `docs/extension-license-report.json` - Purpose: license audit results for extension corpus. Bottom line: licensing risks and statuses are captured here. Link: [View](docs/extension-license-report.json)
- `docs/extension-master-catalog.json` - Purpose: high-authority upstream extension catalog snapshot. Bottom line: this anchors full-corpus sync and provenance checks. Link: [View](docs/extension-master-catalog.json)
- `docs/extension-npm-scan-summary.json` - Purpose: summary of npm-based extension scanning outputs. Bottom line: npm scan posture is summarized here. Link: [View](docs/extension-npm-scan-summary.json)
- `docs/extension-onboarding-queue.json` - Purpose: machine-readable onboarding queue for extension work. Bottom line: queue automation should consume this file. Link: [View](docs/extension-onboarding-queue.json)
- `docs/extension-onboarding-queue.md` - Purpose: human-readable extension onboarding queue and context. Bottom line: this is the operator queue board. Link: [View](docs/extension-onboarding-queue.md)
- `docs/extension-priority-summary.json` - Purpose: summary of extension prioritization signals. Bottom line: priority rollups are centralized here. Link: [View](docs/extension-priority-summary.json)
- `docs/extension-priority.json` - Purpose: detailed extension priority scoring data. Bottom line: per-extension prioritization is machine-readable here. Link: [View](docs/extension-priority.json)
- `docs/extension-registry.md` - Purpose: document extension registry model and usage. Bottom line: registry behavior and expectations are defined here. Link: [View](docs/extension-registry.md)
- `docs/extension-repo-search-summary.json` - Purpose: summary of repository-level extension discovery searches. Bottom line: repo discovery outcomes are consolidated here. Link: [View](docs/extension-repo-search-summary.json)
- `docs/extension-research-playbook.json` - Purpose: machine-readable playbook for extension research workflow. Bottom line: research execution steps and outputs are formalized here. Link: [View](docs/extension-research-playbook.json)
- `docs/extension-runtime-threat-model.md` - Purpose: runtime-focused extension threat model with controls/tests. Bottom line: this maps concrete runtime abuse paths to mitigations. Link: [View](docs/extension-runtime-threat-model.md)
- `docs/extension-sample.json` - Purpose: machine-readable extension sample payload/spec. Bottom line: use this as canonical sample JSON contract. Link: [View](docs/extension-sample.json)
- `docs/extension-tiered-corpus.json` - Purpose: tiered corpus composition for extension testing. Bottom line: corpus tiers and membership are defined here. Link: [View](docs/extension-tiered-corpus.json)
- `docs/extension-tiered-summary.json` - Purpose: summary of tiered corpus coverage and counts. Bottom line: corpus tier health is summarized here. Link: [View](docs/extension-tiered-summary.json)
- `docs/extension-troubleshooting.md` - Purpose: troubleshooting guide specific to extension runtime issues. Bottom line: start here for extension failures and policy denials. Link: [View](docs/extension-troubleshooting.md)
- `docs/extension-validated-dedup.json` - Purpose: deduplicated validated extension list. Bottom line: use this to avoid duplicate extension artifacts. Link: [View](docs/extension-validated-dedup.json)

### 2. Core Ops and UX

- `docs/flake-triage-policy.md` - Purpose: policy for test flake detection and triage handling. Bottom line: flakes must be classified and handled via this rubric. Link: [View](docs/flake-triage-policy.md)
- `docs/keybindings.md` - Purpose: keybinding reference for interactive TUI usage. Bottom line: this is the operator shortcut map. Link: [View](docs/keybindings.md)
- `docs/models.md` - Purpose: model catalog behavior, selection, and overrides. Bottom line: model resolution logic is documented here. Link: [View](docs/models.md)
- `docs/non-mock-rubric.json` - Purpose: rubric defining non-mock testing expectations. Bottom line: use this to gate real-behavior evidence quality. Link: [View](docs/non-mock-rubric.json)
- `docs/packages.md` - Purpose: package installation and package-content conventions. Bottom line: package usage and structure are defined here. Link: [View](docs/packages.md)
- `docs/dropin-certification-contract.json` - Purpose: strict drop-in certification contract and gate thresholds. Bottom line: strict replacement messaging is allowed only when this contract is satisfied and the verdict artifact is `CERTIFIED`. Link: [View](docs/dropin-certification-contract.json)
- `docs/dropin-parity-gap-ledger.json` - Purpose: machine-readable ledger of known drop-in parity gaps and severity. Bottom line: unresolved critical/high gaps block strict replacement messaging. Link: [View](docs/dropin-parity-gap-ledger.json)
- `docs/integrator-migration-playbook.md` - Purpose: operator/integrator migration and rollback playbook for moving from TypeScript Pi to Rust Pi. Bottom line: use this to run staged, evidence-backed migrations. Link: [View](docs/integrator-migration-playbook.md)
- `docs/parity-certification.json` - Purpose: machine-readable parity progress snapshot. Bottom line: informational status only; strict replacement release claims are controlled by the drop-in contract plus verdict artifact. Link: [View](docs/parity-certification.json)
- `docs/program-governance.md` - Purpose: governance model for roadmap, gates, and ownership. Bottom line: governance decisions and responsibilities are defined here. Link: [View](docs/program-governance.md)
- `docs/prompt-templates.md` - Purpose: prompt template system and usage guide. Bottom line: reusable prompt behaviors are managed via this doc. Link: [View](docs/prompt-templates.md)
- `docs/sdk.md` - Purpose: SDK cookbook and migration guide for embedding Pi programmatically. Bottom line: use this for copy/paste Rust equivalents of TypeScript SDK workflows. Link: [View](docs/sdk.md)

### 3. Provider Subsystem

- `docs/provider-audit-evidence-index.json` - Purpose: index of provider audit evidence artifacts. Bottom line: audit traceability begins with this index. Link: [View](docs/provider-audit-evidence-index.json)
- `docs/provider-audit-reconciliation-ledger.json` - Purpose: ledger of provider audit reconciliation decisions. Bottom line: discrepancies and resolutions are recorded here. Link: [View](docs/provider-audit-reconciliation-ledger.json)
- `docs/provider-auth-crosswalk.json` - Purpose: provider auth alias/env-key crosswalk. Bottom line: credential resolution mapping is centralized here. Link: [View](docs/provider-auth-crosswalk.json)
- `docs/provider-auth-failure-signatures.json` - Purpose: known provider auth failure signatures and fingerprints. Bottom line: diagnose auth failures by matching this catalog. Link: [View](docs/provider-auth-failure-signatures.json)
- `docs/provider-auth-playbook-validation.json` - Purpose: validation outcomes for provider auth playbook coverage. Bottom line: auth playbook quality is measured here. Link: [View](docs/provider-auth-playbook-validation.json)
- `docs/provider-auth-redaction-diagnostics.json` - Purpose: diagnostics for provider auth redaction behavior. Bottom line: redaction correctness and gaps are captured here. Link: [View](docs/provider-auth-redaction-diagnostics.json)
- `docs/provider-auth-troubleshooting.md` - Purpose: troubleshooting guide for provider auth issues. Bottom line: use this first for provider key/auth problems. Link: [View](docs/provider-auth-troubleshooting.md)
- `docs/provider-baseline-audit.json` - Purpose: machine-readable baseline provider audit data. Bottom line: structured provider baseline evidence lives here. Link: [View](docs/provider-baseline-audit.json)
- `docs/provider-baseline-audit.md` - Purpose: narrative baseline provider audit report. Bottom line: high-level provider gap picture is explained here. Link: [View](docs/provider-baseline-audit.md)
- `docs/provider-canonical-id-policy.json` - Purpose: machine-readable canonical provider ID policy. Bottom line: provider ID normalization rules are defined here. Link: [View](docs/provider-canonical-id-policy.json)
- `docs/provider-canonical-id-policy.md` - Purpose: human-readable canonical provider ID policy. Bottom line: this is the normative provider naming policy. Link: [View](docs/provider-canonical-id-policy.md)
- `docs/provider-canonical-id-table.json` - Purpose: canonical provider ID lookup table. Bottom line: use this for deterministic provider ID mapping. Link: [View](docs/provider-canonical-id-table.json)
- `docs/provider-cerebras-capability-profile.json` - Purpose: Cerebras provider capability profile. Bottom line: Cerebras support envelope and constraints are here. Link: [View](docs/provider-cerebras-capability-profile.json)
- `docs/provider-cerebras-setup.json` - Purpose: Cerebras setup/config contract. Bottom line: configure Cerebras using this artifact. Link: [View](docs/provider-cerebras-setup.json)
- `docs/provider-closure-truth-table.json` - Purpose: closure truth table for provider audit status. Bottom line: this is the provider closure scoreboard. Link: [View](docs/provider-closure-truth-table.json)
- `docs/provider-config-examples.json` - Purpose: machine-readable provider config examples. Bottom line: automation-ready sample configs live here. Link: [View](docs/provider-config-examples.json)
- `docs/provider-config-examples.md` - Purpose: human-readable provider config examples. Bottom line: copy from here when setting up providers. Link: [View](docs/provider-config-examples.md)
- `docs/provider-discrepancy-classification.json` - Purpose: taxonomy for provider discrepancy classes. Bottom line: classify provider mismatches with this schema. Link: [View](docs/provider-discrepancy-classification.json)
- `docs/provider-discrepancy-ledger.json` - Purpose: ledger of concrete provider discrepancies. Bottom line: discrepancy history and status are tracked here. Link: [View](docs/provider-discrepancy-ledger.json)
- `docs/provider-gaps-audit-report.json` - Purpose: aggregate provider gap audit report. Bottom line: provider parity gaps are quantified here. Link: [View](docs/provider-gaps-audit-report.json)
- `docs/provider-gaps-test-matrix.json` - Purpose: test matrix for closing provider gaps. Bottom line: missing provider tests are enumerated here. Link: [View](docs/provider-gaps-test-matrix.json)
- `docs/provider-gate-compiler-report.json` - Purpose: provider gate report from compiler/build checks. Bottom line: compile-time provider gate status is here. Link: [View](docs/provider-gate-compiler-report.json)
- `docs/provider-gate-e2e-report.json` - Purpose: provider gate report from end-to-end tests. Bottom line: provider E2E gate results are captured here. Link: [View](docs/provider-gate-e2e-report.json)
- `docs/provider-gate-tests-report.json` - Purpose: consolidated provider test gate report. Bottom line: provider test gate pass/fail view is here. Link: [View](docs/provider-gate-tests-report.json)
- `docs/provider-gate-triage-matrix.json` - Purpose: triage matrix for provider gate failures. Bottom line: use this to route provider gate failures fast. Link: [View](docs/provider-gate-triage-matrix.json)
- `docs/provider-gate-ubs-report.json` - Purpose: UBS scan output for provider gate runs. Bottom line: provider bug-scan findings are recorded here. Link: [View](docs/provider-gate-ubs-report.json)
- `docs/provider-groq-capability-profile.json` - Purpose: Groq provider capability profile. Bottom line: Groq support/limits are defined here. Link: [View](docs/provider-groq-capability-profile.json)
- `docs/provider-groq-setup.json` - Purpose: Groq provider setup contract. Bottom line: configure Groq following this spec. Link: [View](docs/provider-groq-setup.json)
- `docs/provider-implementation-modes.json` - Purpose: provider implementation mode classification. Bottom line: know which providers are native/bridged/mock from this file. Link: [View](docs/provider-implementation-modes.json)
- `docs/provider-kimi-capability-profile.json` - Purpose: Kimi provider capability profile. Bottom line: Kimi behavior envelope is documented here. Link: [View](docs/provider-kimi-capability-profile.json)
- `docs/provider-kimi-setup.json` - Purpose: Kimi provider setup contract. Bottom line: use this for Kimi integration setup. Link: [View](docs/provider-kimi-setup.json)
- `docs/provider-longtail-evidence.md` - Purpose: evidence report for longtail provider coverage. Bottom line: longtail provider support claims are justified here. Link: [View](docs/provider-longtail-evidence.md)
- `docs/provider-migration-guide.md` - Purpose: migration guide for provider model changes. Bottom line: follow this when migrating provider integrations. Link: [View](docs/provider-migration-guide.md)
- `docs/provider-native-parity-report.json` - Purpose: report on native provider parity status. Bottom line: native parity progress is measured here. Link: [View](docs/provider-native-parity-report.json)
- `docs/provider-onboarding-checklist.md` - Purpose: onboarding checklist for adding new providers. Bottom line: provider additions must pass this checklist. Link: [View](docs/provider-onboarding-checklist.md)
- `docs/provider-onboarding-playbook.md` - Purpose: operational playbook for provider onboarding lifecycle. Bottom line: this is the end-to-end onboarding runbook. Link: [View](docs/provider-onboarding-playbook.md)
- `docs/provider-openrouter-auth-contract.json` - Purpose: OpenRouter auth behavior contract. Bottom line: OpenRouter auth resolution must match this contract. Link: [View](docs/provider-openrouter-auth-contract.json)
- `docs/provider-openrouter-capability-profile.json` - Purpose: OpenRouter capability profile. Bottom line: OpenRouter feature boundaries are documented here. Link: [View](docs/provider-openrouter-capability-profile.json)
- `docs/provider-openrouter-dynamic-models-contract.json` - Purpose: contract for OpenRouter dynamic model behavior. Bottom line: dynamic model handling rules are defined here. Link: [View](docs/provider-openrouter-dynamic-models-contract.json)
- `docs/provider-openrouter-model-registry-contract.json` - Purpose: OpenRouter model registry contract and expectations. Bottom line: registry consistency requirements are here. Link: [View](docs/provider-openrouter-model-registry-contract.json)
- `docs/provider-openrouter-setup.json` - Purpose: OpenRouter setup/config contract. Bottom line: configure OpenRouter from this artifact. Link: [View](docs/provider-openrouter-setup.json)
- `docs/provider-parity-checklist.json` - Purpose: checklist for provider parity closure criteria. Bottom line: parity completion gates are encoded here. Link: [View](docs/provider-parity-checklist.json)
- `docs/provider-parity-reconciliation-report.json` - Purpose: report on provider parity reconciliations. Bottom line: reconciliation outcomes and rationale are here. Link: [View](docs/provider-parity-reconciliation-report.json)
- `docs/provider-parity-reconciliation.json` - Purpose: machine-readable parity reconciliation ledger. Bottom line: this is the structured parity reconciliation record. Link: [View](docs/provider-parity-reconciliation.json)
- `docs/provider-qwen-capability-profile.json` - Purpose: Qwen provider capability profile. Bottom line: Qwen support/constraints are captured here. Link: [View](docs/provider-qwen-capability-profile.json)
- `docs/provider-qwen-setup.json` - Purpose: Qwen provider setup contract. Bottom line: configure Qwen using this file. Link: [View](docs/provider-qwen-setup.json)
- `docs/provider-remediation-manifest.json` - Purpose: manifest of provider remediation actions. Bottom line: remediation backlog and ownership are tracked here. Link: [View](docs/provider-remediation-manifest.json)
- `docs/provider-remediation-routing-validation.json` - Purpose: validation of provider remediation routing logic. Bottom line: routing correctness is evidenced here. Link: [View](docs/provider-remediation-routing-validation.json)
- `docs/provider-support-baseline-audit.md` - Purpose: baseline audit of declared provider support. Bottom line: support claims are grounded by this audit. Link: [View](docs/provider-support-baseline-audit.md)
- `docs/provider-test-matrix-validation-report.json` - Purpose: validation report for provider test matrix completeness. Bottom line: provider test matrix health is measured here. Link: [View](docs/provider-test-matrix-validation-report.json)
- `docs/provider-test-obligations.md` - Purpose: define provider-specific test obligations and gates. Bottom line: this is the provider test contract. Link: [View](docs/provider-test-obligations.md)
- `docs/provider-upstream-catalog-snapshot.json` - Purpose: machine-readable snapshot of upstream provider catalogs. Bottom line: upstream drift detection starts from this snapshot. Link: [View](docs/provider-upstream-catalog-snapshot.json)
- `docs/provider-upstream-catalog-snapshot.md` - Purpose: narrative summary of upstream provider snapshot changes. Bottom line: upstream provider changes are explained here. Link: [View](docs/provider-upstream-catalog-snapshot.md)
- `docs/provider_e2e_artifact_contract.json` - Purpose: artifact contract for provider E2E evidence. Bottom line: provider E2E artifacts must satisfy this schema. Link: [View](docs/provider_e2e_artifact_contract.json)
- `docs/providers.md` - Purpose: provider architecture, behavior, and usage reference. Bottom line: this is the primary provider subsystem guide. Link: [View](docs/providers.md)

### 4. QA, Schemas, Security, and Platform

- `docs/qa-runbook.md` - Purpose: QA operating runbook across suites and artifacts. Bottom line: use this for repeatable QA execution. Link: [View](docs/qa-runbook.md)
- `docs/releasing.md` - Purpose: release process and checklist documentation. Bottom line: follow this for consistent, safe releases. Link: [View](docs/releasing.md)
- `docs/rpc.md` - Purpose: RPC mode protocol and usage contract. Bottom line: this is the wire-level RPC integration guide. Link: [View](docs/rpc.md)
- `docs/schema/extension_manifest.json` - Purpose: JSON schema for extension manifests. Bottom line: extension manifest validation must pass this schema. Link: [View](docs/schema/extension_manifest.json)
- `docs/schema/extension_protocol.json` - Purpose: JSON schema for extension protocol messages. Bottom line: extension message contracts are formalized here. Link: [View](docs/schema/extension_protocol.json)
- `docs/schema/mock_spec.json` - Purpose: schema for mock/test spec fixtures. Bottom line: mock fixtures should conform to this contract. Link: [View](docs/schema/mock_spec.json)
- `docs/schema/runtime_hostcall_telemetry.json` - Purpose: JSON schema for runtime hostcall telemetry events. Bottom line: hostcall telemetry producers must conform to this schema. Link: [View](docs/schema/runtime_hostcall_telemetry.json)
- `docs/sec_traceability_matrix.json` - Purpose: machine-readable security requirement traceability matrix. Bottom line: security coverage and test mappings are tracked here. Link: [View](docs/sec_traceability_matrix.json)
- `docs/sec_traceability_matrix.md` - Purpose: narrative security traceability matrix with requirement-to-test linkage. Bottom line: human-readable security coverage status is documented here. Link: [View](docs/sec_traceability_matrix.md)
- `docs/security/baseline-audit.md` - Purpose: code-grounded security baseline audit and gap analysis. Bottom line: this is the current security posture snapshot. Link: [View](docs/security/baseline-audit.md)
- `docs/security/incident-response-runbook.md` - Purpose: incident response procedures for security events. Bottom line: follow this runbook during active security incidents. Link: [View](docs/security/incident-response-runbook.md)
- `docs/security/incident-runbook.md` - Purpose: incident detection, classification, and handling guide. Bottom line: use this for initial incident triage and escalation. Link: [View](docs/security/incident-runbook.md)
- `docs/security/invariants.machine.json` - Purpose: machine-checkable security invariant manifest with test mappings. Bottom line: invariant automation should read this file. Link: [View](docs/security/invariants.machine.json)
- `docs/security/invariants.md` - Purpose: normative security invariants and precedence semantics. Bottom line: this is the SEC-1.2 policy/risk contract. Link: [View](docs/security/invariants.md)
- `docs/security/lockfile-format.md` - Purpose: lockfile format specification for extension integrity verification. Bottom line: lockfile structure and validation rules are defined here. Link: [View](docs/security/lockfile-format.md)
- `docs/security/maintenance-playbook.md` - Purpose: security maintenance procedures and scheduled operations. Bottom line: use this for routine security upkeep and policy refresh. Link: [View](docs/security/maintenance-playbook.md)
- `docs/security/manifest-v2-migration.md` - Purpose: migration guidance from legacy extension manifests to manifest v2 security fields. Bottom line: use this to upgrade manifests without losing capability/policy intent. Link: [View](docs/security/manifest-v2-migration.md)
- `docs/security/operator-handbook.md` - Purpose: comprehensive operator handbook for security operations. Bottom line: this is the primary security ops reference for day-to-day work. Link: [View](docs/security/operator-handbook.md)
- `docs/security/operator-quick-reference.md` - Purpose: quick-reference card for security operators. Bottom line: use this cheat sheet for fast lookups during operations. Link: [View](docs/security/operator-quick-reference.md)
- `docs/security/policy-tuning-guide.md` - Purpose: guide for tuning extension security policies and risk thresholds. Bottom line: policy calibration and adjustment procedures are here. Link: [View](docs/security/policy-tuning-guide.md)
- `docs/security/runtime-hostcall-telemetry.md` - Purpose: runtime hostcall telemetry design and event catalog. Bottom line: hostcall observability and event semantics are documented here. Link: [View](docs/security/runtime-hostcall-telemetry.md)
- `docs/security/security-slos.md` - Purpose: quantitative security SLOs, risk budgets, and release/rollback gates. Bottom line: security release readiness is numerically gated here. Link: [View](docs/security/security-slos.md)
- `docs/security/threat-model.md` - Purpose: formal extension ecosystem threat model. Bottom line: this is the SEC-1.1 attacker and control baseline. Link: [View](docs/security/threat-model.md)
- `docs/session.md` - Purpose: session model, persistence, and branching semantics. Bottom line: session behavior and storage contracts are here. Link: [View](docs/session.md)
- `docs/settings.md` - Purpose: settings schema, precedence, and config behavior. Bottom line: configuration behavior is canonicalized here. Link: [View](docs/settings.md)
- `docs/skills.md` - Purpose: skills system usage and packaging guidance. Bottom line: extend prompt behavior through documented skill contracts. Link: [View](docs/skills.md)
- `docs/streaming-hostcalls.md` - Purpose: streaming hostcall behavior and lifecycle details. Bottom line: use this to reason about streaming tool execution. Link: [View](docs/streaming-hostcalls.md)
- `docs/terminal-setup.md` - Purpose: terminal environment setup and ergonomics guidance. Bottom line: recommended terminal configuration is here. Link: [View](docs/terminal-setup.md)
- `docs/termux.md` - Purpose: Termux-specific setup and runtime guidance. Bottom line: Android/Termux usage is documented here. Link: [View](docs/termux.md)
- `docs/test_double_inventory.json` - Purpose: inventory of test doubles and mock surfaces. Bottom line: test double usage is auditable via this file. Link: [View](docs/test_double_inventory.json)
- `docs/testing-policy.md` - Purpose: testing policy, quality bars, and suite expectations. Bottom line: this is the normative test governance doc. Link: [View](docs/testing-policy.md)
- `docs/themes.md` - Purpose: theme system configuration and customization. Bottom line: terminal rendering themes are documented here. Link: [View](docs/themes.md)
- `docs/traceability_matrix.json` - Purpose: requirement-to-test traceability matrix. Bottom line: evidence traceability across requirements lives here. Link: [View](docs/traceability_matrix.json)
- `docs/tree.md` - Purpose: session tree navigation and branching behavior guide. Bottom line: use this to understand conversation tree operations. Link: [View](docs/tree.md)
- `docs/troubleshooting.md` - Purpose: general troubleshooting guide for common failures. Bottom line: start here for non-extension operational issues. Link: [View](docs/troubleshooting.md)
- `docs/tui.md` - Purpose: TUI behavior, controls, and rendering notes. Bottom line: interactive UI semantics are defined here. Link: [View](docs/tui.md)
- `docs/windows.md` - Purpose: Windows-specific installation and runtime guidance. Bottom line: Windows support details are centralized here. Link: [View](docs/windows.md)
- `docs/wit/extension.wit` - Purpose: WIT interface definition for extension host contracts. Bottom line: typed extension host ABI is defined here. Link: [View](docs/wit/extension.wit)

---

## About Contributions

Please don't take this the wrong way, but I do not accept outside contributions for any of my projects. I simply don't have the mental bandwidth to review anything, and it's my name on the thing, so I'm responsible for any problems it causes; thus, the risk-reward is highly asymmetric from my perspective. I'd also have to worry about other "stakeholders," which seems unwise for tools I mostly make for myself for free. Feel free to submit issues, and even PRs if you want to illustrate a proposed fix, but know I won't merge them directly. Instead, I'll have Claude or Codex review submissions via `gh` and independently decide whether and how to address them. Bug reports in particular are welcome. Sorry if this offends, but I want to avoid wasted time and hurt feelings. I understand this isn't in sync with the prevailing open-source ethos that seeks community contributions, but it's the only way I can move at this velocity and keep my sanity.

---

## License

MIT License. See [LICENSE](LICENSE) for details.

---

<p align="center">
  <sub>Built with Rust, for developers who live in the terminal.</sub>
</p>
