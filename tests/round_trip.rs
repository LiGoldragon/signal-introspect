use signal_core::{FrameBody, Request, SemaVerb};
use signal_persona_auth::EngineId;
use signal_persona_introspect::{
    ComponentReadiness, ComponentSnapshot, ComponentSnapshotQuery, DeliveryTrace,
    DeliveryTraceQuery, DeliveryTraceStatus, EngineSnapshot, EngineSnapshotQuery, Frame,
    IntrospectionReply, IntrospectionRequest, IntrospectionTarget, PrototypeWitness,
};

#[test]
fn engine_snapshot_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::EngineSnapshot(EngineSnapshotQuery {
        engine: EngineId::new("prototype"),
    });
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { verb, payload }) => {
            assert_eq!(verb, SemaVerb::Assert);
            assert_eq!(payload, request);
        }
        other => panic!("expected Assert request, got {other:?}"),
    }
}

#[test]
fn component_snapshot_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::ComponentSnapshot(ComponentSnapshotQuery {
        engine: EngineId::new("prototype"),
        target: IntrospectionTarget::Router,
    });
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { payload, .. }) => {
            assert_eq!(payload, request);
        }
        other => panic!("expected request, got {other:?}"),
    }
}

#[test]
fn delivery_trace_query_round_trips_through_length_prefixed_frame() {
    let request = IntrospectionRequest::DeliveryTrace(DeliveryTraceQuery {
        engine: EngineId::new("prototype"),
        correlation: "fixture-delivery".into(),
    });
    let frame = Frame::new(FrameBody::Request(Request::assert(request.clone())));

    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");

    match decoded.into_body() {
        FrameBody::Request(Request::Operation { payload, .. }) => {
            assert_eq!(payload, request);
        }
        other => panic!("expected request, got {other:?}"),
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
