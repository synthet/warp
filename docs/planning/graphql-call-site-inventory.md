# `send_graphql_request` call-site inventory (backlog item #10)

Follow-up inventory for [conversation-backlog.md](conversation-backlog.md) item **#10 — Quiet
leftover GraphQL to `192.0.2.0:9`**. Generated 2026-09-12 by reading the tree; re-run the greps
below to refresh.

## Why this matters

On OSS the shipped `ChannelState::init()` is `Channel::Oss` + `WarpServerConfig::disabled()`, whose
server root is the TEST-NET-1 blackhole `http://192.0.2.0:9`
([`channel/config.rs`](../../crates/warp_core/src/channel/config.rs), `DISABLED_HTTP_ROOT`). The
low-level transport [`crates/graphql/src/client.rs`](../../crates/graphql/src/client.rs)
`send_graphql_request` builds its endpoint from `ChannelState::server_root_url()` and executes it
with **no short-circuit** for the disabled root. So any GraphQL operation that actually fires on OSS
dials the sinkhole and waits for the connection to fail. Nothing today stubs this at the GraphQL
layer — OSS is protected only by caller-level gating (feature flags / `warp_cloud_enabled()` checks
in the code that *decides* to call an API method). Item #10 asks for defense-in-depth so OSS never
even attempts the sinkhole.

## The chokepoint

Every application-level call funnels through one function:

- **`crates/warp_server_client/src/graphql_helpers.rs::send_graphql_request`** — returns
  `anyhow::Result<QF>`, and is called by the `ServerApi::send_graphql_request` wrapper
  ([`app/src/server/server_api.rs:553`](../../app/src/server/server_api.rs)) and directly by the
  auth module. Guarding here needs **no** `GraphQLError` enum change (it already `anyhow::bail!`s),
  and one guard covers all 111 invocation sites below.

The universal fallback is the transport in `crates/graphql` (covers any future non-helper caller),
but it returns `GraphQLError`, so a guard there needs a new enum variant and updates to the ~5
exhaustive matches (`is_actionable`, `retry_strategies.rs`, `sync_queue.rs`, `graphql_helpers.rs`).

## Inventory (111 application-level invocation sites)

All counts exclude `_test` files and the three `fn send_graphql_request` definitions.

### `app/src/server/server_api/` — 102, via `self.send_graphql_request(...)`

| Domain file | Sites | Hosted surface it talks to |
|-------------|------:|----------------------------|
| `object.rs` | 29 | Warp Drive objects / cloud sync |
| `ai.rs` | 27 | hosted AI / Warp-credit inference |
| `team.rs` | 16 | Teams |
| `managed_secrets.rs` | 8 | hosted secret vault |
| `integrations.rs` | 7 | third-party integrations |
| `workspace.rs` | 5 | workspace / billing-adjacent |
| `factory.rs` | 3 | factory/agents |
| `block.rs` | 3 | shared blocks |
| `referral.rs` | 2 | referral program |
| `tui_onboarding.rs` | 1 | TUI onboarding |
| `managed_mcp.rs` | 1 | managed MCP catalog |

### `crates/warp_server_client/src/auth/mod.rs` — 7

Lines 194, 279, 329, 390, 422, 448, 457. User settings / account / token operations. Call
`crate::graphql_helpers::send_graphql_request` directly.

### `app/src/ai/agent_sdk/environment.rs` — 2

Lines 150, 350 (`server_api.send_graphql_request`). Cloud-agent environment image fetches.

## Guard status — implemented 2026-09-12

One guard now sits at the chokepoint `graphql_helpers::send_graphql_request`, before the request is
built, covering all 111 sites:

```rust
if warp_core::channel::is_disabled_root_url(
    warp_core::channel::ChannelState::server_root_url().as_ref(),
) {
    anyhow::bail!("disabled in Synth Warp: GraphQL server is offline on this build");
}
```

### Why this predicate

The guard checks `is_disabled_root_url(server_root_url())` — the **same** accessor the transport
(`crates/graphql/src/client.rs`) uses to build the endpoint, so it tests the actual outgoing target.
It is `true` only for the `192.0.2.0:9` blackhole, and a self-hoster who sets a real
`SYNTH_WARP_SERVER_ROOT_URL` passes through. No `GraphQLError` enum change (the chokepoint returns
`anyhow`).

This turned out to need **zero test changes**, avoiding the item-#21 `CHANNEL_STATE`-mutation hazard
entirely: `warp_server_client`'s dev-deps build `warp_core` with `test-util`, under which
`server_root_url()` returns the mockito localhost URL (not the blackhole), so the guard is skipped in
tests. (An earlier sketch gated on `warp_cloud_enabled()`, which reads the config root directly and
*would* have tripped the 6 `graphql_helpers_tests.rs` tests — switching to the transport-facing
`server_root_url()` both fixed that and made the check more precise.)

### Verified

`cargo test -p warp_server_client --lib` → 49 passed, 0 failed (incl. the 6 `graphql_helpers` tests);
`cargo clippy -p warp_server_client --lib` clean; `cargo fmt -- --check` clean.

### Residual

The guard is at the app-level chokepoint, which every current call path funnels through. A
hypothetical *future* caller that reaches the low-level `crates/graphql` transport directly (not via
`graphql_helpers`) would bypass it; a transport-level guard there would need a new `GraphQLError`
variant and updates to ~5 exhaustive matches. Not needed today — left as a note.

## Refresh commands

```bash
# per-domain counts under server_api
rg -n 'send_graphql_request\(' app/src/server/server_api --glob '!*_test*' | \
  sed 's|:.*||' | sort | uniq -c | sort -rn
# all application-level invocation sites
rg -n '\.send_graphql_request|send_graphql_request\(' app crates --glob '!*_test*' \
  | rg -v 'fn send_graphql_request'
```
