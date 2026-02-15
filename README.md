# bb

`bb` is a Linux/macOS CLI that translates natural-language requests into shell commands.

The project uses a single-binary client/server design:
- default mode: short-lived client
- `--daemon` mode: long-lived background server

## Current Status

This repository is in scaffold stage:
- project structure and architecture are defined
- runtime logic (IPC lifecycle, daemon management, LLM provider, shell prefill behavior) is still in progress

See `AGENTS.md` for the full architecture and roadmap.

## Project Layout

- `src/main.rs`: CLI entrypoint and mode routing
- `src/client.rs`: client workflow orchestration
- `src/server.rs`: daemon workflow orchestration
- `src/daemon/*`: daemon lock/spawn/runtime modules
- `src/ipc/*`: IPC transport modules
- `src/shell/*`: shell integration modules
- `src/llm/*`: LLM abstraction modules
- `shell/bb.bash`, `shell/bb.zsh`: shell wrapper scaffolds
- `AGENTS.md`: source of truth for architecture and implementation plan

## Prerequisites

- Rust stable toolchain
- Linux or macOS

## Local Development

```bash
cargo build
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

Optional auto-format:

```bash
cargo fmt --all
```

## Shell Integration (Planned)

Shell wrapper files are present in `shell/`, but full line-buffer prefill behavior is not implemented yet.

## Roadmap

Implementation sequence, constraints, and acceptance criteria are documented in `AGENTS.md`.
