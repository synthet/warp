---
type: Report
title: Synth Term branding audit (OSS build), 2026-11-21
description: Where Warp branding still reaches Synth Term users after the About, Privacy and Warp Drive fixes; updates the 2026-11-19 audit.
resource: reports/synth-term-branding-audit-2026-11-21.md
tags: [docs, reports, branding, rename]
timestamp: 2026-11-21T18:30:00Z
okf_version: 0.1
---

# Synth Term branding audit (2026-11-21, update of 2026-11-19)

Baseline: master 59dd6f99; branch af/warp-ctke6-about-page-synth-term-mark-name (191ca150 About/logs/Account/Privacy, ecf63c0e Warp Drive hidden).

## Summary (visible or likely visible string lines)
App chrome/menus/palette/notifications ~160 (47 files); Settings ~110 (19); Onboarding 37 (~15 confirmed); AI/agents/terminal ~265 (74); TUI 25 (9); CLI/MCP 19 (12); Packaging ~40 (8); Warp-mark SVGs 20 files (8 marks + 12 dead loading frames). Hidden by gates ~243 lines (68 files).
Since 2026-11-19 master fixed only Windows deploy naming (1c91fc40).

## Top visible items
| file:line | string | where | suggestion |
|---|---|---|---|
| app/src/root_view.rs:118 | "Synth Warp" | window title, taskbar | "Synth Term" |
| app/channels/oss/dev.warp.WarpOss.desktop:7 | Name=WarpOss | Linux launcher | Synth Term |
| script/windows/bundle.ps1:160 -> windows-installer.iss:127-153 | APP_NAME 'WarpOss' | installer, Start menu, Explorer | Synth Term (keep AppId/mutex) |
| crates/onboarding/src/slides/intro_slide.rs:165; pane_group/pane/get_started_view.rs:226 | "Welcome to Synth Warp" | onboarding, Get Started tab | Synth Term |
| crates/onboarding/src/slides/customize_slide.rs:148 | "Customize your Warp" | onboarding | "Customize Synth Term" |
| crates/onboarding/src/slides/theme_picker_slide.rs:588 (+55) | "By continuing, you agree to Warp's " + ToS link | theme picker | remove on OSS |
| app/src/terminal/input.rs:454-473 | "Warp anything e.g. ..." (20) | agent input placeholder | "Ask anything, e.g. ..." |
| app/src/ai/blocklist/agent_view/zero_state_block.rs:429 | "New Warp Agent conversation" | every new agent conversation | "New agent conversation" |
| app/src/ai/blocklist/block/view_impl/common.rs:114; status_bar.rs:1165 | "Warping...", "Warping with another model." | agent loading/status | "Working...", "Retrying with another model." |
| app/src/terminal/view.rs:8087-8104, 845-854 | "Warp Agent needs your permission...", "Warp can notify you..." | notifications | "Agent needs...", "Synth Term can notify..." |
| app/src/terminal/view/inline_banner/open_in_warp.rs:51-82; view.rs:17651; tooltips.rs:61 | "View in Warp", "Edit in Warp", "Open in Warp" | file banners, context menu | Synth Term / "Open in editor" |
| plugin_manager/claude.rs:249,274 (+codex 276,326; gemini 126,138; opencode 35,55); agent_input_footer/mod.rs:521,546,1533 | "Install Warp Plugin for Claude Code" ... | third-party CLI agent setup | "Synth Term plugin" (keep warpdotdev/* marketplace ids) |
| plugin_manager/claude.rs:310,327 | "From your Synth Warp checkout..." | Windows hooks instructions | Synth Term |
| workspace/mod.rs:1097; quit_warning/mod.rs:544 | "Quit Warp", "Quit Warp?" | palette, quit dialog | Synth Term |
| settings_view/mod.rs:408; warpify_page.rs:166,517,639; workspace/mod.rs:1588-1589 | Warpify page and palette | settings, palette | "Shell integration" (keep slug) |
| terminal/view.rs:10564,10568; block_banner/warpify.rs:49; warpify_footer.rs:33,36 | "Warpify subshell" ... | banners | "Enable shell integration" |
| prompt_render_helper.rs:255-259; ssh_remote_server_*:18,78 | "Warp SSH Extension" | SSH prompt/banner | Synth Term SSH extension |
| settings_view/features_page.rs:4907,4909,738,5139,5150,5245,5987,7738 | "Start Warp at login", "Make Warp the default terminal", "Warp completions", "Restart Warp..." | Settings > Features | Synth Term / "Built-in completions" |
| settings_view/appearance_page.rs:1661,3761,251 | "Pin to the bottom (Warp mode)", radio "Warp", "Toggle Input Mode (Warp/Classic)" | Appearance > Input | "Modern" |
| settings_view/privacy_page.rs:85,1601 | "Help improve Warp", "Read more about Warp's use of data" | Privacy | Synth Term / gate link |
| agent_profiles_page.rs (6), knowledge_page.rs:237, mcp_servers/list_page.rs:73 | "...the Warp Agent should always ask..." | agent settings | "the agent" |
| warp_agent_page.rs:5449,5461 | "...will consume Warp credits.", "...not on Warp's servers." | Agent > API keys (policy bug) | drop / "never leave this device" |
| shell_terminated_banner.rs:219,257; onboarding_prompt_block.rs:243 | github.com/warpdotdev/Warp/issues | "File issue" buttons (policy bug) | feedback_form_url() |
| slash_command_menu/static_commands/commands.rs:175,195,245,297 | "Exit Warp", "Log out of Warp", "Open a file in Warp's code editor" | slash menu | neutral wording |
| crates/warp_tui/src/zero_state.rs:652; ui.rs:141,152; terminal_session_view.rs:5540; cli_agent_osc_event_publisher.rs:82-203 | "Warp Agent CLI", "Welcome to Warp", "Warping" | TUI zero state, notifications | Synth Term |
| crates/warp_tui/src/api_keys_menu.rs:797 | "Warp credit fallback" | TUI API keys (no hosted-AI gate, policy bug) | gate on warp_hosted_ai_enabled() |
| workspace/view/openwarp_launch_modal/view.rs:220,228 | "Warp is now open-source" + warpdotdev/oz links | one-time launch modal (default-on, unclear trigger) | strip OpenWarpLaunchModal on OSS |
| workspace/hoa_onboarding/welcome_banner.rs:108 | "...level up any coding agent with Warp" | HOA banner (unclear) | Synth Term |
| crates/onboarding/src/slides/offer_slide.rs:65,75,110,354; callout/view.rs:62,122,139 | "Use Warp with AI", "Get Warping", "Meet the Warp input" | onboarding end, callout | Synth Term / "Get started" |
| crates/mcp/src/oauth/loopback.rs:128,132 | "...You can return to Warp." | browser page after MCP OAuth | Synth Term |
| workspace/mod.rs:1573; app_menus.rs:227,207,914,54-55; appearance_page.rs:3082,3122 | macOS: "About Synth Warp", Menu "Warp", "Show Warp in Dock" | macOS only | Synth Term |
| windows-installer.iss:19,144,67-68; debian control.template; rpm warp.spec.template; script/macos/bundle:432-433,965 | "Synth Warp contributors", "Add Warp to PATH", maintainers @warp.dev, WarpOss | packaging | Synth Term |
| app/assets/bundled/svg/warp-drive.svg, warp.svg, warp-2.svg, warp-3.svg, warp-logo-{light,dark,neutral}.svg | Warp two-panel mark | Icon::Warp/Agent/Oz, intro slide, Get Started, TUI logo source | redraw from synth-term masters (Icon::Warp/WarpDrive files are swapped, icons.rs:418-419) |

Keep: About "Based on Warp..." line, warpdotdev/* plugin marketplace ids, settings slugs (settings_view/mod.rs:407-457), AppUserModelID io.github.synthet.Warp, citation chip "Warp Documentation".

## Open policy bugs
docs.warp.dev links still not routed through warp_inc_links_enabled() (only 5 files call it); intro_slide.rs:86 "Already have an account? Log in" unconditional; telemetry_banner.rs:21 (hidden); TUI credit fallback ungated.
