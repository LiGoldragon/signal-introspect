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
pub use signal_persona_message::MessageSlot as MessageIdentifier;
use signal_persona_origin::{ComponentName, EngineIdentifier, OwnerIdentity};

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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineSnapshotQuery {
    pub engine: EngineIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ComponentSnapshotQuery {
    pub engine: EngineIdentifier,
    pub target: IntrospectionTarget,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTraceQuery {
    pub engine: EngineIdentifier,
    pub message_identifier: MessageIdentifier,
    pub originator: ComponentName,
}

impl DeliveryTraceQuery {
    pub fn join_key(&self) -> DeliveryTraceJoinKey {
        DeliveryTraceJoinKey::new(
            self.engine.clone(),
            self.message_identifier,
            self.originator,
        )
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct PrototypeWitnessQuery {
    pub engine: EngineIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EngineSnapshot {
    pub engine: EngineIdentifier,
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
    pub engine: EngineIdentifier,
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

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct HopIndex(u32);

impl HopIndex {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub const fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// Join portion of [`DeliveryTraceKey`]. All hops for one delivery
/// share this value; `DeliveryTraceKey.hop_index` orders the events
/// inside the joined chain.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTraceJoinKey {
    pub engine: EngineIdentifier,
    pub message_identifier: MessageIdentifier,
    pub originator: ComponentName,
}

impl DeliveryTraceJoinKey {
    pub fn new(
        engine: EngineIdentifier,
        message_identifier: MessageIdentifier,
        originator: ComponentName,
    ) -> Self {
        Self {
            engine,
            message_identifier,
            originator,
        }
    }

    pub fn matches_query(&self, query: &DeliveryTraceQuery) -> bool {
        self == &query.join_key()
    }
}

/// Cross-component delivery correlation key. The first three fields are
/// the join key; `hop_index` is the deterministic order key inside one
/// delivery chain.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTraceKey {
    pub engine: EngineIdentifier,
    pub message_identifier: MessageIdentifier,
    pub originator: ComponentName,
    pub hop_index: HopIndex,
}

impl DeliveryTraceKey {
    pub fn new(
        engine: EngineIdentifier,
        message_identifier: MessageIdentifier,
        originator: ComponentName,
        hop_index: HopIndex,
    ) -> Self {
        Self {
            engine,
            message_identifier,
            originator,
            hop_index,
        }
    }

    pub fn matches_query(&self, query: &DeliveryTraceQuery) -> bool {
        self.join_key().matches_query(query)
    }

    pub fn join_key(&self) -> DeliveryTraceJoinKey {
        DeliveryTraceJoinKey::new(
            self.engine.clone(),
            self.message_identifier,
            self.originator,
        )
    }

    pub fn next_hop(&self) -> Self {
        Self::new(
            self.engine.clone(),
            self.message_identifier,
            self.originator,
            self.hop_index.next(),
        )
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTraceEvent {
    pub key: DeliveryTraceKey,
    pub component: ComponentName,
    pub status: DeliveryTraceStatus,
}

impl DeliveryTraceEvent {
    pub fn new(
        key: DeliveryTraceKey,
        component: ComponentName,
        status: DeliveryTraceStatus,
    ) -> Self {
        Self {
            key,
            component,
            status,
        }
    }

    pub fn key(&self) -> &DeliveryTraceKey {
        &self.key
    }
}

/// Hop-ordered delivery observations for one message chain. An empty
/// `events` vector means the introspect daemon has not yet seen any Tap
/// event for the selected join key.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct DeliveryTrace {
    pub engine: EngineIdentifier,
    pub message_identifier: MessageIdentifier,
    pub originator: ComponentName,
    pub events: Vec<DeliveryTraceEvent>,
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
    pub engine: EngineIdentifier,
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
