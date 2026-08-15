---
title: Synth forks — Zed × Warp Hybrid
description: Synth Warp and Synth Zed are commercial-free / local-first. No Drive, no hosted Warp AI. CLI agents only.
---

# Synth forks

← [Zed × Warp](../README.md)

Both checkouts are commercial-free, local-first forks. The hybrid must not depend on surfaces the Warp fork removed.

Warp user-facing summary: `(Warp) FAQ.md`, `README.md`.

## Removed or disabled in Synth Warp

- Billing, credits, upgrades, referrals
- Warp Drive cloud sync
- Hosted Warp-credit AI / cloud Oz as a required backend

Warp’s server, Drive backend, and Oz orchestration are **not** in the Warp repository.

## What that means for the hybrid

| Tempting dependency | Reality |
|---------------------|---------|
| Cloud/ambient tasks as session list | Empty or degraded without Warp servers |
| Drive-backed shared objects as transport | Not available |
| Credits / usage APIs | Irrelevant |
| `AgentConversationsModel` cloud fields | Will not populate usefully |
| CLI agents + OSC 777 plugins | **Durable dual-app source of truth** |
| BYOK / custom inference | Fine where the client still supports it |

`TabConfigPaneType::Cloud` exists in the schema; do not require it for Zed layouts. Prefer `terminal` and `agent` (CLI) panes.

Synth Warp: app id `io.github.synthet.Warp`, URL scheme **`warposs`**, default-run binary `warp-oss`.

Synth Zed: keep upstream GPL; deep links `synthzed://…`. Do not assume a license change.

## See also

- [Overview](../overview.md)
- [Licenses](../licenses.md)
- [CLI agent sessions](../surfaces/cli-agent-sessions.md)
