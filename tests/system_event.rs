use nota::{NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};
use signal_introspect::{
    BluetoothPowerEvent, BluetoothPowerObservation, BluetoothSystemEvent, BluetoothTarget,
    BluetoothTopic, BootIdentifier, BoundedPayload, EventIdentifier, EventInstant, EventProvenance,
    EventSeverity, ExtractorRevision, IntrospectionFrame, IntrospectionFrameBody,
    IntrospectionReply, IntrospectionRequest, JournalSource, MAXIMUM_BOUNDED_PAYLOAD_BYTES,
    PolicyRevision, RecordSystemEvent, SystemEvent, SystemEventAccepted, TargetedSystemEvent,
    TargetedUnclassifiedStatus,
};

struct SystemEventFixture;

impl SystemEventFixture {
    fn event(
        identifier: u64,
        observed_at: u64,
        observation: BluetoothPowerObservation,
        payload: Option<BoundedPayload>,
    ) -> SystemEvent {
        SystemEvent {
            identifier: EventIdentifier::new(identifier),
            boot: BootIdentifier::new(1, 2),
            observed_at: EventInstant::new(observed_at),
            classification: TargetedSystemEvent::Bluetooth(BluetoothSystemEvent {
                target: BluetoothTarget::Controller,
                topic: BluetoothTopic::Power(observation),
            }),
            severity: EventSeverity::Notice,
            provenance: EventProvenance::trusted_journal(JournalSource::SystemdBluetoothService),
            extractor_revision: ExtractorRevision::new(3),
            policy_revision: PolicyRevision::new(4),
            payload,
        }
    }

    fn exchange() -> ExchangeIdentifier {
        ExchangeIdentifier::new(
            SessionEpoch::new(1),
            ExchangeLane::Connector,
            LaneSequence::first(),
        )
    }
}

#[test]
fn bounded_payload_truncates_on_a_utf8_boundary_without_retaining_fallback_text() {
    let input = format!("{}x", "é".repeat(256));
    let payload = BoundedPayload::from_redacted_allowlisted(&input);

    assert_eq!(input.len(), 513);
    assert_eq!(payload.as_str().len(), MAXIMUM_BOUNDED_PAYLOAD_BYTES);
    assert!(payload.as_str().is_char_boundary(payload.as_str().len()));
    assert!(payload.truncated());
    assert_eq!(payload.original_byte_length(), 513);
    assert!(!payload.as_str().ends_with('x'));
}

#[test]
fn bounded_payload_preserves_short_unicode_and_reports_original_length() {
    let payload = BoundedPayload::from_redacted_allowlisted("på");

    assert_eq!(payload.as_str(), "på");
    assert!(!payload.truncated());
    assert_eq!(payload.original_byte_length(), 3);
}

#[test]
fn decoded_payload_metadata_must_preserve_the_bound_invariant() {
    let valid = BoundedPayload::from_redacted_allowlisted("short");
    let invalid_text = valid.to_nota().replace("False 5", "False 6");
    let decoded = NotaSource::new(&invalid_text)
        .parse::<BoundedPayload>()
        .expect("decode deliberately inconsistent payload fixture");

    assert!(
        decoded.validate().is_err(),
        "fixture did not invalidate payload: {invalid_text}"
    );
}

#[test]
fn targeted_unclassified_input_rejects_a_durable_message_preview() {
    let event = SystemEventFixture::event(
        1,
        10,
        BluetoothPowerObservation::Unclassified(TargetedUnclassifiedStatus::Counted),
        Some(BoundedPayload::from_redacted_allowlisted(
            "raw command line or address",
        )),
    );

    assert!(event.validate().is_err());
    let status_only = SystemEventFixture::event(
        2,
        11,
        BluetoothPowerObservation::Unclassified(TargetedUnclassifiedStatus::Redacted),
        None,
    );
    assert!(status_only.validate().is_ok());
}

#[test]
fn exact_duplicate_identity_excludes_incidental_identity_and_time() {
    let first = SystemEventFixture::event(
        1,
        10,
        BluetoothPowerObservation::Event(BluetoothPowerEvent::ObservedOn),
        Some(BoundedPayload::from_redacted_allowlisted("state on")),
    );
    let second = SystemEventFixture::event(
        999,
        999_999,
        BluetoothPowerObservation::Event(BluetoothPowerEvent::ObservedOn),
        Some(BoundedPayload::from_redacted_allowlisted("state on")),
    );

    assert_eq!(
        first.exact_duplicate_identity(),
        second.exact_duplicate_identity()
    );
}

#[test]
fn system_event_operation_round_trips_at_the_nota_projection_boundary() {
    let request = IntrospectionRequest::RecordSystemEvent(RecordSystemEvent {
        event: SystemEventFixture::event(
            7,
            100,
            BluetoothPowerObservation::Event(BluetoothPowerEvent::RequestedOn),
            None,
        ),
    });
    let text = request.to_nota();
    let decoded = NotaSource::new(&text)
        .parse::<IntrospectionRequest>()
        .expect("decode system-event NOTA projection");
    assert_eq!(decoded, request);
    assert!(text.starts_with("(RecordSystemEvent "));
}

#[test]
fn system_event_ingestion_and_reply_round_trip_as_typed_binary_frames() {
    let event = SystemEventFixture::event(
        7,
        100,
        BluetoothPowerObservation::Event(BluetoothPowerEvent::RequestedOn),
        None,
    );
    let request = IntrospectionRequest::RecordSystemEvent(RecordSystemEvent {
        event: event.clone(),
    });
    let request_frame = IntrospectionFrame::new(IntrospectionFrameBody::Request {
        exchange: SystemEventFixture::exchange(),
        request: request.clone().into_request(),
    });
    let decoded = IntrospectionFrame::decode_length_prefixed(
        &request_frame
            .encode_length_prefixed()
            .expect("encode request"),
    )
    .expect("decode request");
    let IntrospectionFrameBody::Request { request: batch, .. } = decoded.into_body() else {
        panic!("request frame expected");
    };
    assert_eq!(batch.payloads().head(), &request);

    let reply = IntrospectionReply::SystemEventAccepted(SystemEventAccepted {
        representative_identifier: event.identifier,
        count: 2,
        suppressed_count: 1,
    });
    let reply_frame = IntrospectionFrame::new(IntrospectionFrameBody::Reply {
        exchange: SystemEventFixture::exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
    let decoded = IntrospectionFrame::decode_length_prefixed(
        &reply_frame.encode_length_prefixed().expect("encode reply"),
    )
    .expect("decode reply");
    let IntrospectionFrameBody::Reply { reply: decoded, .. } = decoded.into_body() else {
        panic!("reply frame expected");
    };
    let Reply::Accepted { per_operation, .. } = decoded else {
        panic!("accepted reply expected");
    };
    assert_eq!(per_operation.into_head(), SubReply::Ok(reply));
}
