use super::*;

/// The File explorer chip is offered to the agent view toolbelt editor.
#[test]
fn file_explorer_is_offered_in_the_agent_view() {
    assert!(
        AgentToolbarItemKind::FileExplorer
            .available_in()
            .is_available_for_agent_view()
    );
    assert!(AgentToolbarItemKind::all_available().contains(&AgentToolbarItemKind::FileExplorer));
}

/// ...but it is opt-in, so it must stay out of the default agent view layout.
#[test]
fn file_explorer_is_not_an_agent_view_default() {
    assert!(!AgentToolbarItemKind::default_left().contains(&AgentToolbarItemKind::FileExplorer));
    assert!(!AgentToolbarItemKind::default_right().contains(&AgentToolbarItemKind::FileExplorer));
}

/// Without voice support (OSS builds), the CLI-agent configurator offers no Voice Input chip.
#[test]
fn cli_configurator_omits_voice_input_when_unsupported() {
    assert!(
        !AgentToolbarItemKind::all_available_for_cli_input_with(false)
            .contains(&AgentToolbarItemKind::VoiceInput)
    );
}

/// ...and Restore default yields only supported chips.
#[test]
fn cli_defaults_omit_voice_input_when_unsupported() {
    assert!(
        !AgentToolbarItemKind::cli_default_left_with(false)
            .contains(&AgentToolbarItemKind::VoiceInput)
    );
    assert!(!AgentToolbarItemKind::cli_default_right().contains(&AgentToolbarItemKind::VoiceInput));
}

/// Builds that support voice keep the chip in both lists.
#[test]
fn cli_lists_keep_voice_input_when_supported() {
    assert!(
        AgentToolbarItemKind::all_available_for_cli_input_with(true)
            .contains(&AgentToolbarItemKind::VoiceInput)
    );
    assert!(
        AgentToolbarItemKind::cli_default_left_with(true)
            .contains(&AgentToolbarItemKind::VoiceInput)
    );
}

/// Every CLI default is also offered by the configurator, so a restored layout is never
/// pruned down to something other than the defaults when saved.
#[test]
fn cli_defaults_are_subset_of_available() {
    for supported in [false, true] {
        let available = AgentToolbarItemKind::all_available_for_cli_input_with(supported);
        for item in AgentToolbarItemKind::cli_default_left_with(supported)
            .into_iter()
            .chain(AgentToolbarItemKind::cli_default_right())
        {
            assert!(
                available.contains(&item),
                "{item:?} missing (voice supported: {supported})"
            );
        }
    }
}
