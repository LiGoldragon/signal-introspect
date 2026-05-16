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
use signal_persona::{SocketMode, WirePath};
use signal_persona_auth::{EngineId, OwnerIdentity};

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

/// Snapshot of one peer component's readiness as observed by the
/// introspect daemon. `readiness` is `None` when the daemon has not yet
/// queried the peer (initial state before any observation lands);
/// `Some(state)` carries the closed observation of that peer's readiness.
/// The `Option<>` carries the "not yet observed" axis so
/// `ComponentReadiness` itself stays closed per ESSENCE
/// §"Perfect specificity at boundaries."
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentSnapshot {
    pub engine: EngineId,
    pub target: IntrospectionTarget,
    pub readiness: Option<ComponentReadiness>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ComponentReadiness {
    Ready,
    NotReady,
}

/// Latest observed delivery state for the named correlation. `status` is
/// `None` when the introspect daemon has not yet seen any router trace
/// event for that correlation (subscriptions or pulls have not delivered
/// the trace yet); `Some(state)` carries the closed status mirroring
/// `signal_persona_router::RouterDeliveryStatus`. The split keeps
/// `DeliveryTraceStatus` closed; the "not observed yet" axis is named in
/// the `Option<>` rather than smuggled into the status enum.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTrace {
    pub engine: EngineId,
    pub correlation: CorrelationId,
    pub status: Option<DeliveryTraceStatus>,
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
}

/// Roll-up of the prototype's three peer-component observations plus the
/// most recent delivery trace. Every field is an `Option<>`; `None`
/// means "the introspect daemon has not yet collected an observation
/// from that peer in this engine," distinguishable from observed states
/// without polluting the closed inner enums.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PrototypeWitness {
    pub engine: EngineId,
    pub manager_seen: Option<ComponentReadiness>,
    pub router_seen: Option<ComponentReadiness>,
    pub terminal_seen: Option<ComponentReadiness>,
    pub delivery_status: Option<DeliveryTraceStatus>,
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
    channel Introspection {
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
}

// ─── Daemon configuration ──────────────────────────────────
//
// Typed startup configuration for `persona-introspect-daemon`.
// The persona manager writes one of these (NOTA or rkyv) to a
// state-dir path and passes that path as argv. The daemon decodes
// through `nota_config::ConfigurationSource::from_argv()?.decode()?`
// and runs with the resulting record. No environment variables on
// the production launch path.

/// Startup configuration for `persona-introspect-daemon`.
///
/// Replaces the previous `PERSONA_INTROSPECT_SOCKET`,
/// `PERSONA_SOCKET_PATH`, `PERSONA_SOCKET_MODE`,
/// `PERSONA_SUPERVISION_SOCKET_PATH`,
/// `PERSONA_SUPERVISION_SOCKET_MODE`,
/// `PERSONA_INTROSPECT_STORE`, `PERSONA_STATE_PATH`,
/// `PERSONA_MANAGER_SOCKET_PATH`, and the
/// `PERSONA_PEER_*` peer-socket enumeration environment-variable
/// surface.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct IntrospectDaemonConfiguration {
    /// Where the daemon binds its introspection-query Unix socket.
    pub introspect_socket_path: WirePath,
    /// chmod applied to the introspection-query socket after bind.
    pub introspect_socket_mode: SocketMode,
    /// Where the daemon binds its supervision Unix socket.
    pub supervision_socket_path: WirePath,
    /// chmod applied to the supervision socket after bind.
    pub supervision_socket_mode: SocketMode,
    /// Path to the introspect daemon's redb store file.
    pub store_path: WirePath,
    /// Engine manager's supervision socket (peer).
    pub manager_socket_path: WirePath,
    /// Router daemon's domain socket (peer).
    pub router_socket_path: WirePath,
    /// Terminal supervisor's domain socket (peer).
    pub terminal_socket_path: WirePath,
    /// The engine owner identity passed to the introspect daemon.
    pub owner_identity: OwnerIdentity,
}

nota_config::impl_rkyv_configuration!(IntrospectDaemonConfiguration);
