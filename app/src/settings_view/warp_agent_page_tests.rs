use warp_core::channel::ChannelState;

use super::{
    AgentAttributionToggleState, GrokSubscriptionButtonAction, byok_backend_available,
    custom_inference_description_paragraphs, derive_agent_attribution_toggle_state,
    grok_subscription_button_action, provider_keys_editable,
};
use crate::workspaces::workspace::AdminEnablementSetting;

#[test]
fn respect_user_setting_returns_user_pref_unlocked() {
    let state = derive_agent_attribution_toggle_state(
        &AdminEnablementSetting::RespectUserSetting,
        true,
        true,
    );
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: true,
            is_forced_by_org: false,
            is_disabled: false,
        }
    );
}

#[test]
fn respect_user_setting_with_user_off_returns_unchecked_unlocked() {
    let state = derive_agent_attribution_toggle_state(
        &AdminEnablementSetting::RespectUserSetting,
        false,
        true,
    );
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: false,
            is_forced_by_org: false,
            is_disabled: false,
        }
    );
}

#[test]
fn team_enable_locks_toggle_on_regardless_of_user_pref() {
    let state = derive_agent_attribution_toggle_state(&AdminEnablementSetting::Enable, false, true);
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: true,
            is_forced_by_org: true,
            is_disabled: true,
        }
    );
}

#[test]
fn team_disable_locks_toggle_off_regardless_of_user_pref() {
    let state = derive_agent_attribution_toggle_state(&AdminEnablementSetting::Disable, true, true);
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: false,
            is_forced_by_org: true,
            is_disabled: true,
        }
    );
}

#[test]
fn ai_globally_disabled_marks_toggle_disabled_but_not_forced() {
    let state = derive_agent_attribution_toggle_state(
        &AdminEnablementSetting::RespectUserSetting,
        true,
        false,
    );
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: true,
            is_forced_by_org: false,
            is_disabled: true,
        }
    );
}

#[test]
fn team_force_takes_precedence_over_global_ai_disabled() {
    let state =
        derive_agent_attribution_toggle_state(&AdminEnablementSetting::Enable, false, false);
    assert_eq!(
        state,
        AgentAttributionToggleState {
            is_enabled: true,
            is_forced_by_org: true,
            is_disabled: true,
        }
    );
}

#[test]
fn grok_button_action_reflects_tokens_and_attempt_phase() {
    assert_eq!(
        grok_subscription_button_action(false, None),
        GrokSubscriptionButtonAction::Connect
    );
    assert_eq!(
        grok_subscription_button_action(false, Some(false)),
        GrokSubscriptionButtonAction::Cancel
    );
    assert_eq!(
        grok_subscription_button_action(false, Some(true)),
        GrokSubscriptionButtonAction::Cancelling
    );
    // Stored tokens take precedence regardless of attempt phase.
    assert_eq!(
        grok_subscription_button_action(true, None),
        GrokSubscriptionButtonAction::Disconnect
    );
    assert_eq!(
        grok_subscription_button_action(true, Some(false)),
        GrokSubscriptionButtonAction::Disconnect
    );
    assert_eq!(
        grok_subscription_button_action(true, Some(true)),
        GrokSubscriptionButtonAction::Disconnect
    );
}

// Synth Warp is local-first: the Custom Inference copy and key fields must not
// promise Warp credits or Warp's servers, and must say a backend is required.

#[test]
fn oss_custom_inference_copy_never_mentions_warp_credits_or_servers() {
    for backend_available in [false, true] {
        let text = custom_inference_description_paragraphs(true, true, false, backend_available)
            .join("\n");
        assert!(!text.contains("Warp credits"), "{text}");
        assert!(!text.contains("Warp's servers"), "{text}");
        assert!(text.contains("self-hosted backend"), "{text}");
    }
}

#[test]
fn oss_custom_inference_copy_states_the_backend_prerequisite_only_without_a_backend() {
    let without = custom_inference_description_paragraphs(true, false, false, false).join("\n");
    assert!(without.contains("SYNTH_WARP_SERVER_ROOT_URL"), "{without}");
    assert!(without.contains("not used"), "{without}");

    let with = custom_inference_description_paragraphs(true, false, false, true).join("\n");
    assert!(!with.contains("not used"), "{with}");
}

#[test]
fn warp_hosted_channels_keep_the_upstream_copy() {
    let text = custom_inference_description_paragraphs(true, true, true, true).join("\n");
    assert!(text.contains("Warp credits"), "{text}");
}

#[test]
fn no_description_when_no_custom_inference_content_is_shown() {
    assert!(custom_inference_description_paragraphs(false, false, false, false).is_empty());
}

#[test]
fn provider_key_fields_require_a_backend() {
    assert!(!provider_keys_editable(true, true, true, false));
    assert!(provider_keys_editable(true, true, true, true));
    assert!(!provider_keys_editable(false, true, true, true));
}

#[test]
fn oss_unit_tests_have_no_byok_backend() {
    assert!(
        !ChannelState::warp_cloud_enabled(),
        "unit tests run as Channel::Oss with Warp cloud disabled"
    );
    assert!(!byok_backend_available());
}

#[test]
fn backend_prerequisite_is_not_shown_for_custom_endpoints_alone() {
    let text = custom_inference_description_paragraphs(false, true, false, false).join("\n");
    assert!(!text.contains("not used"), "{text}");
}
