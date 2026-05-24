use signal_core::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalVerb, SubReply,
};
use signal_introspect::{
    ComponentReadiness, ComponentSnapshot, ComponentSnapshotQuery, DeliveryTrace,
    DeliveryTraceEvent, DeliveryTraceKey, DeliveryTraceQuery, DeliveryTraceStatus, EngineSnapshot,
    EngineSnapshotQuery, HopIndex, IntrospectionFrame as Frame,
    IntrospectionFrameBody as FrameBody, IntrospectionReply, IntrospectionRequest,
    IntrospectionTarget, MessageIdentifier, PrototypeWitness, PrototypeWitnessQuery,
};
use signal_persona_origin::{ComponentName, EngineIdentifier};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: IntrospectionRequest) {
    let expected_verb = request.signal_verb();
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            let operation = decoded_request.operations().head();
            assert_eq!(operation.verb, expected_verb);
            assert_eq!(operation.verb, SignalVerb::Match);
            assert_eq!(operation.payload, request);
        }
        other => panic!("expected Match request, got {other:?}"),
    }
}

fn round_trip_reply(reply: IntrospectionReply) -> IntrospectionReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::completed(NonEmpty::single(SubReply::Ok {
            verb: SignalVerb::Match,
            payload: reply,
        })),
    });

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok { payload, .. } => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn engine_snapshot_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::EngineSnapshot(EngineSnapshotQuery {
        engine: EngineIdentifier::new("prototype"),
    });
    round_trip_request(request);
}

#[test]
fn component_snapshot_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::ComponentSnapshot(ComponentSnapshotQuery {
        engine: EngineIdentifier::new("prototype"),
        target: IntrospectionTarget::Router,
    });
    round_trip_request(request);
}

#[test]
fn delivery_trace_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::DeliveryTrace(DeliveryTraceQuery {
        engine: EngineIdentifier::new("prototype"),
        message_identifier: MessageIdentifier::new(7),
        originator: ComponentName::Message,
    });
    round_trip_request(request);
}

#[test]
fn prototype_witness_query_round_trips_as_match_request() {
    let request = IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery {
        engine: EngineIdentifier::new("prototype"),
    });
    round_trip_request(request);
}

#[test]
fn introspection_request_variants_are_read_shaped_match_operations() {
    let requests = [
        IntrospectionRequest::EngineSnapshot(EngineSnapshotQuery {
            engine: EngineIdentifier::new("prototype"),
        }),
        IntrospectionRequest::ComponentSnapshot(ComponentSnapshotQuery {
            engine: EngineIdentifier::new("prototype"),
            target: IntrospectionTarget::Router,
        }),
        IntrospectionRequest::DeliveryTrace(DeliveryTraceQuery {
            engine: EngineIdentifier::new("prototype"),
            message_identifier: MessageIdentifier::new(7),
            originator: ComponentName::Message,
        }),
        IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery {
            engine: EngineIdentifier::new("prototype"),
        }),
    ];

    for request in requests {
        assert_eq!(request.signal_verb(), SignalVerb::Match);
    }
}

#[test]
fn prototype_witness_reply_round_trips_through_length_prefixed_frame() {
    let reply = IntrospectionReply::PrototypeWitness(PrototypeWitness {
        engine: EngineIdentifier::new("prototype"),
        manager_seen: Some(ComponentReadiness::Ready),
        router_seen: Some(ComponentReadiness::Ready),
        terminal_seen: Some(ComponentReadiness::Ready),
        delivery_status: Some(DeliveryTraceStatus::Delivered),
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn prototype_witness_reply_round_trips_with_no_observations_yet() {
    // Witness for the "not observed yet" semantic: the closed-enum fields
    // stay closed, and the unobserved state is named by None on the
    // Option<>. Adding back an `Unknown` variant on either inner enum
    // would defeat the closed-enum integrity test below.
    let reply = IntrospectionReply::PrototypeWitness(PrototypeWitness {
        engine: EngineIdentifier::new("prototype"),
        manager_seen: None,
        router_seen: None,
        terminal_seen: None,
        delivery_status: None,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn component_observations_are_wrapped_not_defined_here() {
    let engine_reply = IntrospectionReply::EngineSnapshot(EngineSnapshot {
        engine: EngineIdentifier::new("prototype"),
        observed_components: vec![
            IntrospectionTarget::EngineManager,
            IntrospectionTarget::Router,
            IntrospectionTarget::Terminal,
        ],
    });
    let component_reply = IntrospectionReply::ComponentSnapshot(ComponentSnapshot {
        engine: EngineIdentifier::new("prototype"),
        target: IntrospectionTarget::Router,
        readiness: Some(ComponentReadiness::Ready),
    });
    let trace_reply = IntrospectionReply::DeliveryTrace(DeliveryTrace {
        engine: EngineIdentifier::new("prototype"),
        message_identifier: MessageIdentifier::new(7),
        originator: ComponentName::Message,
        events: vec![DeliveryTraceEvent::new(
            DeliveryTraceKey::new(
                EngineIdentifier::new("prototype"),
                MessageIdentifier::new(7),
                ComponentName::Message,
                HopIndex::new(1),
            ),
            ComponentName::Router,
            DeliveryTraceStatus::Routed,
        )],
    });

    assert!(matches!(
        engine_reply,
        IntrospectionReply::EngineSnapshot(_)
    ));
    assert!(matches!(
        component_reply,
        IntrospectionReply::ComponentSnapshot(_)
    ));
    assert!(matches!(trace_reply, IntrospectionReply::DeliveryTrace(_)));
}

#[test]
fn delivery_trace_key_round_trips_with_four_correlation_fields() {
    let trace_key = DeliveryTraceKey::new(
        EngineIdentifier::new("prototype"),
        MessageIdentifier::new(7),
        ComponentName::Message,
        HopIndex::new(3),
    );
    let reply = IntrospectionReply::DeliveryTrace(DeliveryTrace {
        engine: EngineIdentifier::new("prototype"),
        message_identifier: MessageIdentifier::new(7),
        originator: ComponentName::Message,
        events: vec![DeliveryTraceEvent::new(
            trace_key.clone(),
            ComponentName::Harness,
            DeliveryTraceStatus::Failed,
        )],
    });

    assert_eq!(round_trip_reply(reply.clone()), reply);
    assert_eq!(trace_key.hop_index.value(), 3);
    assert_eq!(trace_key.next_hop().hop_index.value(), 4);
    assert_eq!(
        trace_key.join_key().engine,
        EngineIdentifier::new("prototype")
    );
    assert_eq!(
        trace_key.join_key().message_identifier,
        MessageIdentifier::new(7)
    );
    assert_eq!(trace_key.join_key().originator, ComponentName::Message);
}

#[test]
fn introspection_status_enums_are_closed_no_unknown_variants() {
    // Witness for the closed-enum integrity rule: ComponentReadiness and
    // DeliveryTraceStatus must enumerate only positively-named observed
    // states. The "not yet observed" axis lives on Option<> wrapping these
    // values in their carrier records (ComponentSnapshot.readiness,
    // DeliveryTrace.status, PrototypeWitness.*), never inside the enums.
    for readiness in [ComponentReadiness::Ready, ComponentReadiness::NotReady] {
        let observed = match readiness {
            ComponentReadiness::Ready => "ready",
            ComponentReadiness::NotReady => "not-ready",
        };
        assert!(!observed.is_empty());
    }
    for status in [
        DeliveryTraceStatus::Accepted,
        DeliveryTraceStatus::Routed,
        DeliveryTraceStatus::Delivered,
        DeliveryTraceStatus::Deferred,
        DeliveryTraceStatus::Failed,
    ] {
        let observed = match status {
            DeliveryTraceStatus::Accepted => "accepted",
            DeliveryTraceStatus::Routed => "routed",
            DeliveryTraceStatus::Delivered => "delivered",
            DeliveryTraceStatus::Deferred => "deferred",
            DeliveryTraceStatus::Failed => "failed",
        };
        assert!(!observed.is_empty());
    }
}

#[test]
fn introspect_daemon_configuration_round_trips_through_nota_text() {
    use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
    use signal_introspect::IntrospectDaemonConfiguration;
    use signal_persona::{SocketMode, WirePath};
    use signal_persona_origin::{OwnerIdentity, UnixUserId};

    let configuration = IntrospectDaemonConfiguration {
        introspect_socket_path: WirePath::new("/run/persona/X/introspect.sock"),
        introspect_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/introspect-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/introspect.redb"),
        manager_socket_path: WirePath::new("/run/persona/X/persona.sock"),
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        terminal_socket_path: WirePath::new("/run/persona/X/terminal.sock"),
        owner_identity: OwnerIdentity::UnixUser(UnixUserId::new(1000)),
    };

    let mut encoder = Encoder::new();
    configuration
        .encode(&mut encoder)
        .expect("encode configuration");
    let text = encoder.into_string();
    let mut decoder = Decoder::new(&text);
    let recovered =
        IntrospectDaemonConfiguration::decode(&mut decoder).expect("decode configuration");

    assert_eq!(recovered, configuration);
}

#[test]
fn introspect_daemon_configuration_round_trips_through_rkyv() {
    use nota_config::ConfigurationRecord;
    use signal_introspect::IntrospectDaemonConfiguration;
    use signal_persona::{SocketMode, WirePath};
    use signal_persona_origin::{OwnerIdentity, UnixUserId};

    let configuration = IntrospectDaemonConfiguration {
        introspect_socket_path: WirePath::new("/run/persona/X/introspect.sock"),
        introspect_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/introspect-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/introspect.redb"),
        manager_socket_path: WirePath::new("/run/persona/X/persona.sock"),
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        terminal_socket_path: WirePath::new("/run/persona/X/terminal.sock"),
        owner_identity: OwnerIdentity::UnixUser(UnixUserId::new(1000)),
    };

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&configuration).expect("archive");
    let recovered = IntrospectDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}
