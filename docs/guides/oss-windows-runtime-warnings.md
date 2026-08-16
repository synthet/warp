---
type: Runbook
title: OSS Windows runtime warnings
description: Expected warp-oss.exe [WARN] lines on a logged-out Windows GUI launch — not launch failures.
resource: guides/oss-windows-runtime-warnings.md
tags: [docs, guides, windows, oss, logging]
timestamp: 2026-08-16T01:20:00Z
okf_version: 0.1
---

# OSS Windows runtime warnings

When `target\release\warp-oss.exe` (or debug) starts **logged out** on Windows, the console fills with `[WARN]` lines. Those are almost all expected. Treat a successful PowerShell bootstrap and a usable window as the health signal, not a clean log.

Snapshot of a 2026-08-15 session: [reports/oss-windows-startup-warnings-2026-08-15.md](../reports/oss-windows-startup-warnings-2026-08-15.md). Fork boundary: [architecture/synth-fork.md](../architecture/synth-fork.md).

## Do not chase

| Log line | Why it fires |
|----------|----------------|
| `SQLite error 283` WAL recovered | Previous run left `warp.sqlite-wal` under the OSS AppId data dir. SQLite replayed frames and continued. |
| `No distribution matched the default guid` | WSL `Lxss\DefaultDistribution` GUID does not match an enumerated distro (none installed, uninstalled default, or only Docker/Rancher). Local `pwsh` still starts. Code: `app/src/terminal/wsl/model.rs`. |
| `Deprioritizing non DX12 Nvidia adapter due to version > 572` | NVIDIA **Vulkan** on drivers newer than 572 is ranked down. NVIDIA **DX12** stays available. Code: `crates/warpui/src/rendering/wgpu/resources.rs`. |
| `HandleFocusChange … no view handled it` | Terminal focuses before `PaneGroup` is on the responder chain. Later focus changes succeed. |
| `Tried to update block filter query without active_filter_editor_block_index` | Filter update with no active filter editor. No UI effect. |
| `Environment variable "=C:" was invalid` | Windows drive-cwd vars (`=C:`, `=D:`, …). Names cannot contain `=`; Warp skips them. Code: `app/src/terminal/local_tty/windows/environment.rs`. |
| `Tried to render share modal without a model` | Share modal is constructed empty. `AppContext::render_views` calls `render()` on **every** GUI view in the window, including closed overlays. |
| `No alert dialog was set for the native modal` | Same all-views render path for the unused native quit dialog. |
| `SuggestedAgentModeWorkflowModal has not been initialized` | Same; modal is `Default` until opened. |
| `GitDiffStats` / `KubernetesContext` could not prepare execution context | Prompt chips run before the shell session/cwd is ready. |
| `Unable to create cloud preferences due to unset personal drive` | No Warp Drive / not logged in. Expected OSS. |
| `Not handling MouseMoved … back-to-back synthetic events` | Hoverable loop guard; one hover callback skipped. |

Related **INFO** (not WARN):

- `Unable to read user from secure storage: NotFound` — logged out. Expected.
- `server_root_url: "http://192.0.2.0:9"` — OSS sinkhole (`WarpServerConfig::disabled()`, TEST-NET-1 + discard port). Cloud AI, auth, Drive, and Privacy “data management” links will not reach Warp production. Local terminal still works.

## GPU: Intel iGPU is the Windows default

On Windows/Linux, `system.prefer_low_power_gpu` defaults **on**, so Intel UHD DX12 outranks NVIDIA DX12. The NVIDIA warning only demotes Vulkan. To use a discrete GPU, turn off “Prefer low-power GPU” in Settings. Setting: `app/src/settings/gpu.rs`.

## Worth fixing (log hygiene only)

Nothing in that pattern is a user-facing bug. The only cheap follow-up is quieter empty-overlay `render()`: share modal, native modal, and suggested-workflow modal should not `log::warn` in their expected empty state. Same idea, lower value: demote first-frame `HandleFocusChange` and `=C:` to `debug`.

Do not “fix” WSL GUID, WAL 283 after a fast quit, NVIDIA Vulkan ranking, Git/K8s chips before bootstrap, or cloud preferences without a personal drive.
