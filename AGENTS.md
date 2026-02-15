# AGENTS.md

## Project
`bb` is a Linux/macOS command-line utility that turns natural-language requests into executable shell commands.

It is distributed as a **single binary** but runs in two modes:
- **Client mode** (default): short-lived, responsive CLI entrypoint.
- **Server/Daemon mode** (`--daemon`): long-lived background process that handles LLM work and IPC requests.

## Product Goal
When a user types `bb`, the tool should:
1. Ensure daemon availability (autospawn if needed).
2. Send user natural-language request to daemon over local IPC.
3. Receive a generated shell command.
4. Return control with that command prefilled for review/edit/execute in the shell.

No automatic execution of generated command by default.

## Architecture (Single Binary, Dual Mode)

### Process Roles
- Client path:
  - Parse CLI args.
  - Resolve runtime paths.
  - Probe daemon health.
  - Spawn daemon if absent.
  - Collect prompt input.
  - Send `ClientRequest`.
  - Receive `ServerResponse`.
  - Hand command to shell integration contract.
- Daemon path:
  - Acquire singleton lock.
  - Bind Unix socket.
  - Serve requests concurrently.
  - Route request to LLM provider abstraction.
  - Return command payload.

### Module Layout
- `src/main.rs`: mode routing and top-level startup.
- `src/client.rs`: client workflow orchestration.
- `src/server.rs`: daemon workflow orchestration.
- `src/daemon/*`: lock/pid/spawn/runtime concerns.
- `src/ipc/*`: Unix socket transport + framing.
- `src/shell/*`: shell contract payload helpers.
- `src/llm/*`: provider abstraction and adapters.
- `src/protocol.rs`: request/response schemas.
- `src/config.rs`: runtime paths and settings.
- `src/error.rs`: shared error model.

## Shell Buffer Injection Strategy (Bash/Zsh)

### Core Constraint
A child process cannot portably and safely mutate the parent shell’s interactive buffer directly.  
Therefore, shell-prefill must be done by shell-native wrapper code running in the parent shell.

### Selected Strategy
- Strict `bb` + Enter workflow requested.
- `bb` shell function is required (not raw binary invocation).
- Binary returns structured payload; wrapper injects command into interactive context.

### Zsh Strategy
- Use Zsh-native wrapper in `shell/bb.zsh`.
- Use ZLE-aware path for buffer insertion.
- Primary behavior: place generated command in editable command line buffer (not auto-run).

### Bash Strategy (selected: prompt-command rewrite)
- Use wrapper in `shell/bb.bash`.
- `bb` function stores generated command into pending variable.
- `PROMPT_COMMAND` hook attempts line-buffer priming before prompt accept cycle.
- If line-buffer priming is unavailable in a given Bash/readline environment, fallback:
  - `history -s "<cmd>"` with immediate user-visible guidance.
- Keep behavior non-destructive and never auto-execute generated command.

### Security/UX Rules
- Never eval remote text automatically.
- Escape/sanitize shell-visible output in wrappers.
- Keep generated command as plain editable text for user confirmation.
- Provide explicit shell install instructions (`source shell/bb.bash` / `source shell/bb.zsh`).

## Daemon Autospawn, Locking, and IPC Strategy

### Runtime Paths
Use per-user runtime directory:
- Prefer `$XDG_RUNTIME_DIR/bb/` when present.
- Fallback to `/tmp/bb-$UID/` on Linux/macOS.

Files:
- `bb.sock` (Unix domain socket)
- `bb.pid` (pid file)
- `bb.lock` (singleton lock file)

### Client Startup Flow
1. Compute runtime paths.
2. Probe daemon lock state via non-blocking exclusive lock attempt on `bb.lock`.
   - If lock is held (`WouldBlock`), daemon is considered running/starting.
   - If lock is free, release probe lock immediately and spawn same binary with `--daemon`.
3. Wait for socket readiness with bounded retry/backoff.
4. Send request over socket.
5. Return response to shell wrapper contract.

### Daemon Startup Flow
1. Acquire exclusive lock (`bb.lock`).
2. If lock cannot be acquired (`WouldBlock`), exit immediately (singleton enforcement).
3. Keep lock fd open for whole daemon lifetime (OS releases lock on crash/exit).
4. Bind Unix socket with user-only permissions.
5. Start async request loop.
6. Handle shutdown signals; cleanup pid/socket/lock.

### IPC Transport
- Unix domain sockets via Tokio.
- Framing: newline-delimited JSON (`serde_json`) or length-prefixed JSON.
- Include protocol version field for forward compatibility.
- Require request IDs for correlation and logs.

### Reliability and Security
- Enforce socket file mode to user-only.
- Reject malformed/oversized payloads.
- Set client timeouts and retry policy.
- Use idempotent ping handshake.
- Rely on OS lock release on daemon crash/exit; never trust lock-file existence alone.

## Dependencies (Planned)
- `clap` for CLI parsing and mode flags.
- `tokio` for async runtime and Unix socket support.
- `serde`, `serde_json` for protocol serialization.
- `thiserror` and/or `anyhow` for error handling.
- `tracing`, `tracing-subscriber` for structured logs.
- `directories` (or `dirs`) for OS-aware runtime paths.
- `fs2` for cross-platform file locking (`try_lock_exclusive`).
- `nix` (optional) for Unix process/session utilities if needed.

## Implementation Roadmap (Decision-Complete)

1. **Scaffold + Contracts**
- Create module/file layout.
- Define protocol types and config path model.
- Define CLI mode flags and top-level router signatures.

2. **IPC Foundation**
- Implement socket path resolution.
- Implement framed request/response transport.
- Add ping request and timeout behavior.

3. **Daemon Lifecycle**
- Implement lock/pid/socket artifact management.
- Implement daemon mode startup and async serve loop.
- Implement signal-aware graceful shutdown and cleanup.

4. **Client Autospawn**
- Implement health probe + spawn-if-missing flow.
- Add readiness wait with bounded retries.
- Add robust error messages for startup failures.

5. **Shell Integration**
- Implement `shell/bb.zsh` and `shell/bb.bash` wrappers.
- Implement strict `bb`+Enter flow target.
- Implement Bash fallback behavior when direct prefill path is not available.
- Document install and troubleshooting.

6. **LLM Provider Abstraction**
- Add provider trait/interface and config stubs.
- Keep provider behind daemon boundary.
- Return deterministic placeholder command in early integration tests.

7. **Hardening + Observability**
- Add structured logging fields (`request_id`, pid, socket path, latency).
- Add payload limits and validation.
- Add compatibility version checks.

8. **Tests + Acceptance**
- Unit tests for path resolution, lock logic, protocol parsing.
- Integration tests for daemon autospawn and IPC roundtrip.
- Shell contract tests (payload formatting, fallback conditions).
- Manual tests on Linux and macOS for end-to-end UX.

## Acceptance Criteria
- `bb` client returns quickly and autospawns daemon when absent.
- Daemon singleton behavior enforced with lock/pid discipline.
- IPC request/response works via Unix socket on Linux/macOS.
- Shell wrappers provide editable generated command without auto-run.
- No generated command is executed without explicit user Enter.

## Known Risks / Mitigations
- Bash prefill behavior varies by readline/shell environment.
  - Mitigation: prompt-command approach plus tested fallback to history insertion.
- Multi-client spawn race.
  - Mitigation: daemon-level exclusive lock guarantees single active daemon.

## Assumptions and Defaults
- Default shell targets are Bash and Zsh only.
- One daemon per user session.
- Localhost-only IPC via Unix socket (no TCP).
- Initial LLM backend is abstracted; provider chosen later.
- Generated commands are suggestions only; user remains in control.
