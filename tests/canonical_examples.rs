//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as an `IntrospectionRequest` or `IntrospectionReply` and asserting
//! the re-encoded text equals the canonical form. Adding a new
//! variant requires adding both a canonical-text example and the
//! matching expected value here; the witness is what keeps the
//! examples file aligned with the typed surface.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_persona_auth::EngineId;
use signal_persona_introspect::{
    ComponentReadiness, ComponentSnapshot, ComponentSnapshotQuery, CorrelationId, DeliveryTrace,
    DeliveryTraceQuery, DeliveryTraceStatus, EngineSnapshot, EngineSnapshotQuery,
    IntrospectionDenied, IntrospectionDeniedReason, IntrospectionReply, IntrospectionRequest,
    IntrospectionScope, IntrospectionTarget, IntrospectionUnimplemented,
    IntrospectionUnimplementedReason, PrototypeWitness, PrototypeWitnessQuery,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn engine() -> EngineId {
    EngineId::new("prototype")
}

fn correlation() -> CorrelationId {
    CorrelationId::new("trace-7")
}

#[test]
fn canonical_request_examples_round_trip() {
    let expected: Vec<(IntrospectionRequest, &str)> = vec![
        (
            IntrospectionRequest::EngineSnapshot(EngineSnapshotQuery { engine: engine() }),
            "(EngineSnapshotQuery prototype)",
        ),
        (
            IntrospectionRequest::ComponentSnapshot(ComponentSnapshotQuery {
                engine: engine(),
                target: IntrospectionTarget::Router,
            }),
            "(ComponentSnapshotQuery prototype Router)",
        ),
        (
            IntrospectionRequest::DeliveryTrace(DeliveryTraceQuery {
                engine: engine(),
                correlation: correlation(),
            }),
            "(DeliveryTraceQuery prototype trace-7)",
        ),
        (
            IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery { engine: engine() }),
            "(PrototypeWitnessQuery prototype)",
        ),
    ];

    for (value, canonical_text) in expected {
        let mut encoder = Encoder::new();
        value.encode(&mut encoder).expect("encode");
        let text = encoder.into_string();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let mut decoder = Decoder::new(canonical_text);
        let decoded = IntrospectionRequest::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}

#[test]
fn canonical_reply_examples_round_trip() {
    let expected: Vec<(IntrospectionReply, &str)> = vec![
        (
            IntrospectionReply::EngineSnapshot(EngineSnapshot {
                engine: engine(),
                observed_components: vec![
                    IntrospectionTarget::Router,
                    IntrospectionTarget::Terminal,
                ],
            }),
            "(EngineSnapshot prototype [Router Terminal])",
        ),
        (
            IntrospectionReply::ComponentSnapshot(ComponentSnapshot {
                engine: engine(),
                target: IntrospectionTarget::Router,
                readiness: Some(ComponentReadiness::Ready),
            }),
            "(ComponentSnapshot prototype Router Ready)",
        ),
        (
            IntrospectionReply::ComponentSnapshot(ComponentSnapshot {
                engine: engine(),
                target: IntrospectionTarget::Router,
                readiness: None,
            }),
            "(ComponentSnapshot prototype Router None)",
        ),
        (
            IntrospectionReply::DeliveryTrace(DeliveryTrace {
                engine: engine(),
                correlation: correlation(),
                status: Some(DeliveryTraceStatus::Routed),
            }),
            "(DeliveryTrace prototype trace-7 Routed)",
        ),
        (
            IntrospectionReply::DeliveryTrace(DeliveryTrace {
                engine: engine(),
                correlation: correlation(),
                status: None,
            }),
            "(DeliveryTrace prototype trace-7 None)",
        ),
        (
            IntrospectionReply::PrototypeWitness(PrototypeWitness {
                engine: engine(),
                manager_seen: Some(ComponentReadiness::Ready),
                router_seen: Some(ComponentReadiness::Ready),
                terminal_seen: Some(ComponentReadiness::Ready),
                delivery_status: Some(DeliveryTraceStatus::Routed),
            }),
            "(PrototypeWitness prototype Ready Ready Ready Routed)",
        ),
        (
            IntrospectionReply::PrototypeWitness(PrototypeWitness {
                engine: engine(),
                manager_seen: None,
                router_seen: None,
                terminal_seen: None,
                delivery_status: None,
            }),
            "(PrototypeWitness prototype None None None None)",
        ),
        (
            IntrospectionReply::Unimplemented(IntrospectionUnimplemented {
                scope: IntrospectionScope::EngineSnapshot,
                reason: IntrospectionUnimplementedReason::NotInPrototypeScope,
            }),
            "(IntrospectionUnimplemented EngineSnapshot NotInPrototypeScope)",
        ),
        (
            IntrospectionReply::Denied(IntrospectionDenied {
                scope: IntrospectionScope::ComponentSnapshot,
                reason: IntrospectionDeniedReason::NotAuthorized,
            }),
            "(IntrospectionDenied ComponentSnapshot NotAuthorized)",
        ),
    ];

    for (value, canonical_text) in expected {
        let mut encoder = Encoder::new();
        value.encode(&mut encoder).expect("encode");
        let text = encoder.into_string();
        assert_eq!(text, canonical_text, "encode for {value:?}");

        let mut decoder = Decoder::new(canonical_text);
        let decoded = IntrospectionReply::decode(&mut decoder).expect("decode");
        assert_eq!(decoded, value, "decode for {canonical_text}");

        assert!(
            CANONICAL.contains(canonical_text),
            "examples/canonical.nota missing line: {canonical_text}",
        );
    }
}
