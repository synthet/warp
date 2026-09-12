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

## Guard status

**None** of the 111 sites has a GraphQL-layer `Disable`-trait stub. Item #10's "Phase 1 stubs Disable
traits with `anyhow!(\"disabled in Synth Warp\")`" is **not yet implemented**.

## Recommendation

Add one guard at the chokepoint `graphql_helpers::send_graphql_request`, before the request is built:

```rust
if !warp_core::channel::ChannelState::warp_cloud_enabled() {
    anyhow::bail!("disabled in Synth Warp: GraphQL is offline on this build");
}
```

`warp_cloud_enabled()` is precisely the right predicate: `false` for the OSS blackhole root **and**
for any hosted-warp-production root (which the fork must never re-enable), but `true` when a user
points the client at their own backend via `SYNTH_WARP_SERVER_ROOT_URL` — so self-hosting still
works. This covers all 111 sites, needs no `GraphQLError` change, and is the defense-in-depth guard
`docs/architecture/synth-fork.md` calls for.

**Landing constraint (must be handled in the same change):** the 6 unit tests in
`crates/warp_server_client/src/graphql_helpers_tests.rs` drive `send_graphql_request` under the
default `Oss` + disabled root with a `FakeGraphqlOperation`, asserting it proceeds to send. The guard
would short-circuit them. They must be updated to run under a cloud-enabled state — but flipping the
global `CHANNEL_STATE` singleton in a test is the same process-wide-state hazard documented in
backlog item #21, so the fix needs a scoped test-state override (or a `#[cfg(test)]` seam), not a
bare mutation that leaks across parallel tests. This is why the guard is left as a recommendation
rather than applied here.

## Refresh commands

```bash
# per-domain counts under server_api
rg -n 'send_graphql_request\(' app/src/server/server_api --glob '!*_test*' | \
  sed 's|:.*||' | sort | uniq -c | sort -rn
# all application-level invocation sites
rg -n '\.send_graphql_request|send_graphql_request\(' app crates --glob '!*_test*' \
  | rg -v 'fn send_graphql_request'
```
