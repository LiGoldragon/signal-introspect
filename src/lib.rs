//! Signal contract for Persona introspection query and projection envelopes.
//!
//! This crate asks and wraps observations. Component-owned observation
//! records live in the component contract that owns the observed state
//! (`signal-persona`, `signal-persona-terminal`,
//! `signal-persona-router`, etc.). This crate must not become a bucket
//! for every component's internal rows.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;
use signal_persona_auth::EngineId;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum IntrospectionTarget {
    EngineManager,
    Mind,
    Message,
    Router,
    System,
    Harness,
    Terminal,
    Introspect,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum IntrospectionScope {
    EngineSnapshot,
    ComponentSnapshot,
    DeliveryTrace,
    PrototypeWitness,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct CorrelationId(String);

impl CorrelationId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&str> for CorrelationId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineSnapshotQuery {
    pub engine: EngineId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentSnapshotQuery {
    pub engine: EngineId,
    pub target: IntrospectionTarget,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTraceQuery {
    pub engine: EngineId,
    pub correlation: CorrelationId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PrototypeWitnessQuery {
    pub engine: EngineId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineSnapshot {
    pub engine: EngineId,
    pub observed_components: Vec<IntrospectionTarget>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentSnapshot {
    pub engine: EngineId,
    pub target: IntrospectionTarget,
    pub readiness: ComponentReadiness,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ComponentReadiness {
    Ready,
    NotReady,
    Unknown,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTrace {
    pub engine: EngineId,
    pub correlation: CorrelationId,
    pub status: DeliveryTraceStatus,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum DeliveryTraceStatus {
    Accepted,
    Routed,
    Delivered,
    Deferred,
    Failed,
    Unknown,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PrototypeWitness {
    pub engine: EngineId,
    pub manager_seen: ComponentReadiness,
    pub router_seen: ComponentReadiness,
    pub terminal_seen: ComponentReadiness,
    pub delivery_status: DeliveryTraceStatus,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct IntrospectionUnimplemented {
    pub scope: IntrospectionScope,
    pub reason: IntrospectionUnimplementedReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum IntrospectionUnimplementedReason {
    NotInPrototypeScope,
    ComponentObservationMissing,
    SubscriptionNotImplemented,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct IntrospectionDenied {
    pub scope: IntrospectionScope,
    pub reason: IntrospectionDeniedReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum IntrospectionDeniedReason {
    NotAuthorized,
    Redacted,
}

signal_channel! {
    request IntrospectionRequest {
        Match EngineSnapshot(EngineSnapshotQuery),
        Match ComponentSnapshot(ComponentSnapshotQuery),
        Match DeliveryTrace(DeliveryTraceQuery),
        Match PrototypeWitness(PrototypeWitnessQuery),
    }

    reply IntrospectionReply {
        EngineSnapshot(EngineSnapshot),
        ComponentSnapshot(ComponentSnapshot),
        DeliveryTrace(DeliveryTrace),
        PrototypeWitness(PrototypeWitness),
        Unimplemented(IntrospectionUnimplemented),
        Denied(IntrospectionDenied),
    }
}
