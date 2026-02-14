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
```

The installer is idempotent and supports a migration path from TypeScript Pi:
- Detect existing TS `pi` command
- Prompt to install Rust Pi as canonical `pi`
- Preserve old CLI behind `legacy-pi`
- Record state for clean uninstall/restore

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
│ • OpenAI        │  │  • write • grep    │  │  • Capability    │
│ • OpenAI Resp.  │  │  • edit  • find    │  │    policy        │
│ • Gemini        │  │  • ls              │  │  • Node shims    │
│ • Cohere/Azure  │  │  • ext-registered  │  │  • Event hooks   │
│ • Extensions    │  │                    │  │                  │
└────────┬────────┘  └─────────┬──────────┘  └───────┬──────────┘
         │                     │                      │
┌────────▼─────────────────────▼──────────────────────▼──────────┐
│                     Session Persistence                         │
│  • JSONL format (v3)   • Tree structure   • Session index/cache  │
│  • Per-project dirs    • Optional SQLite backend                │
└─────────────────────────────────────────────────────────────────┘
```

### Key Design Decisions

1. **No unsafe code**: `#![forbid(unsafe_code)]` enforced project-wide
2. **Streaming-first**: Custom SSE parser, no blocking on responses
3. **Process tree management**: `sysinfo` crate ensures no orphaned processes
4. **Structured errors**: `thiserror` with specific error types per component
5. **Size-optimized release**: LTO + strip + opt-level=z for lean binaries

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
| **Not all provider APIs** | Built-in support includes Anthropic, OpenAI (Chat + Responses), Gemini, Cohere, and Azure OpenAI; some ecosystem-specific APIs are still TBD |
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

**Q: Can I use OpenAI/Gemini/Cohere/Azure models?**
A: Yes. Configure the relevant provider key (`OPENAI_API_KEY`, `GOOGLE_API_KEY`, `COHERE_API_KEY`, or `AZURE_OPENAI_API_KEY`) and select with `--provider`/`--model` (for example `--provider openai --model gpt-4o`).

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
| **Providers** | Anthropic | Anthropic | Many | Many |
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

```
src/
├── main.rs          # CLI entry point
├── lib.rs           # Library exports
├── agent.rs         # Agent loop
├── cli.rs           # Argument parsing
├── config.rs        # Configuration
├── error.rs         # Error types
├── model.rs         # Message types
├── provider.rs      # Provider trait
├── models.rs        # Model registry + models.json overrides
├── providers/
│   ├── anthropic.rs
│   ├── openai.rs
│   ├── openai_responses.rs
│   ├── gemini.rs
│   ├── cohere.rs
│   ├── azure.rs
│   └── mod.rs       # Provider factory + extension bridge
├── extensions.rs    # Extension protocol + capability policy
├── extensions_js.rs # QuickJS runtime bridge
├── interactive.rs   # Interactive TUI app loop/state
├── rpc.rs           # RPC/stdio mode
├── resources.rs     # Skills/prompt/theme/extension loading
├── session.rs       # Session persistence (JSONL/tree)
├── session_index.rs # Session metadata index/cache
├── sse.rs           # SSE parser
├── tools.rs         # Built-in tools
└── tui.rs           # Terminal UI rendering helpers
```

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
