use super::TELEMETRY_POLICY;

#[test]
fn process_policy_forbids_remote_export() {
    assert!(!TELEMETRY_POLICY.remote_export_allowed());
}
