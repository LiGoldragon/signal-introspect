use signal_introspect::{
    ByteViewable, ComponentTraceQuery, IntrospectionTarget, Query, Response, Restorable, Signal,
    Signalizable,
};
#[test]
fn typed_binary_trace_contract_round_trips_fresh_peer_bytes() {
    let query = Query::ComponentTrace(ComponentTraceQuery {
        engine_identifier: "engine".into(),
        introspection_target: IntrospectionTarget::Spirit,
        optional_trace_event_name: None,
    });
    let sent = query.signalize().expect("archive");
    let received = Signal::<Query>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore"), query);
    let response = Response::Denied(signal_introspect::IntrospectionDenied {
        introspection_scope: signal_introspect::IntrospectionScope::EngineSnapshot,
        denied_reason: signal_introspect::DeniedReason::NotAuthorized,
    });
    let sent = response.signalize().expect("archive");
    let received = Signal::<Response>::from(sent.bytes().to_vec());
    assert_eq!(received.restore().expect("restore"), response);
}
#[test]
fn malformed_signal_is_rejected() {
    assert!(Signal::<Query>::from(vec![255, 0, 1]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn datom_round_trip_preserves_trace_query() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let query = Query::ComponentTrace(ComponentTraceQuery {
        engine_identifier: "engine".into(),
        introspection_target: IntrospectionTarget::Spirit,
        optional_trace_event_name: Some("started".into()),
    });
    let rendered = query.clone().datomize(vec![]).protosize().textualize();
    let mut pending = Potential::<Query>::from(rendered);
    assert_eq!(
        pending
            .actualize(&mut Budget {
                remaining: 4096,
                reader: ReaderBudget { remaining: 4096 },
                depth: 0,
                maximum_depth: 256
            })
            .expect("actualize"),
        query
    );
}
