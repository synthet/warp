# Synth Warp FAQ

Synth Warp is a commercial-free, local-first fork of the Warp client.

## Can I use Warp’s paid cloud features?

No. Billing, credits, Warp Drive cloud sync, and hosted Warp AI are removed or disabled in this fork. Warp’s server is not part of this repository.

## Do I need a Warp account?

No. Local terminal use does not require signing in.

## What about AI agents?

Use external CLI agents (Claude Code, Codex, Gemini CLI, etc.) or BYOK/custom inference where the client supports them. Hosted Warp-credit agents are not available here.

## Licensing

Same as upstream: AGPL v3 for the app; MIT for `warpui` / `warpui_core`. See [LICENSE-AGPL](LICENSE-AGPL) and [LICENSE-MIT](LICENSE-MIT).

## Can I use this with Zed?

Yes, as a separate editor. Warp can open file links in an external editor; Zed tasks can open Synth Warp tabs via `warposs://` URLs. A deeper session bridge is planned but not implemented. See the [Zed × Warp wiki](docs/zed-warp/README.md).

## Where are build instructions?

See [README.md](README.md) and [AGENTS.md](AGENTS.md). Design notes live under [docs/](docs/README.md).
