# Synth Warp — Local-first fork of the Warp terminal client

> Agent orientation for this checkout. Engineering commands and Warp-specific conventions live in
> [`AGENTS.md`](AGENTS.md). Run `python scripts/sync_assistant_trees.py` after editing `.claude/`
> assets; it updates Cursor and Codex mirrors.

## Related Projects

| Project | Repository | Role |
|---------|------------|------|
| Synth Warp (this) | https://github.com/synthet/warp | Local-first Warp client fork |
| synthet-code-framework | sibling `../synthet-code-framework` | Agent scaffold this repo adopted |

## Backlog & queue

Work comes from **GitHub issues** and existing specs (`specs/`, `.agents/specs/`, `agents/specs/`).
Do not invent items in `.agent/backlog/items.md`. Optional `/task-claim` docs remain under
[`.agent/backlog/`](.agent/backlog/README.md).

## Architecture

Two front-ends share `warp_core` / `warpui` (Entity/model core, actions, appearance, feature flags):

| Module / component | Role |
|--------------------|------|
| `app/` | GUI desktop app on WarpUI (GPU/WGSL) |
| `crates/warp_tui` | Headless TUI front-end |
| `crates/warp_core` | Shared utilities and platform abstractions |
| `crates/warpui`, `crates/warpui_core` | Shared UI core; GUI elements plus TUI cell-grid |
| `crates/editor` | Text editing |
| `crates/ipc` | Inter-process communication |
| `crates/graphql` | GraphQL client |
| `crates/integration` | GUI-only integration tests |

## Key Files

- [`AGENTS.md`](AGENTS.md) — build/test/lint commands and Warp coding conventions
- [`docs/CANONICAL_SOURCES.md`](docs/CANONICAL_SOURCES.md) — authority map
- [`.agent/SAFETY.md`](.agent/SAFETY.md) — safety rules
- `.claude/skills/` — canonical skills (framework + Warp domain). Mirrors: `.cursor/skills/`, `.agents/skills/`

Warp domain skills (GUI/TUI/feature-flag workflows) stay in `.claude/skills/` next to framework skills.
Author Warp skills there, then sync.

## Commands

```bash
./script/run
./script/run-tui
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2
./script/format
cargo clippy --workspace --all-targets --all-features --tests -- -D warnings
./script/presubmit
```

Fast subset: `cargo nextest run -p <crate>`. Full suite: `./script/presubmit`.

## Testing

| You say | Canonical name | Where | How to run |
|---------|----------------|-------|------------|
| unit | crate unit tests | `*_tests.rs` / `mod_test.rs` | `cargo nextest run -p <crate>` |
| nextest | workspace nextest | workspace | `cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2` |
| GUI integration | WarpUI integration tests | `crates/integration` | `gui-integration-test` skill |
| TUI | render-to-lines unit tests | `crates/warp_tui` | `tui-testing` skill |

## Tool permissions and write access

- **Default read-only mode:** the scaffolded `.claude/settings.json` only allows read-oriented inspection (`git status`, `git diff:*`, `git log:*`) plus `WebSearch`.
- **Local writes are opt-in:** to let an agent stage or commit local changes, copy or merge `.claude/settings.write.example.json` into the active Claude settings for that workspace, preferably enabling only the entries needed for the current task.
- **Remote writes are separate:** GitHub mutations through `gh pr:*`, `gh issue:*`, or `gh project:*` affect shared remote state and may notify people; enable them only after explicit task intent and target verification.
- **External export approval:** exporting code, prompts, logs, or generated artifacts to external services/providers requires explicit approval and a secrets check, even when local writes are already allowed.

## Development Guidelines

Follow [`AGENTS.md`](AGENTS.md) for Warp-specific style, comments, terminal-model locking, feature flags, and exhaustive matching. Additional scaffold rules:

- **Minimal diffs** — prefer targeted edits over rewrites; no drive-by refactors.
- **Secrets** go in `secrets.json` / `.env` (git-ignored), never in committed config.
- **Never modify `.git/config`**.

## Documentation

Start with [`docs/CANONICAL_SOURCES.md`](docs/CANONICAL_SOURCES.md), then
[`docs/WIKI_SCHEMA.md`](docs/WIKI_SCHEMA.md) when adding wiki pages.

- [`AGENTS.md`](AGENTS.md) — MCP config, tool surface, agent contract
- [`docs/ai-workflow/README.md`](docs/ai-workflow/README.md) — agent asset map + SDLC loop
- [`.agent/SAFETY.md`](.agent/SAFETY.md) — safety & hygiene rules
- [`.agent/AGENT_INFRA_INVENTORY.md`](.agent/AGENT_INFRA_INVENTORY.md) — full agent-infra inventory
