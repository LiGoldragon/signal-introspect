use dotos::DotosEncode;
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, RootCode,
    SessionEpoch, SignalOperationHeads, SubReply, VariantCode, WireRoute,
};
use signal_introspect::{
    ComponentReadiness, ComponentSnapshot, ComponentSnapshotQuery, ComponentTrace,
    ComponentTraceEvent, ComponentTraceQuery, DeliveryTrace, DeliveryTraceEvent, DeliveryTraceKey,
    DeliveryTraceQuery, DeliveryTraceStatus, EngineSnapshot, EngineSnapshotQuery, HopIndex,
    IntrospectionFrame as Frame, IntrospectionFrameBody as FrameBody, IntrospectionReply,
    IntrospectionRequest, IntrospectionTarget, PrototypeWitness, PrototypeWitnessQuery, SocketMode,
    TraceEventName, TraceLayer, TraceSequence, WirePath,
};
use signal_message::schema::lib::z2VLZR;
use signal_persona::schema::lib::{z2VRuG, z2VUT8};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn component_name(value: &str) -> z2VUT8 {
    z2VUT8::new(value.to_owned())
}

fn round_trip_request(request: IntrospectionRequest) {
    let request_payload = request.clone().into_request();
    let route = request_payload.route().expect("operation route");
    let frame = Frame::new(
        route,
        FrameBody::Request {
            exchange: exchange(),
            request: request_payload,
        },
    );

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => {
            assert_eq!(decoded_request.payloads().head(), &request);
        }
        other => panic!("expected introspection request, got {other:?}"),
    }
}

fn round_trip_reply(reply: IntrospectionReply) -> IntrospectionReply {
    let frame = Frame::new(
        WireRoute::new(RootCode::new(0), VariantCode::new(0)),
        FrameBody::Reply {
            exchange: exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
        },
    );

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
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
        engine: z2VRuG::new("prototype".to_owned()),
    });
    round_trip_request(request);
}

#[test]
fn component_snapshot_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::ComponentSnapshot(ComponentSnapshotQuery {
        engine: z2VRuG::new("prototype".to_owned()),
        target: IntrospectionTarget::Router,
    });
    round_trip_request(request);
}

#[test]
fn delivery_trace_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::DeliveryTrace(DeliveryTraceQuery {
        engine: z2VRuG::new("prototype".to_owned()),
        message_identifier: z2VLZR::new(7),
        originator: component_name("Message"),
    });
    round_trip_request(request);
}

#[test]
fn component_trace_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::ComponentTrace(ComponentTraceQuery::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        Some(TraceEventName::new("SignalAdmitted")),
    ));
    round_trip_request(request);
}

#[test]
fn component_trace_query_round_trips_with_no_event_name_filter() {
    let request = IntrospectionRequest::ComponentTrace(ComponentTraceQuery::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        None,
    ));
    round_trip_request(request);
}

#[test]
fn component_trace_reply_round_trips_through_length_prefixed_frame() {
    let events = vec![
        ComponentTraceEvent::new(
            z2VRuG::new("prototype".to_owned()),
            IntrospectionTarget::Signal,
            TraceLayer::Signal,
            TraceEventName::new("SignalAdmitted"),
            TraceSequence::new(0),
        ),
        ComponentTraceEvent::new(
            z2VRuG::new("prototype".to_owned()),
            IntrospectionTarget::Signal,
            TraceLayer::Sema,
            TraceEventName::new("SemaWriteApplied"),
            TraceSequence::new(1),
        ),
    ];
    let reply = IntrospectionReply::ComponentTrace(ComponentTrace::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        events.clone(),
    ));

    let recovered = round_trip_reply(reply.clone());
    assert_eq!(recovered, reply);
    match recovered {
        IntrospectionReply::ComponentTrace(trace) => {
            assert_eq!(trace.events(), events.as_slice());
            assert_eq!(trace.events()[0].sequence.value(), 0);
            assert_eq!(trace.events()[1].sequence, TraceSequence::new(0).next());
            assert_eq!(trace.events()[0].layer, TraceLayer::Signal);
            assert_eq!(trace.events()[1].event_name, "SemaWriteApplied");
        }
        other => panic!("expected component trace reply, got {other:?}"),
    }
}

#[test]
fn component_trace_event_round_trips_through_trace_event_frame() {
    use triad_runtime::trace::TraceEventFrame;

    let event = ComponentTraceEvent::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        TraceLayer::Nexus,
        TraceEventName::new("NexusEntered"),
        TraceSequence::new(42),
    );

    let archive = event
        .to_trace_archive()
        .expect("archive component trace event");
    let recovered = ComponentTraceEvent::from_trace_archive(&archive).expect("dearchive");
    assert_eq!(recovered, event);
}

#[test]
fn spirit_authorization_trace_event_round_trips_through_trace_event_frame() {
    use triad_runtime::trace::TraceEventFrame;

    let event = ComponentTraceEvent::new(
        z2VRuG::new("spirit".to_owned()),
        IntrospectionTarget::Spirit,
        TraceLayer::Authorization,
        TraceEventName::new("AuthorizationObserved"),
        TraceSequence::new(43),
    );

    let archive = event
        .to_trace_archive()
        .expect("archive spirit authorization trace event");
    let recovered = ComponentTraceEvent::from_trace_archive(&archive).expect("dearchive");
    assert_eq!(recovered, event);
}

#[test]
fn component_trace_event_displays_as_dotos() {
    let event = ComponentTraceEvent::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        TraceLayer::Signal,
        TraceEventName::new("SignalAdmitted"),
        TraceSequence::new(7),
    );
    assert_eq!(event.to_string(), event.to_dotos());
}

#[test]
fn component_trace_query_filter_matches_by_component_and_event_name() {
    let event = ComponentTraceEvent::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        TraceLayer::Signal,
        TraceEventName::new("SignalAdmitted"),
        TraceSequence::new(0),
    );

    let unfiltered = ComponentTraceQuery::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        None,
    );
    let matching_name = ComponentTraceQuery::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        Some(TraceEventName::new("SignalAdmitted")),
    );
    let other_name = ComponentTraceQuery::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Signal,
        Some(TraceEventName::new("NexusEntered")),
    );
    let other_component = ComponentTraceQuery::new(
        z2VRuG::new("prototype".to_owned()),
        IntrospectionTarget::Router,
        None,
    );

    assert!(event.matches_query(&unfiltered));
    assert!(event.matches_query(&matching_name));
    assert!(!event.matches_query(&other_name));
    assert!(!event.matches_query(&other_component));
}

#[test]
fn prototype_witness_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery {
        engine: z2VRuG::new("prototype".to_owned()),
    });
    round_trip_request(request);
}

#[test]
fn introspection_request_heads_are_contract_local_operations() {
    assert_eq!(
        <IntrospectionRequest as SignalOperationHeads>::HEADS,
        &[
            "EngineSnapshot",
            "ComponentSnapshot",
            "DeliveryTrace",
            "PrototypeWitness",
            "ComponentTrace",
            "RecordSystemEvent",
            "SystemEvents",
            "FlushSystemEvents",
        ]
    );
}

#[test]
fn prototype_witness_reply_round_trips_through_length_prefixed_frame() {
    let reply = IntrospectionReply::PrototypeWitness(PrototypeWitness {
        engine: z2VRuG::new("prototype".to_owned()),
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
        engine: z2VRuG::new("prototype".to_owned()),
        manager_seen: None,
        router_seen: None,
        terminal_seen: None,
        delivery_status: None,
    });
    assert_eq!(round_trip_reply(reply.clone()), reply);
}

#[test]
fn component_observations_are_wrapped_not_defined_here() {
    let engine_reply = IntrospectionReply::EngineSnapshot(EngineSnapshot::new(
        z2VRuG::new("prototype".to_owned()),
        vec![
            IntrospectionTarget::EngineManager,
            IntrospectionTarget::Router,
            IntrospectionTarget::Terminal,
        ],
    ));
    let component_reply = IntrospectionReply::ComponentSnapshot(ComponentSnapshot {
        engine: z2VRuG::new("prototype".to_owned()),
        target: IntrospectionTarget::Router,
        readiness: Some(ComponentReadiness::Ready),
    });
    let trace_reply = IntrospectionReply::DeliveryTrace(DeliveryTrace::new(
        z2VRuG::new("prototype".to_owned()),
        z2VLZR::new(7),
        component_name("Message"),
        vec![DeliveryTraceEvent::new(
            DeliveryTraceKey::new(
                z2VRuG::new("prototype".to_owned()),
                z2VLZR::new(7),
                component_name("Message"),
                HopIndex::new(1),
            ),
            component_name("Router"),
            DeliveryTraceStatus::Routed,
        )],
    ));

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
        z2VRuG::new("prototype".to_owned()),
        z2VLZR::new(7),
        component_name("Message"),
        HopIndex::new(3),
    );
    let reply = IntrospectionReply::DeliveryTrace(DeliveryTrace::new(
        z2VRuG::new("prototype".to_owned()),
        z2VLZR::new(7),
        component_name("Message"),
        vec![DeliveryTraceEvent::new(
            trace_key.clone(),
            component_name("Harness"),
            DeliveryTraceStatus::Failed,
        )],
    ));

    assert_eq!(round_trip_reply(reply.clone()), reply);
    assert_eq!(trace_key.hop_index.value(), 3);
    assert_eq!(trace_key.next_hop().hop_index.value(), 4);
    assert_eq!(
        trace_key.join_key().engine,
        z2VRuG::new("prototype".to_owned())
    );
    assert_eq!(trace_key.join_key().message_identifier, z2VLZR::new(7));
    assert_eq!(trace_key.join_key().originator, component_name("Message"));
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
fn introspect_daemon_configuration_round_trips_through_dotos_text() {
    use dotos::{DotosEncode, DotosSource};
    use signal_introspect::IntrospectDaemonConfiguration;
    use signal_persona::schema::lib::{z2VRBs, z2VaTc};

    let configuration = IntrospectDaemonConfiguration {
        introspect_socket_path: WirePath::new("/run/persona/X/introspect.sock"),
        introspect_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/introspect-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/introspect.sema"),
        manager_socket_path: WirePath::new("/run/persona/X/persona.sock"),
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        terminal_socket_path: WirePath::new("/run/persona/X/terminal.sock"),
        trace_socket_path: WirePath::new("/run/persona/X/introspect-trace.sock"),
        owner_identity: z2VRBs::z2VWNV(z2VaTc::new(1000)),
    };

    let text = configuration.to_dotos();
    let recovered = DotosSource::new(&text)
        .parse::<IntrospectDaemonConfiguration>()
        .expect("decode configuration");

    assert_eq!(recovered, configuration);
    assert!(text.contains("/run/persona/X/introspect.sock"));
}

#[test]
fn introspect_daemon_configuration_round_trips_through_rkyv() {
    use signal_introspect::IntrospectDaemonConfiguration;
    use signal_persona::schema::lib::{z2VRBs, z2VaTc};

    let configuration = IntrospectDaemonConfiguration {
        introspect_socket_path: WirePath::new("/run/persona/X/introspect.sock"),
        introspect_socket_mode: SocketMode::new(0o600),
        supervision_socket_path: WirePath::new("/run/persona/X/introspect-supervision.sock"),
        supervision_socket_mode: SocketMode::new(0o600),
        store_path: WirePath::new("/var/lib/persona/X/introspect.sema"),
        manager_socket_path: WirePath::new("/run/persona/X/persona.sock"),
        router_socket_path: WirePath::new("/run/persona/X/router.sock"),
        terminal_socket_path: WirePath::new("/run/persona/X/terminal.sock"),
        trace_socket_path: WirePath::new("/run/persona/X/introspect-trace.sock"),
        owner_identity: z2VRBs::z2VWNV(z2VaTc::new(1000)),
    };

    let bytes = configuration.to_rkyv_bytes().expect("archive");
    let recovered = IntrospectDaemonConfiguration::from_rkyv_bytes(&bytes).expect("decode rkyv");
    assert_eq!(recovered, configuration);
}
