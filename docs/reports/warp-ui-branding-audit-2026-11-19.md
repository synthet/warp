---
type: Report
title: Warp UI branding audit (OSS build)
description: Where the Warp brand still reaches Synth Warp/Synth Term users, what is gated, and the fix plan.
resource: reports/warp-ui-branding-audit-2026-11-19.md
tags: [docs, reports, branding, rename]
timestamp: 2026-11-19T23:59:00Z
okf_version: 0.1
---

# Warp UI branding audit (OSS build), 2026-11-19

Read-only sweep of the GUI, onboarding, settings, AI/terminal content and the TUI, done before the
Synth Term rename ([synth-fork.md](../architecture/synth-fork.md)). Scope: user-visible strings that
name Warp (including feature names such as Warp Drive, Warp Agent, Warpify), Warp logo assets, and
warp.dev / docs.warp.dev / github.com/warpdotdev links. Identifiers, comments, logs, telemetry names
and tests were ignored. Visibility was judged for `Channel::Oss` with default Cargo features.

## Totals

| Area | Visible in OSS | Hidden | Unclear |
|------|---------------:|-------:|--------:|
| App chrome and system integration | 40 rows | 13 | 5 |
| Onboarding, auth and settings | 50 rows | 43 | 8 |
| AI, agents, Drive, terminal content, TUI | ~52 rows | ~30 | ~17 |

Rows group related strings, so the string count is higher (about 220 Rust files contain a
user-facing "Warp" literal; about 160 lines link to warp.dev).

## Gating facts the verdicts rest on

- OSS: `shows_warp_inc_links()` and `offers_warp_hosted_ai()` are false (`crates/warp_core/src/channel/mod.rs`).
- `autoupdate_config: None` (`app/src/bin/oss.rs`): every update banner, pill and menu item is unreachable.
- `warp_cloud_enabled()` is false: no sign-in, no changelog fetch; Drive is off for logged-out users (`drive/settings.rs`).
- `filter_unsupported_features_for` strips cloud-agent, Warp-hosted-AI and control-plane flags (`channel/state.rs`).
- AI is not login-gated (`settings/ai.rs`): the built-in agent UI renders; requests fail unless `SYNTH_WARP_SERVER_ROOT_URL` names a backend.
- Billing, Referrals, Teams, Warp Drive, Environments and API-key settings pages are removed (`is_hidden_in_local_only_build`).
- The macOS menu bar only exists on macOS (`app/src/lib.rs`).

## Highest-impact visible items

1. **Product name everywhere:** window title (`root_view.rs` WINDOW_TITLE), About page name/copyright,
   onboarding "Welcome to Synth Warp" / "Customize your Warp", bundle names (`app/Cargo.toml`,
   `dev.warp.WarpOss.desktop`, `script/windows/bundle.ps1` `WarpOss`, `script/macos/bundle`).
2. **Logo assets are still Warp's two-panel mark:** `warp-drive.svg` (via `Icon::Warp` and `Icon::Agent`:
   zero-state block of every new session, agent surfaces), `warp-2.svg`/`warp-3.svg` (slash menu,
   `Icon::Oz`), `warp-logo-light/dark/neutral.svg` (onboarding, Get Started tab, bundled skills),
   `warp-logo-with-{light,dark}-title.svg` (About page), `warp.svg` (Drive toolbelt). The TUI zero
   state samples `warp-logo-light.svg`. Note `Icon::Warp` and `Icon::WarpDrive` map to each other's files.
3. **Agent input placeholder** "Warp anything e.g. …" (`terminal/input.rs`), "New Warp Agent conversation",
   "Warping…" loading text, "Warp Agent needs your permission…" notifications.
4. **"Open in Warp" banners and menus** after `cat`/`vim` of a file (`inline_banner/open_in_warp.rs`, `view.rs`).
5. **Third-party CLI agent plugin prompts** ("Install the Warp plugin…", "Install Warp Plugin for Claude
   Code/Codex/Gemini/OpenCode") — the main OSS AI path; `claude.rs` still says "Synth Warp checkout".
6. **Warpify / SSH extension copy and the "Warpify" settings page.**
7. **Quit/lifecycle strings:** "Quit Warp?", "Quit/Hide Warp" bindings, "View Warp logs", "Start Warp at
   login", "Make Warp the default terminal", notification-permission errors.
8. **TUI zero state:** "Warp Agent CLI", "Welcome to Warp", built-in Warp logo animation.

## Policy bugs found (fix regardless of the rename)

- Hardcoded `github.com/warpdotdev/Warp/issues/new/choose` "File issue" buttons in `shell_terminated_banner.rs`
  and `onboarding_prompt_block.rs` bypass `feedback_form_url()`.
- About 50 docs.warp.dev "Learn more"/feedback links are not routed through `warp_inc_links_enabled()`
  (agent tips ×31, settings pages ×19, banners), contrary to `util/links.rs`.
- Warp Drive surfaces show although Drive is unavailable: left-panel toolbelt tab + "Warp Drive is not
  available" panel (`compute_left_panel_views` checks only `enable_warp_drive`), Appearance "Warp Drive"
  toggle, agent tips advertising Drive, "Auto-sync plans to Warp Drive" in profiles, onboarding Customize chip.
- Welcome slide "Already have an account? Log in" opens a disabled server URL (gate on `warp_cloud_enabled()`).
- Agent → Custom Inference copy promises "will consume Warp credits" / "not on Warp's servers" on OSS.
- `telemetry_banner.rs` hardcodes warp.dev/privacy (hidden today).
- TUI API-keys menu shows "Warp credit fallback" with no `offers_warp_hosted_ai()` gate.

## Fix plan (feeds the Synth Term rename tickets)

1. **Assets (one change, dozens of call sites):** redraw `warp-drive.svg`, `warp-2.svg`, `warp-3.svg`,
   `warp-logo-{light,dark,neutral}.svg`, `warp-logo-with-{light,dark}-title.svg` and `warp.svg` from the
   Synth Term masters (`assets/branding/synth-term/`); TUI ASCII mark; delete the dead `warp-loading-*` frames.
2. **Product-name strings:** "Warp"/"Synth Warp" → "Synth Term" in chrome, lifecycle, onboarding, About,
   settings and notifications. Feature names: Warp Agent → "Agent", Warpify → "Shell integration",
   "Warp" input type → "Modern", Warp plugin → "Synth Term plugin" (keep upstream marketplace ids).
3. **Links:** route every docs.warp.dev link through one helper that hides it (or retargets to fork docs)
   when `!warp_inc_links_enabled()`; remove the hardcoded issue links.
4. **Drive:** gate every Drive surface on Drive availability, not on the default-on setting.
5. **Keep as factual upstream references:** "fork of Warp" attribution, upstream plugin marketplace ids,
   agent citation chips labelled "Warp docs", settings slugs used by warpctrl/session restore.
