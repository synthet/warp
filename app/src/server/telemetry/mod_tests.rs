use rudder_message::Track;
use virtual_fs::VirtualFS;
use warp_core::channel::RudderStackDestination;

use super::*;

// Tests that events with UGC are not persisted to desk.
#[test]
fn test_persist_events_doesnt_include_ugc_events() {
    let telemetry_api = TelemetryApi::new();

    VirtualFS::test(
        "test_persist_events_doesnt_include_ugc_events",
        |dirs, _sandbox| {
            // Add one event without UGC
            let user_id = Some("user".into());
            let anonymous_id = "anonymous_id".to_owned();

            warpui::telemetry::record_event(
                user_id.clone(),
                anonymous_id.clone(),
                "non UGC event name".into(),
                None,  /* payload */
                false, /* contains_ugc  */
                warpui::time::get_current_time(),
            );

            warpui::telemetry::record_event(
                user_id.clone(),
                anonymous_id.clone(),
                "UGC event name".into(),
                None, /* payload */
                true, /* contains_ugc  */
                warpui::time::get_current_time(),
            );

            let file_path = dirs.root().join("rudderstack");

            telemetry_api
                .flush_and_persist_events_at_path(
                    10,
                    PrivacySettingsSnapshot::mock_opted_in(),
                    &file_path,
                )
                .expect("Should be able to persist events");

            let file_content: Vec<RudderBatchMessage> =
                serde_json::from_reader(File::open(file_path).expect("Failed to open file"))
                    .expect("Failed to parse file");

            // `record_event` feeds a process-wide queue that other tests write to as
            // well, so a whole-suite run flushes their events here too. Assert on the
            // two events this test recorded rather than on the total.
            let event_names: Vec<&str> = file_content
                .iter()
                .filter_map(|message| match message {
                    RudderBatchMessage::Track(track) => Some(track.event.as_str()),
                    _ => None,
                })
                .collect();
            assert!(
                event_names.contains(&"non UGC event name"),
                "the non-UGC event should be persisted, got {event_names:?}"
            );
            assert!(
                !event_names.contains(&"UGC event name"),
                "the UGC event must never be persisted, got {event_names:?}"
            );
        },
    );
}

#[tokio::test]
async fn send_batch_does_not_http_when_remote_export_disabled() {
    let mut telemetry_api = TelemetryApi::new();
    let requested = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let requested_for_hook = requested.clone();
    telemetry_api
        .client
        .set_before_request_fn(Box::new(move |_, _| {
            requested_for_hook.store(true, std::sync::atomic::Ordering::SeqCst);
        }));

    let messages = vec![RudderBatchMessageWithMetadata {
        message: RudderBatchMessage::Track(Track {
            event: "test.event".into(),
            ..Default::default()
        }),
        contains_ugc: false,
    }];

    telemetry_api
        .send_batch_messages_to_rudder(messages, PrivacySettingsSnapshot::mock_opted_in())
        .await
        .expect("remote-export kill-switch should succeed without HTTP");

    assert!(
        !requested.load(std::sync::atomic::Ordering::SeqCst),
        "Rudderstack HTTP must not run when remote export is disabled"
    );
}

#[tokio::test]
async fn send_rudder_request_does_not_http_when_remote_export_disabled() {
    let mut telemetry_api = TelemetryApi::new();
    let requested = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let requested_for_hook = requested.clone();
    telemetry_api
        .client
        .set_before_request_fn(Box::new(move |_, _| {
            requested_for_hook.store(true, std::sync::atomic::Ordering::SeqCst);
        }));

    telemetry_api
        .send_rudder_request(
            RudderMessage::Track(Track {
                event: "test.event".into(),
                ..Default::default()
            }),
            RudderStackDestination {
                root_url: "https://example.com".into(),
                write_key: "fake-write-key".into(),
            },
        )
        .await
        .expect("remote-export kill-switch should succeed without HTTP");

    assert!(
        !requested.load(std::sync::atomic::Ordering::SeqCst),
        "Rudderstack HTTP must not run when remote export is disabled"
    );
}

#[test]
fn flush_persisted_events_does_not_http_when_remote_export_disabled() {
    let mut telemetry_api = TelemetryApi::new();
    let requested = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let requested_for_hook = requested.clone();
    telemetry_api
        .client
        .set_before_request_fn(Box::new(move |_, _| {
            requested_for_hook.store(true, std::sync::atomic::Ordering::SeqCst);
        }));

    warpui::r#async::block_on(async {
        telemetry_api
            .flush_persisted_events_to_rudder(
                std::path::Path::new("rudder_telemetry_events.json"),
                PrivacySettingsSnapshot::mock_opted_in(),
            )
            .await
            .expect("persisted flush should no-op without HTTP");
    });

    assert!(
        !requested.load(std::sync::atomic::Ordering::SeqCst),
        "persisted Rudderstack flush must not HTTP when remote export is disabled"
    );
}
