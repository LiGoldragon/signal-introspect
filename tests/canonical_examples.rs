//! Canonical examples round-trip witness.
//!
//! Parses `examples/canonical.nota` end-to-end, decoding each record
//! as an `IntrospectionRequest` or `IntrospectionReply` and asserting
//! the re-encoded text equals the canonical form. Adding a new
//! variant requires adding both a canonical-text example and the
//! matching expected value here; the witness is what keeps the
//! examples file aligned with the typed surface.

use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use signal_persona_auth::{ComponentName, EngineId};
use signal_persona_introspect::{
    ComponentReadiness, ComponentSnapshot, ComponentSnapshotQuery, DeliveryTrace,
    DeliveryTraceEvent, DeliveryTraceKey, DeliveryTraceQuery, DeliveryTraceStatus, EngineSnapshot,
    EngineSnapshotQuery, HopIndex, IntrospectionDenied, IntrospectionDeniedReason,
    IntrospectionReply, IntrospectionRequest, IntrospectionScope, IntrospectionTarget,
    IntrospectionUnimplemented, IntrospectionUnimplementedReason, MessageIdentifier,
    PrototypeWitness, PrototypeWitnessQuery,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn engine() -> EngineId {
    EngineId::new("prototype")
}

#[test]
fn canonical_request_examples_round_trip() {
    let expected: Vec<(IntrospectionRequest, &str)> = vec![
        (
            IntrospectionRequest::EngineSnapshot(EngineSnapshotQuery { engine: engine() }),
            "(EngineSnapshot (prototype))",
        ),
        (
            IntrospectionRequest::ComponentSnapshot(ComponentSnapshotQuery {
                engine: engine(),
                target: IntrospectionTarget::Router,
            }),
            "(ComponentSnapshot (prototype Router))",
        ),
        (
            IntrospectionRequest::DeliveryTrace(DeliveryTraceQuery {
                engine: engine(),
                message_identifier: MessageIdentifier::new(7),
                originator: ComponentName::Message,
            }),
            "(DeliveryTrace (prototype 7 Message))",
        ),
        (
            IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery { engine: engine() }),
            "(PrototypeWitness (prototype))",
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
            "(EngineSnapshot (prototype [Router Terminal]))",
        ),
        (
            IntrospectionReply::ComponentSnapshot(ComponentSnapshot {
                engine: engine(),
                target: IntrospectionTarget::Router,
                readiness: Some(ComponentReadiness::Ready),
            }),
            "(ComponentSnapshot (prototype Router (Some Ready)))",
        ),
        (
            IntrospectionReply::ComponentSnapshot(ComponentSnapshot {
                engine: engine(),
                target: IntrospectionTarget::Router,
                readiness: None,
            }),
            "(ComponentSnapshot (prototype Router None))",
        ),
        (
            IntrospectionReply::DeliveryTrace(DeliveryTrace {
                engine: engine(),
                message_identifier: MessageIdentifier::new(7),
                originator: ComponentName::Message,
                events: vec![DeliveryTraceEvent::new(
                    DeliveryTraceKey::new(
                        engine(),
                        MessageIdentifier::new(7),
                        ComponentName::Message,
                        HopIndex::new(1),
                    ),
                    ComponentName::Router,
                    DeliveryTraceStatus::Routed,
                )],
            }),
            "(DeliveryTrace (prototype 7 Message [((prototype 7 Message 1) Router Routed)]))",
        ),
        (
            IntrospectionReply::DeliveryTrace(DeliveryTrace {
                engine: engine(),
                message_identifier: MessageIdentifier::new(7),
                originator: ComponentName::Message,
                events: Vec::new(),
            }),
            "(DeliveryTrace (prototype 7 Message []))",
        ),
        (
            IntrospectionReply::PrototypeWitness(PrototypeWitness {
                engine: engine(),
                manager_seen: Some(ComponentReadiness::Ready),
                router_seen: Some(ComponentReadiness::Ready),
                terminal_seen: Some(ComponentReadiness::Ready),
                delivery_status: Some(DeliveryTraceStatus::Routed),
            }),
            "(PrototypeWitness (prototype (Some Ready) (Some Ready) (Some Ready) (Some Routed)))",
        ),
        (
            IntrospectionReply::PrototypeWitness(PrototypeWitness {
                engine: engine(),
                manager_seen: None,
                router_seen: None,
                terminal_seen: None,
                delivery_status: None,
            }),
            "(PrototypeWitness (prototype None None None None))",
        ),
        (
            IntrospectionReply::Unimplemented(IntrospectionUnimplemented {
                scope: IntrospectionScope::EngineSnapshot,
                reason: IntrospectionUnimplementedReason::NotInPrototypeScope,
            }),
            "(Unimplemented (EngineSnapshot NotInPrototypeScope))",
        ),
        (
            IntrospectionReply::Denied(IntrospectionDenied {
                scope: IntrospectionScope::ComponentSnapshot,
                reason: IntrospectionDeniedReason::NotAuthorized,
            }),
            "(Denied (ComponentSnapshot NotAuthorized))",
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
