# Synth Warp

**Synth Warp** is a commercial-free, local-first fork of [Warp](https://github.com/warpdotdev/warp)’s open-source client.

It keeps the terminal and local tooling, and removes billing, upgrades, credits, referrals, Warp Drive cloud sync, and hosted Warp AI/credit flows. Use external CLI agents (Claude Code, Codex, Gemini CLI, and others) or BYOK where the client supports them.

## What this fork is

- Local-first terminal / agentic IDE client
- No Stripe billing, plan upgrades, credit packs, or referral UI
- No Warp Drive cloud sync or hosted Warp-credit AI surfaces
- Runs without a Warp account for local use
- Licensed under the same terms as upstream: **AGPL v3** for the app, **MIT** for `warpui` / `warpui_core`

## What this fork is not

Warp’s server, Drive backend, and hosted agent inference are **not** in this repository and are not recreated here. Synth Warp does not unlock paid Warp cloud services against Warp’s servers.

## Building

```bash
./script/bootstrap   # platform-specific setup
./script/run         # build and run Synth Warp
./script/presubmit   # fmt, clippy, and tests
```

See [AGENTS.md](AGENTS.md) for the engineering guide and [docs/](docs/README.md) for design notes, including the [Zed × Warp hybrid](docs/zed-warp/README.md).

## Licensing

- UI framework crates (`warpui_core`, `warpui`): [MIT](LICENSE-MIT)
- Everything else in this repository: [AGPL v3](LICENSE-AGPL)

Distributing modified builds requires complying with AGPL (including source availability for network-used modifications).

## Upstream

Forked from [synthet/warp](https://github.com/synthet/warp) / [warpdotdev/warp](https://github.com/warpdotdev/warp).
