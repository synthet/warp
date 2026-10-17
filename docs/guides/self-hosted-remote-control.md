---
type: Guide
title: Self-hosted remote control
description: Run synth-remote-control-server and control Synth Warp terminals from a phone or browser, replacing Warp's hosted session sharing.
resource: guides/self-hosted-remote-control.md
tags: [docs, guides, remote-control, session-sharing, self-hosted]
timestamp: 2026-10-17T00:00:00Z
okf_version: 0.1
---

# Self-hosted remote control

Watch and drive your Synth Warp terminals — including a CLI agent waiting for approval — from a
phone or any browser. You run the server; no Warp account or warp.dev service is involved.

Design: [specs/self-hosted-remote-control](../../specs/self-hosted-remote-control/PRODUCT.md).

## 1. Run the server

```bash
cargo build --release -p remote_control_server
export SYNTH_RC_TOKEN="$(openssl rand -hex 32)"   # keep this; terminals and the web app need it
./target/release/synth-remote-control-server --bind 0.0.0.0:8787
```

If no token is set, the server generates one and prints it at startup (valid for that run only).

| Flag / env | Default | Meaning |
|------------|---------|---------|
| `--bind` / `SYNTH_RC_BIND` | `127.0.0.1:8787` | Listen address |
| `--token` / `SYNTH_RC_TOKEN` | generated | Shared secret |
| `--token-file` / `SYNTH_RC_TOKEN_FILE` | — | Read the secret from a file |
| `--owner-name` / `SYNTH_RC_OWNER_NAME` | `Owner` | Name in participant lists |
| `--viewer-role` / `SYNTH_RC_VIEWER_ROLE` | `full` | `reader` makes the server watch-only |
| `--max-session-mib` / `SYNTH_RC_MAX_SESSION_MIB` | `256` | Per-terminal memory cap |
| `--reconnect-grace-minutes` | `10` | Keep a session this long while its terminal is offline |

### TLS

Plain `ws://`/`http://` is only for a trusted LAN or a tailnet. Anywhere else, terminate TLS in
front of the server, which also lets the web app install as a PWA. Caddy:

```caddyfile
rc.example.com {
    reverse_proxy 127.0.0.1:8787
}
```

Tailscale: `tailscale serve --bg 8787` and use the `https://<machine>.<tailnet>.ts.net` name.

## 2. Point Synth Warp at it

Set these before launching Synth Warp (`warp-oss`) on each machine whose terminals you want to reach:

```bash
export SYNTH_WARP_REMOTE_CONTROL_URL=wss://rc.example.com      # ws://host:8787 on a LAN
export SYNTH_WARP_REMOTE_CONTROL_TOKEN=<the server token>
export SYNTH_WARP_REMOTE_CONTROL_ALL_TERMINALS=1              # optional: share every terminal
export SYNTH_WARP_REMOTE_CONTROL_HOST_LABEL="work laptop"     # optional: dashboard name
```

On Windows use `setx` (or System Properties → Environment Variables) and restart Warp.

A URL on a `warp.dev` host is refused: this never re-enables Warp's paid service.

### Sharing a terminal

- **One terminal**: pane menu → *Share session*, the `/remote-control` slash command, or the
  *Remote control* chip in an agent footer. The web link is copied to the clipboard.
- **Every terminal**: with `SYNTH_WARP_REMOTE_CONTROL_ALL_TERMINALS=1`, each new terminal shares
  itself once it starts. Links are not copied; open the dashboard instead.
- *Stop sharing* works as usual. Self-hosted shares are not ended by desktop inactivity.

## 3. Use the web app

Open `https://rc.example.com`, sign in with the token, and pick a terminal.

- **Idle prompt**: type in the command bar (or the terminal) and press *Run*.
- **Running program** (shell job, `vim`, Claude Code, …): typing goes straight to it; the key bar
  sends Esc, Tab, ^C, ^D, arrows, and Enter.
- When the phone is the only viewer, the terminal resizes to fit it.
- *Add to Home Screen* installs it as an app.

## Troubleshooting

| Symptom | Check |
|---------|-------|
| No sharing entries in Synth Warp | `SYNTH_WARP_REMOTE_CONTROL_URL` set in the environment Warp was launched from; it must start with `ws://` or `wss://` |
| "Failed to share" | Server reachable from that machine; `SYNTH_WARP_REMOTE_CONTROL_TOKEN` matches (`warp-oss.log` warns when it is unset) |
| Web app keeps asking for the token | The cookie is `SameSite=Strict`; open the server's own URL, not through another site |
| Terminal stopped appearing after a while | A share ends at 100 MB of output (client cap) or when the server restarts; share it again |
| Commands rejected as busy | A command is already running; use the key bar / typing instead |
