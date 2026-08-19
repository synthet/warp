---
type: Guide
title: Claude Code Warp hooks on Windows
description: Install PowerShell notification hooks for Claude Code in Warp on Windows; avoid the broken bash plugin.
resource: guides/claude-code-warp-windows-hooks.md
tags: [docs, guides, windows, claude-code, hooks]
timestamp: 2026-08-17T00:00:00Z
okf_version: 0.1
---

# Claude Code Warp hooks on Windows

## Problem

The official `warp@claude-code-warp` Claude Code plugin registers hooks as bare `.sh` script paths. On Windows, PowerShell and Claude Code cannot execute those in-process; the path falls through to **ShellExecute** and the `.sh` file association (often Git Bash, an editor, or Warp itself). The result looks like random script files opening on every tool call.

**Do not install `warp@claude-code-warp` on Windows.**

## Fix (Synth Warp checkout)

From the repo root, run:

```powershell
.\scripts\install_claude_warp_hooks_windows.ps1
```

This script:

1. Copies [`scripts/claude-warp-hooks/`](../../scripts/claude-warp-hooks/) into `%USERPROFILE%\.claude\hooks\warp\`
2. Merges a `hooks` block into `%USERPROFILE%\.claude\settings.json` using `pwsh -NoProfile -File` commands
3. Warns if `warp@claude-code-warp` is still listed in `installed_plugins.json`

Then **restart Claude Code** (exit and start `claude` again).

## Verify

1. `claude plugin list` — should **not** show an enabled `warp@claude-code-warp` entry.
2. Run a tool in Claude Code inside Warp.
3. Hook logs should reference `.../hooks/warp/on-post-tool-use.ps1`, not `.sh` under `plugins/cache/claude-code-warp`.

If the bash plugin was previously installed:

```powershell
claude plugin uninstall warp@claude-code-warp
```

Optional cleanup of stale cache:

```powershell
Remove-Item -Recurse -Force "$env:USERPROFILE\.claude\plugins\cache\claude-code-warp" -ErrorAction SilentlyContinue
```

## How Warp treats this

On Windows, Warp **does not** auto-install the bash notification plugin (`can_auto_install` is false). The agent footer treats PowerShell hooks in `~/.claude/hooks/warp/` plus a matching `settings.json` entry as “installed.” Use the install chip’s manual instructions if you need to re-run the installer.

## macOS / Linux

Use the normal `warp@claude-code-warp` plugin path (`claude plugin install warp@claude-code-warp`). Auto-install from Warp applies there.

## Related

- [build-and-run.md](build-and-run.md) — Windows toolchain and compile notes
- [TROUBLESHOOTING.md](../TROUBLESHOOTING.md) — general Windows issues
- `.claude/rules/windows-script-invocation.md` — why bare `.sh` paths fail on Windows
