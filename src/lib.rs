//! Signal contract for Persona introspection query and projection envelopes.
//!
//! This crate asks and wraps observations. Component-owned observation
//! records live in the component contract that owns the observed state
//! (`signal-persona`, `signal-terminal`,
//! `signal-router`, etc.). This crate must not become a bucket
//! for every component's internal rows.

use nota_next::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_message::MessageSlot as MessageIdentifier;
use signal_persona::{ComponentName, EngineIdentifier, OwnerIdentity};

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct WirePath(String);

impl WirePath {
    pub fn new(payload: impl Into<String>) -> Self {
        Self(payload.into())
    }

    pub fn payload(&self) -> &String {
        &self.0
    }

    pub fn into_payload(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl From<String> for WirePath {
    fn from(payload: String) -> Self {
        Self::new(payload)
    }
}

impl std::fmt::Display for WirePath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.payload().fmt(formatter)
    }
}

impl AsRef<str> for WirePath {
    fn as_ref(&self) -> &str {
        self.payload().as_str()
    }
}

impl PartialEq<&str> for WirePath {
    fn eq(&self, other: &&str) -> bool {
        self.payload() == other
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SocketMode(u64);

impl SocketMode {
    pub const fn new(payload: u64) -> Self {
        Self(payload)
    }

    pub const fn payload(&self) -> &u64 {
        &self.0
    }

    pub const fn into_payload(self) -> u64 {
        self.0
    }

    pub const fn into_u32(self) -> u32 {
        self.0 as u32
    }
}

impl From<u64> for SocketMode {
    fn from(payload: u64) -> Self {
        Self::new(payload)
    }
}

impl std::fmt::Display for SocketMode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.payload().fmt(formatter)
    }
}

impl PartialEq<u64> for SocketMode {
    fn eq(&self, other: &u64) -> bool {
        self.payload() == other
    }
}

impl PartialOrd<u64> for SocketMode {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.payload().partial_cmp(other)
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
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
    Signal,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum IntrospectionScope {
    EngineSnapshot,
    ComponentSnapshot,
    DeliveryTrace,
    PrototypeWitness,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EngineSnapshotQuery {
    pub engine: EngineIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentSnapshotQuery {
    pub engine: EngineIdentifier,
    pub target: IntrospectionTarget,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryTraceQuery {
    pub engine: EngineIdentifier,
    pub message_identifier: MessageIdentifier,
    pub originator: ComponentName,
}

impl DeliveryTraceQuery {
    pub fn join_key(&self) -> DeliveryTraceJoinKey {
        DeliveryTraceJoinKey::new(
            self.engine.clone(),
            self.message_identifier.clone(),
            self.originator.clone(),
        )
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PrototypeWitnessQuery {
    pub engine: EngineIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ObservedComponents(Vec<IntrospectionTarget>);

impl ObservedComponents {
    pub fn new(payload: Vec<IntrospectionTarget>) -> Self {
        Self(payload)
    }

    pub fn payload(&self) -> &Vec<IntrospectionTarget> {
        &self.0
    }

    pub fn into_payload(self) -> Vec<IntrospectionTarget> {
        self.0
    }

    pub fn as_slice(&self) -> &[IntrospectionTarget] {
        self.payload().as_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.payload().is_empty()
    }
}

impl From<Vec<IntrospectionTarget>> for ObservedComponents {
    fn from(payload: Vec<IntrospectionTarget>) -> Self {
        Self::new(payload)
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EngineSnapshot {
    pub engine: EngineIdentifier,
    pub observed_components: ObservedComponents,
}

impl EngineSnapshot {
    pub fn new(
        engine: EngineIdentifier,
        observed_components: impl Into<ObservedComponents>,
    ) -> Self {
        Self {
            engine,
            observed_components: observed_components.into(),
        }
    }

    pub fn observed_components(&self) -> &[IntrospectionTarget] {
        self.observed_components.as_slice()
    }

    pub fn into_observed_components(self) -> Vec<IntrospectionTarget> {
        self.observed_components.into_payload()
    }
}

/// Snapshot of one peer component's readiness as observed by the
/// introspect daemon. `readiness` is `None` when the daemon has not yet
/// queried the peer (initial state before any observation lands);
/// `Some(state)` carries the closed observation of that peer's readiness.
/// The `Option<>` carries the "not yet observed" axis so
/// `ComponentReadiness` itself stays closed per ESSENCE
/// §"Perfect specificity at boundaries."
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentSnapshot {
    pub engine: EngineIdentifier,
    pub target: IntrospectionTarget,
    pub readiness: Option<ComponentReadiness>,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ComponentReadiness {
    Ready,
    NotReady,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
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

impl NotaDecode for HopIndex {
    fn from_nota_block(block: &Block) -> Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let index = u32::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
            value: value.to_string(),
        })?;
        Ok(Self(index))
    }
}

impl NotaEncode for HopIndex {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

/// Join portion of [`DeliveryTraceKey`]. All hops for one delivery
/// share this value; `DeliveryTraceKey.hop_index` orders the events
/// inside the joined chain.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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
            self.message_identifier.clone(),
            self.originator.clone(),
        )
    }

    pub fn next_hop(&self) -> Self {
        Self::new(
            self.engine.clone(),
            self.message_identifier.clone(),
            self.originator.clone(),
            self.hop_index.next(),
        )
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
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
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryTraceEvents(Vec<DeliveryTraceEvent>);

impl DeliveryTraceEvents {
    pub fn new(payload: Vec<DeliveryTraceEvent>) -> Self {
        Self(payload)
    }

    pub fn payload(&self) -> &Vec<DeliveryTraceEvent> {
        &self.0
    }

    pub fn into_payload(self) -> Vec<DeliveryTraceEvent> {
        self.0
    }

    pub fn as_slice(&self) -> &[DeliveryTraceEvent] {
        self.payload().as_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.payload().is_empty()
    }
}

impl From<Vec<DeliveryTraceEvent>> for DeliveryTraceEvents {
    fn from(payload: Vec<DeliveryTraceEvent>) -> Self {
        Self::new(payload)
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DeliveryTrace {
    pub engine: EngineIdentifier,
    pub message_identifier: MessageIdentifier,
    pub originator: ComponentName,
    pub events: DeliveryTraceEvents,
}

impl DeliveryTrace {
    pub fn new(
        engine: EngineIdentifier,
        message_identifier: MessageIdentifier,
        originator: ComponentName,
        events: impl Into<DeliveryTraceEvents>,
    ) -> Self {
        Self {
            engine,
            message_identifier,
            originator,
            events: events.into(),
        }
    }

    pub fn events(&self) -> &[DeliveryTraceEvent] {
        self.events.as_slice()
    }

    pub fn into_events(self) -> Vec<DeliveryTraceEvent> {
        self.events.into_payload()
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
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
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PrototypeWitness {
    pub engine: EngineIdentifier,
    pub manager_seen: Option<ComponentReadiness>,
    pub router_seen: Option<ComponentReadiness>,
    pub terminal_seen: Option<ComponentReadiness>,
    pub delivery_status: Option<DeliveryTraceStatus>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct IntrospectionUnimplemented {
    pub scope: IntrospectionScope,
    pub reason: IntrospectionUnimplementedReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum IntrospectionUnimplementedReason {
    NotInPrototypeScope,
    ComponentObservationMissing,
    SubscriptionNotImplemented,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct IntrospectionDenied {
    pub scope: IntrospectionScope,
    pub reason: IntrospectionDeniedReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum IntrospectionDeniedReason {
    NotAuthorized,
    Redacted,
}

// ─── Component-internal trace ingestion ──────────────────────
//
// Pushed actor-boundary trace observations from a single component
// process. Mirrors the [`DeliveryTraceEvent`] vocabulary above:
// component identity + a monotonic order key + a closed
// classification. The wire type is shared by the emitting component
// and the introspect daemon so neither end depends on the other; the
// transport is `triad_runtime::trace`.

/// The schema-derived name of one actor-boundary trace point, e.g.
/// `SignalAdmitted`, `NexusEntered`, `SemaWriteApplied`. Bare-eligible
/// atom at its `String` position.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TraceEventName(String);

impl TraceEventName {
    pub fn new(payload: impl Into<String>) -> Self {
        Self(payload.into())
    }

    pub fn payload(&self) -> &String {
        &self.0
    }

    pub fn into_payload(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl From<String> for TraceEventName {
    fn from(payload: String) -> Self {
        Self::new(payload)
    }
}

impl std::fmt::Display for TraceEventName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.payload().fmt(formatter)
    }
}

impl PartialEq<&str> for TraceEventName {
    fn eq(&self, other: &&str) -> bool {
        self.payload() == other
    }
}

/// Monotonic per-emitter order key for component-internal trace events.
/// Establishes a deterministic replay order inside one emitter the way
/// [`HopIndex`] orders a delivery chain.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct TraceSequence(u64);

impl TraceSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub const fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

impl From<u64> for TraceSequence {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/// Which actor-boundary layer one component-internal trace event came
/// from. Closed: the signal I/O shape axis mentci filters on in the
/// next slice. Positively-named layers only — there is no "unknown"
/// member; an unclassifiable event is simply not emitted.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum TraceLayer {
    Signal,
    Nexus,
    Sema,
}

/// One pushed component-internal trace observation. Mirrors
/// [`DeliveryTraceEvent`]: component identity + ordering + a closed
/// classification. Shared on the wire by the emitting component and the
/// introspect daemon via [`triad_runtime::trace::TraceEventFrame`].
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentTraceEvent {
    pub engine: EngineIdentifier,
    pub component: IntrospectionTarget,
    pub layer: TraceLayer,
    pub event_name: TraceEventName,
    pub sequence: TraceSequence,
}

impl ComponentTraceEvent {
    pub fn new(
        engine: EngineIdentifier,
        component: IntrospectionTarget,
        layer: TraceLayer,
        event_name: TraceEventName,
        sequence: TraceSequence,
    ) -> Self {
        Self {
            engine,
            component,
            layer,
            event_name,
            sequence,
        }
    }

    pub fn matches_query(&self, query: &ComponentTraceQuery) -> bool {
        self.engine == query.engine
            && self.component == query.component
            && query
                .event_name
                .as_ref()
                .is_none_or(|name| &self.event_name == name)
    }
}

impl std::fmt::Display for ComponentTraceEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_nota().fmt(formatter)
    }
}

impl triad_runtime::trace::TraceEventFrame for ComponentTraceEvent {
    fn to_trace_archive(&self) -> Result<Vec<u8>, triad_runtime::trace::TraceError> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|archive| archive.to_vec())
            .map_err(|_| triad_runtime::trace::TraceError::ArchiveEncode)
    }

    fn from_trace_archive(archive: &[u8]) -> Result<Self, triad_runtime::trace::TraceError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(archive)
            .map_err(|_| triad_runtime::trace::TraceError::ArchiveDecode)
    }
}

/// Filter request: by component kind, optionally narrowed to one event
/// name. `event_name: None` returns every event for the component.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentTraceQuery {
    pub engine: EngineIdentifier,
    pub component: IntrospectionTarget,
    pub event_name: Option<TraceEventName>,
}

impl ComponentTraceQuery {
    pub fn new(
        engine: EngineIdentifier,
        component: IntrospectionTarget,
        event_name: Option<TraceEventName>,
    ) -> Self {
        Self {
            engine,
            component,
            event_name,
        }
    }
}

/// Sequence-ordered component-internal trace events for one component.
/// An empty `events` vector means the introspect daemon has not yet
/// seen any pushed event for the selected component.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentTraceEvents(Vec<ComponentTraceEvent>);

impl ComponentTraceEvents {
    pub fn new(payload: Vec<ComponentTraceEvent>) -> Self {
        Self(payload)
    }

    pub fn payload(&self) -> &Vec<ComponentTraceEvent> {
        &self.0
    }

    pub fn into_payload(self) -> Vec<ComponentTraceEvent> {
        self.0
    }

    pub fn as_slice(&self) -> &[ComponentTraceEvent] {
        self.payload().as_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.payload().is_empty()
    }
}

impl From<Vec<ComponentTraceEvent>> for ComponentTraceEvents {
    fn from(payload: Vec<ComponentTraceEvent>) -> Self {
        Self::new(payload)
    }
}

/// Reply: sequence-ordered component-internal trace events for the
/// selected component.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentTrace {
    pub engine: EngineIdentifier,
    pub component: IntrospectionTarget,
    pub events: ComponentTraceEvents,
}

impl ComponentTrace {
    pub fn new(
        engine: EngineIdentifier,
        component: IntrospectionTarget,
        events: impl Into<ComponentTraceEvents>,
    ) -> Self {
        Self {
            engine,
            component,
            events: events.into(),
        }
    }

    pub fn events(&self) -> &[ComponentTraceEvent] {
        self.events.as_slice()
    }

    pub fn into_events(self) -> Vec<ComponentTraceEvent> {
        self.events.into_payload()
    }
}

signal_channel! {
    channel Introspection {
        operation EngineSnapshot(EngineSnapshotQuery),
        operation ComponentSnapshot(ComponentSnapshotQuery),
        operation DeliveryTrace(DeliveryTraceQuery),
        operation PrototypeWitness(PrototypeWitnessQuery),
        operation ComponentTrace(ComponentTraceQuery),
    }
    reply IntrospectionReply {
        EngineSnapshot(EngineSnapshot),
        ComponentSnapshot(ComponentSnapshot),
        DeliveryTrace(DeliveryTrace),
        PrototypeWitness(PrototypeWitness),
        ComponentTrace(ComponentTrace),
        Unimplemented(IntrospectionUnimplemented),
        Denied(IntrospectionDenied),
    }
}

pub type IntrospectionRequest = Operation;
pub type IntrospectionFrame = Frame;
pub type IntrospectionFrameBody = FrameBody;
pub type IntrospectionRequestBuilder = RequestBuilder;

// ─── Daemon configuration ──────────────────────────────────
//
// Typed startup configuration for `introspect-daemon`.
// Human tooling may author this record through NOTA, but the live
// daemon consumes a signal-encoded rkyv archive path. The daemon does
// not parse NOTA.

/// Startup configuration for `introspect-daemon`.
///
/// Replaces the previous `PERSONA_INTROSPECT_SOCKET`,
/// `PERSONA_SOCKET_PATH`, `PERSONA_SOCKET_MODE`,
/// `PERSONA_SUPERVISION_SOCKET_PATH`,
/// `PERSONA_SUPERVISION_SOCKET_MODE`,
/// `PERSONA_INTROSPECT_STORE`, `PERSONA_STATE_PATH`,
/// `PERSONA_MANAGER_SOCKET_PATH`, and the
/// `PERSONA_PEER_*` peer-socket enumeration environment-variable
/// surface.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct IntrospectDaemonConfiguration {
    /// Where the daemon binds its introspection-query Unix socket.
    pub introspect_socket_path: WirePath,
    /// chmod applied to the introspection-query socket after bind.
    pub introspect_socket_mode: SocketMode,
    /// Where the daemon binds its supervision Unix socket.
    pub supervision_socket_path: WirePath,
    /// chmod applied to the supervision socket after bind.
    pub supervision_socket_mode: SocketMode,
    /// Path to the introspect daemon's sema-engine store file.
    pub store_path: WirePath,
    /// Engine manager's supervision socket (peer).
    pub manager_socket_path: WirePath,
    /// Router daemon's domain socket (peer).
    pub router_socket_path: WirePath,
    /// Terminal supervisor's domain socket (peer).
    pub terminal_socket_path: WirePath,
    /// Where the daemon binds the component-trace ingestion Unix socket
    /// that emitting components push `ComponentTraceEvent` frames to.
    /// Empty path disables trace ingestion (mirrors the empty-peer-socket
    /// convention).
    pub trace_socket_path: WirePath,
    /// The engine owner identity passed to the introspect daemon.
    pub owner_identity: OwnerIdentity,
}

impl IntrospectDaemonConfiguration {
    pub fn from_rkyv_bytes(
        bytes: &[u8],
    ) -> Result<Self, IntrospectDaemonConfigurationArchiveError> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(bytes)
            .map_err(|_| IntrospectDaemonConfigurationArchiveError::Decode)
    }

    pub fn to_rkyv_bytes(&self) -> Result<Vec<u8>, IntrospectDaemonConfigurationArchiveError> {
        rkyv::to_bytes::<rkyv::rancor::Error>(self)
            .map(|bytes| bytes.to_vec())
            .map_err(|_| IntrospectDaemonConfigurationArchiveError::Encode)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum IntrospectDaemonConfigurationArchiveError {
    #[error("failed to encode introspect daemon configuration archive")]
    Encode,
    #[error("failed to decode introspect daemon configuration archive")]
    Decode,
}
