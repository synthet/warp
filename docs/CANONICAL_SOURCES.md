---
type: Source-of-Truth Map
title: Canonical Sources
description: Authority map — the single source of truth for each contract, convention, and config in this project.
resource: CANONICAL_SOURCES.md
tags: [docs, governance, authority]
timestamp: 2026-08-15T17:00:00Z
okf_version: 0.1
---

# Canonical sources

Agents and contributors must check this map before inventing API paths, schema names, config keys, or
status values. Fill in the right column as your project grows; the left column is the reusable
question.

| Contract / convention | Source of truth (fill in) |
|-----------------------|---------------------------|
| Public API shape | [`crates/warp_graphql_schema/api/schema.graphql`](../crates/warp_graphql_schema/api/schema.graphql) |
| Data model / schema | [`crates/persistence/src/schema.rs`](../crates/persistence/src/schema.rs), migrations in `crates/persistence/migrations/` |
| Config keys | [`crates/settings/src/schema.rs`](../crates/settings/src/schema.rs) |
| Status / state enums | Feature flags in [`crates/warp_features/src/lib.rs`](../crates/warp_features/src/lib.rs) (re-exported from [`crates/warp_core/src/features.rs`](../crates/warp_core/src/features.rs)); settings schema above. Wiki: [`technical/feature-flags.md`](technical/feature-flags.md) |
| Domain vocabulary | [`AGENTS.md`](../AGENTS.md) architecture overview; wiki: [`architecture/system-overview.md`](architecture/system-overview.md) |
| Build / test / lint commands | [`../AGENTS.md`](../AGENTS.md) |
| Windows GUI compile / resume | [`../script/windows/build.ps1`](../script/windows/build.ps1); wiki: [`guides/build-and-run.md`](guides/build-and-run.md) |
| Windows local overlay onto Program Files | [`../script/windows/deploy.ps1`](../script/windows/deploy.ps1); wiki: [`guides/windows-local-deploy.md`](guides/windows-local-deploy.md) |
| App ID / bundle identifiers | [`../crates/warp_core/src/app_id.rs`](../crates/warp_core/src/app_id.rs); wiki: [`technical/app-id.md`](technical/app-id.md) |
| App feature enable list | [`../app/src/features.rs`](../app/src/features.rs) (`enabled_features()`); enum SoT remains `warp_features` |
| Agent skill authoring | [`.claude/skills/`](../.claude/skills/) then `python scripts/sync_assistant_trees.py` — do not hand-edit `.cursor/` or `.agents/skills/` |
| Framework adopt / bootstrap | Sibling [synthet-code-framework](https://github.com/synthet/synthet-code-framework). Never `bootstrap.py --force` against this checkout. See [`ai-workflow/README.md`](ai-workflow/README.md) |
| Product / tech specs | [`specs/`](../specs/), [`.agents/specs/`](../.agents/specs/), [`agents/specs/`](../agents/specs/) |
| Codex project configuration | [`.codex/config.toml`](../.codex/config.toml) and [`.codex/README.md`](../.codex/README.md) |
| Optional file-search MCP (fff) | [fff repo](https://github.com/dmtrKovalenko/fff); template keys `fff-mcp` / `synth-warp-fff` in [`.cursor/mcp.example.json`](../.cursor/mcp.example.json) |
| CLI tooling skills spec | [`.agent/cli-tools-skills-spec.md`](../.agent/cli-tools-skills-spec.md) |
| CLI install tier order | [`.claude/skills/cli-tools-overview/references/install-tiers.md`](../.claude/skills/cli-tools-overview/references/install-tiers.md) |
| Agent CLI environment (PATH, Cursor restart) | [`.claude/skills/cli-tools-overview/references/agent-environment.md`](../.claude/skills/cli-tools-overview/references/agent-environment.md) |
| Agent assets (rules/commands/skills/agents) | [`ai-workflow/README.md`](ai-workflow/README.md) |
| Safety rules | [`../.agent/SAFETY.md`](../.agent/SAFETY.md) |
| Wiki conventions | [`WIKI_SCHEMA.md`](WIKI_SCHEMA.md) |

**Rule:** code and the written contract must never disagree. If you change one, change the other in
the same PR (see [`../.agent/workflows/cross_repo_contract_change.md`](../.agent/workflows/cross_repo_contract_change.md)).
