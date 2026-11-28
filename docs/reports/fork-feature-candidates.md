---
type: Report
title: Fork feature and practice survey for Synth Term
description: Verified survey of Warp and Zed forks (Zap, warp-lite, man-brain, oh-my-warp, Gram, Zedless) with a pick table for the owner. No porting.
resource: reports/fork-feature-candidates.md
tags: [docs, reports, fork, synth-term]
timestamp: 2026-11-20T06:10:00Z
okf_version: 0.1
---

# Fork feature and practice survey for Synth Term

Task T-12 of the Synth Term epic (`warp-fjamf`): find out what other forks of Warp and Zed do, so the
owner can pick what, if anything, to port later. **Nothing is ported here.** Each pick becomes its own
follow-up ticket.

Every claim was checked against the project's README, file tree, commits or releases on 2026-11-20.
"Synth" means `synthet/warp` `master` at `1501f8bf`; "upstream HEAD" means `warpdotdev/warp@9fa48263`
(2026-11-20). Search-engine summaries were not trusted; two of their claims were wrong and are corrected
below.

## Baseline: what upstream Warp already has

- **Custom inference endpoints are upstream**: third-party model endpoints arrived in
  [#10781](https://github.com/warpdotdev/warp/pull/10781) (2026-05-13), configurable endpoint schemas in
  [#13938](https://github.com/warpdotdev/warp/pull/13938) (2026-07-24). In Synth:
  `app/src/settings_view/custom_inference_modal.rs`, `app/src/ai/llms.rs`.
- **Custom model routers are upstream**: YAML files in `~/.warp/custom_model_routers/`
  ([#13010](https://github.com/warpdotdev/warp/pull/13010)); `custom-router:local:` routers are
  "serialized inline into outbound agent requests" (`app/src/ai/custom_model_routers.rs`).
- **Those requests still go through Warp's backend.** In
  [discussion #9619](https://github.com/warpdotdev/warp/discussions/9619) (2026-05-06), Warp's CEO wrote
  that BYOK and arbitrary endpoints "will still go through our server harness" and listed a client-side
  harness as future work. Later community comments in the thread report requests "still proxied through
  Warp's backend" and a VPN-only gateway failing a DNS lookup on Warp's side.
- **No client-side model loop was found upstream** (no such crate, no matching merged PR; only PR titles
  and the crate list were searched).
- **Third-party harnesses over ACP are upstream and active**: [#16329](https://github.com/warpdotdev/warp/pull/16329),
  [#16331](https://github.com/warpdotdev/warp/pull/16331), [#16905](https://github.com/warpdotdev/warp/pull/16905)
  (2026-11-18). Synth gets these by merging upstream.

## 1. Zap (formerly OpenWarp)

- [zerx-lab/zap-warp](https://github.com/zerx-lab/zap-warp) (`zerx-lab/zap` redirects there). GitHub fork of
  `warpdotdev/warp`, AGPL-3.0. **Archived**; last commit 2026-09-27, last release `v2026.07.09.1`;
  782 commits ahead of upstream, 2,687 behind. Reason for archiving and any successor are unknown.
- README claims: no account or cloud; own providers (any OpenAI-compatible endpoint, or native OpenAI,
  Anthropic, Gemini, DeepSeek, Ollama); Codex, Claude Code and other CLI agents hooked into Blocks; an SSH
  host manager with tmux; editable system prompts; UI in English, Simplified Chinese, Japanese and Russian.
- Confirmed in code: SSH manager (`app/src/ssh_manager/`, `warp_ssh_manager` dependency); prompt
  templating (`minijinja = "2"` in `app/Cargo.toml`); Fluent translations
  (`app/i18n/{en,ja,ru,zh-CN}/warp.ftl`); provider fixes in its #305/#306;
  [`docs/roadmap.md`](https://github.com/zerx-lab/zap-warp/blob/main/docs/roadmap.md) plans to move the
  agent into a standalone service.
- For Synth: no SSH manager or translations in Synth. Its own-provider support goes further than upstream,
  which configures endpoints but proxies them.

## 2. warp-lite (two unrelated projects, both AGPL-3.0)

**[terzigolu/warp-lite](https://github.com/terzigolu/warp-lite)** (macOS; last commit 2026-10-03, `v0.5.9-lite`)

- Ships without AI, login or telemetry; bundle ID `dev.warp-lite.WarpLite`.
- `FORK_NOTICE.md` says `crates/ai`, `graphql`, `warp_server_client` and others were removed; on
  `main` they are still in `crates/`, compiled out by default behind a `warp_platform` Cargo feature.
- Strongest **practice** source: sync by vetted `git cherry-pick -x` with an `upstream-tracking` branch, and a
  written audit per sync (`WARP_LITE_SYNC_2026-10.md`: 368 upstream commits reviewed, 73 adopted; per-commit
  decisions in `docs/upstream-sync/2026-10-02-commits.json`). Asserts `crates/warpui*` are byte-identical
  to upstream.
- Its feature additions (native completions, fzf/atuin handoff, history ranking) came from upstream; Synth
  gets them by merging.

**[AnxiangLemon/warp-lite](https://github.com/AnxiangLemon/warp-lite)** (Windows x64; last commit 2026-11-03, `0.1.0`)

- A `warp_lite` feature and `lite` profile drop the AWS SDK, TLS/AWS-LC, ONNX/Candle, Tantivy, libgit2 and
  QuickJS.
- Adds CPU software rendering with a DX12/Vulkan/GL choice and a 15/30 FPS cap.
- Vendors Git dependencies in `vendor/upstream/` with SHA-256 hashes in `vendor/sources.json`, checked by
  `script/verify-local-sources.py`. Deliberately never syncs upstream.
- For Synth: theme background images already exist (`app/src/themes/theme.rs`); no FPS cap or CPU-render
  toggle in Synth.

## 3. man-brain

- [man-brain/warp](https://github.com/man-brain/warp), branch `local-agent`. GitHub fork, AGPL-3.0; last
  commit 2026-08-25; 12 commits ahead of upstream, 368 behind.
- Adds `crates/local_agent` (`engine.rs`, `openai.rs`, `prompt.rs`) and runs the **agent loop on the
  user's machine** against any OpenAI-compatible endpoint. Routing in `app/src/ai/agent/api/local_route.rs`
  and `local_adapter.rs`; commit `92b7a7f9` adds a fully local mode (no Warp API, no login);
  `a7291716` generates next-command suggestions with the local model. 47 files touched; integration tests in
  `crates/integration/src/test/local_agent_loop.rs`.
- **Correction:** search results said a Warp maintainer endorsed it. The praising comment (2026-08-17) is from
  a community user, and the fork's author wrote (2026-06-17) that they don't want to maintain a fork.
- For Synth: the smallest existing implementation of exactly the gap left upstream.

## 4. oh-my-warp (ambiguous name)

**[AndrewWayne/oh-my-warp](https://github.com/AndrewWayne/oh-my-warp)** ("omw"; last commit 2026-09-29, release
`omw-local-preview-v0.0.15` on 2026-10-21)

- A Warp fork vendored at `vendor/warp-stripped/` plus a `pi-mono` agent submodule. GitHub shows the licence
  as NOASSERTION; the LICENSE text is AGPL-3.0.
- Features: own keys stored in the macOS Keychain / Windows Credential Manager; an inline `# <prompt>` agent
  with approvals; phone remote control of a pane over Tailscale (read-only unless
  `OMW_REMOTE_ALLOW_DEFAULT_WRITE=1`). **Synth already has its own self-hosted remote control**
  (`synthet/warp-remote`, `SYNTH_WARP_REMOTE_CONTROL_URL`).
- Practices: cloud surfaces compiled out behind an `omw_local` feature;
  [`specs/fork-strategy.md`](https://github.com/AndrewWayne/oh-my-warp/blob/main/specs/fork-strategy.md) syncs by
  `rsync`-copying upstream and re-applying changes; `scripts/audit-no-cloud.sh` runs `strings` on the built
  binary and fails on `app.warp.dev`, `api.warp.dev`, `oz.warp.dev`, Firebase and other cloud hosts;
  per-file AGPL headers enforced in CI.

**[hongvincent/oh-my-warp](https://github.com/hongvincent/oh-my-warp)** (MIT; last commit 2026-04-01): a
configuration kit for official Warp (skills, rules, MCP configs, workflows, themes). No client code.

## 5. Gram (Zed fork)

- [codeberg.org/GramEditor/gram](https://codeberg.org/GramEditor/gram). Hard fork; modifications GPLv3, the Zed
  base keeps its licences. Very active: last commit 2026-11-20, release `3.5.2` on 2026-11-14.
- Identity: binary crate renamed to `crates/gram`; Flatpak ID `app.liten.Gram`; desktop file
  `gram-editor.desktop`.
- Stripping: no AI, `collab`, `telemetry` or auto-update crates; no involuntary updates; fixed extension
  sandboxing.
- Upstream sync: selective ports, each tagged in `CHANGELOG.md` with its upstream number
  (e.g. "Reduce terminal flicker on resize (zed#47195)").
- Policy: does not accept AI-generated contributions and bans AI agents from the project. Borrowing a
  practice involves no code; copied code would stay GPLv3.

## 6. Zedless (Zed fork)

- [zedless-editor/zedless](https://github.com/zedless-editor/zedless), AGPL-3.0; last commit 2026-06-06; pins Zed
  `v1.4.4` in `source-info.json`. The earlier git fork `old-school-zedless` is archived.
- Instead of carrying a forked tree, `patch.py` applies syntax-aware [ast-grep](https://ast-grep.github.io/)
  rewrites to a pinned Zed tag, driven by lists in `config.py`: banned crates (`telemetry`, `collab`,
  `auto_update`, `crashes`, cloud model providers), banned modules/functions/structs/arguments (e.g.
  `send_telemetry`, `LlmApiToken`), banned actions (commit `4f755c10` removes account actions), and an
  identity text-replacement table (`dev.zed.Zed` → `org.zedless.Zedless`, "Welcome to Zed" → "Welcome to
  Zedless"). Rewrites repeat until nothing changes; icons come from an `overlay/`; builds use Nix.
- Policy: network features take only a base URL, have no default providers and are off by default.

## Pick table

| Candidate | Feature / practice | Source | Already upstream or in Synth? | Licence fit (AGPL-3.0) | Effort | Recommendation |
|---|---|---|---|---|---|---|
| man-brain | Agent loop on the user's machine via an OpenAI-compatible endpoint | `crates/local_agent`, `local_route.rs` | No (upstream configures endpoints but proxies them, #9619) | AGPL, compatible | L | needs owner (strategic; fork is 368 commits behind) |
| Zap | Native provider protocols (Anthropic, Gemini, Ollama) | zap-warp #305/#306 | No | AGPL, compatible | L | needs owner (archived; overlaps man-brain) |
| Zap | SSH host manager with tmux | `app/src/ssh_manager/` | Not in Synth | AGPL, compatible | M | consider |
| Zap | Fluent-based translations | `app/i18n/*/warp.ftl` | Not in Synth | AGPL, compatible | L | needs owner |
| Zap | Editable system prompts | `minijinja` | No | AGPL, compatible | M | skip until there is a local agent loop |
| omw | Scan the built binary for cloud hostnames | `audit-no-cloud.sh` | No | AGPL, compatible | S | consider (fits the epic's boundary guard) |
| omw | Phone remote control over Tailscale | README, `specs/` | Synth has its own self-hosted remote control | AGPL; `pi-mono` licence unchecked | — | skip |
| omw | Cloud surfaces compiled out behind a Cargo feature | `omw_local` | No (Synth gates at runtime by channel) | AGPL, compatible | M–L | consider as a pattern |
| terzigolu warp-lite | Per-sync audit with a JSON decision per upstream commit | `docs/upstream-sync/*.json` | n/a | n/a | S | consider (complements the epic's sync report) |
| terzigolu warp-lite | Feature boundary; `warpui` byte-identical to upstream | README, sync docs | No | AGPL, compatible | M | consider |
| AnxiangLemon warp-lite | CPU software rendering, backend choice, FPS cap | README, settings | Not in Synth | AGPL, compatible | M | consider (Windows) |
| AnxiangLemon warp-lite | Dependency slimming; vendored Git dependencies with hashes | `vendor/sources.json` | No | AGPL, compatible | M | needs owner |
| Gram | Tag each ported upstream fix with its upstream number | `CHANGELOG.md` | n/a | Practice only | S | consider |
| Zedless | Banned-symbol list and identity replacement table, checked by ast-grep | `config.py`, `patch.py` | n/a | Practice only | S as a lint, L as the build method | consider as a lint; skip as the build method |
| Upstream | Third-party harnesses over ACP | #16329, #16331 | Yes | AGPL | — | skip (arrives with upstream merges) |
| hongvincent oh-my-warp | Skills and rules configuration kit | README | n/a | MIT | S | skip |

## Could not verify

- Why Zap was archived, and whether it has a successor (a third-party "Zap v0.2026.10.21" release was not
  examined).
- Whether upstream has a client-side model harness not visible in PR titles or the crate list.
- The licences of omw's `pi-mono` and `forge-code` dependencies.
- Whether upstream stores BYOK keys in the OS keychain the way omw does.
- Whether terzigolu's removal claims match its default build output (the crates are still in the tree).
- Whether Zedless's patching still applies to current Zed (pinned to `v1.4.4`, last commit 2026-06-06).
- Any regression guard in Gram beyond its CI scripts.
