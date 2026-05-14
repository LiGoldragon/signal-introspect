use signal_core::{FrameBody, Request, SignalVerb};
use signal_persona_auth::EngineId;
use signal_persona_introspect::{
    ComponentReadiness, ComponentSnapshot, ComponentSnapshotQuery, DeliveryTrace,
    DeliveryTraceQuery, DeliveryTraceStatus, EngineSnapshot, EngineSnapshotQuery, Frame,
    IntrospectionReply, IntrospectionRequest, IntrospectionTarget, PrototypeWitness,
    PrototypeWitnessQuery,
};

fn round_trip_request(request: IntrospectionRequest) {
    let expected_verb = request.signal_verb();
    let frame = Frame::new(FrameBody::Request(request.clone().into_signal_request()));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, expected_verb);
            assert_eq!(verb, SignalVerb::Match);
            assert_eq!(payload, request);
        }
        other => panic!("expected Match request, got {other:?}"),
    }
}

#[test]
fn engine_snapshot_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::EngineSnapshot(EngineSnapshotQuery {
        engine: EngineId::new("prototype"),
    });
    round_trip_request(request);
}

#[test]
fn component_snapshot_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::ComponentSnapshot(ComponentSnapshotQuery {
        engine: EngineId::new("prototype"),
        target: IntrospectionTarget::Router,
    });
    round_trip_request(request);
}

#[test]
fn delivery_trace_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::DeliveryTrace(DeliveryTraceQuery {
        engine: EngineId::new("prototype"),
        correlation: "fixture-delivery".into(),
    });
    round_trip_request(request);
}

#[test]
fn prototype_witness_query_round_trips_as_match_request() {
    let request = IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery {
        engine: EngineId::new("prototype"),
    });
    round_trip_request(request);
}

#[test]
fn introspection_request_variants_are_read_shaped_match_operations() {
    let requests = [
        IntrospectionRequest::EngineSnapshot(EngineSnapshotQuery {
            engine: EngineId::new("prototype"),
        }),
        IntrospectionRequest::ComponentSnapshot(ComponentSnapshotQuery {
            engine: EngineId::new("prototype"),
            target: IntrospectionTarget::Router,
        }),
        IntrospectionRequest::DeliveryTrace(DeliveryTraceQuery {
            engine: EngineId::new("prototype"),
            correlation: "fixture-delivery".into(),
        }),
        IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery {
            engine: EngineId::new("prototype"),
        }),
    ];

    for request in requests {
        assert_eq!(request.signal_verb(), SignalVerb::Match);
    }
}

#[test]
fn prototype_witness_reply_round_trips_through_length_prefixed_frame() {
    let reply = IntrospectionReply::PrototypeWitness(PrototypeWitness {
        engine: EngineId::new("prototype"),
        manager_seen: ComponentReadiness::Ready,
        router_seen: ComponentReadiness::Ready,
        terminal_seen: ComponentReadiness::Ready,
        delivery_status: DeliveryTraceStatus::Delivered,
    });
    let frame = Frame::new(FrameBody::Reply(signal_core::Reply::operation(
        reply.clone(),
    )));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Reply(signal_core::Reply::Operation(decoded_reply)) => {
            assert_eq!(decoded_reply, reply);
        }
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn component_observations_are_wrapped_not_defined_here() {
    let engine_reply = IntrospectionReply::EngineSnapshot(EngineSnapshot {
        engine: EngineId::new("prototype"),
        observed_components: vec![
            IntrospectionTarget::EngineManager,
            IntrospectionTarget::Router,
            IntrospectionTarget::Terminal,
        ],
    });
    let component_reply = IntrospectionReply::ComponentSnapshot(ComponentSnapshot {
        engine: EngineId::new("prototype"),
        target: IntrospectionTarget::Router,
        readiness: ComponentReadiness::Ready,
    });
    let trace_reply = IntrospectionReply::DeliveryTrace(DeliveryTrace {
        engine: EngineId::new("prototype"),
        correlation: "fixture-delivery".into(),
        status: DeliveryTraceStatus::Routed,
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
