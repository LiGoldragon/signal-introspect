//! Schema-derived Signal contract for Persona introspection query and
//! projection envelopes.
//!
//! This crate asks and wraps observations. Component-owned observation
//! records live in the component contract that owns the observed state
//! (`signal-persona`, `signal-terminal`, `signal-router`, etc.). This crate
//! must not become a bucket for every component's internal rows.

#[allow(dead_code, private_interfaces)]
#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

pub type IntrospectionRequest = Input;
pub type IntrospectionReply = Output;
pub type IntrospectionFrame = Frame;
pub type IntrospectionFrameBody = FrameBody;
pub type IntrospectionRequestBuilder = RequestBuilder;
pub type IntrospectionRequestKind = InputRoute;
pub type IntrospectionReplyKind = OutputRoute;

impl Input {
    pub fn kind(&self) -> InputRoute {
        self.route()
    }
}

impl WirePath {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl std::fmt::Display for WirePath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.payload().fmt(formatter)
    }
}

impl AsRef<str> for WirePath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<&str> for WirePath {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl SocketMode {
    pub fn into_u32(self) -> u32 {
        self.into_payload() as u32
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

impl DeliveryTraceQuery {
    pub fn join_key(&self) -> DeliveryTraceJoinKey {
        DeliveryTraceJoinKey::new(
            self.engine_identifier.clone(),
            self.message_identifier.clone(),
            self.component_name.clone(),
        )
    }
}

impl ObservedComponents {
    pub fn as_slice(&self) -> &[IntrospectionTarget] {
        self.payload().as_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.payload().is_empty()
    }
}

impl EngineSnapshot {
    pub fn new(
        engine: EngineIdentifier,
        observed_components: impl Into<ObservedComponents>,
    ) -> Self {
        Self {
            engine_identifier: engine,
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

impl Copy for HopIndex {}

impl HopIndex {
    pub fn value(self) -> u32 {
        self.into_payload()
    }

    pub fn next(self) -> Self {
        Self::new(self.value().saturating_add(1))
    }
}

impl PartialOrd for HopIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HopIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.payload().cmp(other.payload())
    }
}

impl std::hash::Hash for HopIndex {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.payload().hash(state);
    }
}

impl DeliveryTraceJoinKey {
    pub fn new(
        engine: EngineIdentifier,
        message_identifier: MessageIdentifier,
        originator: ComponentName,
    ) -> Self {
        Self {
            engine_identifier: engine,
            message_identifier,
            component_name: originator,
        }
    }

    pub fn matches_query(&self, query: &DeliveryTraceQuery) -> bool {
        self == &query.join_key()
    }
}

impl DeliveryTraceKey {
    pub fn new(
        engine: EngineIdentifier,
        message_identifier: MessageIdentifier,
        originator: ComponentName,
        hop_index: HopIndex,
    ) -> Self {
        Self {
            engine_identifier: engine,
            message_identifier,
            component_name: originator,
            hop_index,
        }
    }

    pub fn matches_query(&self, query: &DeliveryTraceQuery) -> bool {
        self.join_key().matches_query(query)
    }

    pub fn join_key(&self) -> DeliveryTraceJoinKey {
        DeliveryTraceJoinKey::new(
            self.engine_identifier.clone(),
            self.message_identifier.clone(),
            self.component_name.clone(),
        )
    }

    pub fn next_hop(&self) -> Self {
        Self::new(
            self.engine_identifier.clone(),
            self.message_identifier.clone(),
            self.component_name.clone(),
            self.hop_index.next(),
        )
    }
}

impl DeliveryTraceEvent {
    pub fn new(
        key: DeliveryTraceKey,
        component: ComponentName,
        status: DeliveryTraceStatus,
    ) -> Self {
        Self {
            delivery_trace_key: key,
            component_name: component,
            delivery_trace_status: status,
        }
    }

    pub fn key(&self) -> &DeliveryTraceKey {
        &self.delivery_trace_key
    }
}

impl DeliveryTraceEvents {
    pub fn as_slice(&self) -> &[DeliveryTraceEvent] {
        self.payload().as_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.payload().is_empty()
    }
}

impl DeliveryTrace {
    pub fn new(
        engine: EngineIdentifier,
        message_identifier: MessageIdentifier,
        originator: ComponentName,
        events: impl Into<DeliveryTraceEvents>,
    ) -> Self {
        Self {
            engine_identifier: engine,
            message_identifier,
            component_name: originator,
            delivery_trace_events: events.into(),
        }
    }

    pub fn events(&self) -> &[DeliveryTraceEvent] {
        self.delivery_trace_events.as_slice()
    }

    pub fn into_events(self) -> Vec<DeliveryTraceEvent> {
        self.delivery_trace_events.into_payload()
    }
}

impl TraceEventName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl std::fmt::Display for TraceEventName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.payload().fmt(formatter)
    }
}

impl PartialEq<&str> for TraceEventName {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl Copy for TraceSequence {}

impl TraceSequence {
    pub fn value(self) -> u64 {
        self.into_payload()
    }

    pub fn next(self) -> Self {
        Self::new(self.value().saturating_add(1))
    }
}

impl PartialOrd for TraceSequence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TraceSequence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.payload().cmp(other.payload())
    }
}

impl std::hash::Hash for TraceSequence {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.payload().hash(state);
    }
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
            engine_identifier: engine,
            introspection_target: component,
            trace_layer: layer,
            trace_event_name: event_name,
            trace_sequence: sequence,
        }
    }

    pub fn matches_query(&self, query: &ComponentTraceQuery) -> bool {
        self.engine_identifier == query.engine_identifier
            && self.introspection_target == query.introspection_target
            && query
                .optional_trace_event_name
                .as_ref()
                .is_none_or(|name| &self.trace_event_name == name)
    }
}

impl std::fmt::Display for ComponentTraceEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "nota-text")]
        {
            use nota::NotaEncode;
            self.to_nota().fmt(formatter)
        }
        #[cfg(not(feature = "nota-text"))]
        {
            std::fmt::Debug::fmt(self, formatter)
        }
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

impl ComponentTraceQuery {
    pub fn new(
        engine: EngineIdentifier,
        component: IntrospectionTarget,
        event_name: Option<TraceEventName>,
    ) -> Self {
        Self {
            engine_identifier: engine,
            introspection_target: component,
            optional_trace_event_name: event_name,
        }
    }
}

impl ComponentTraceEvents {
    pub fn as_slice(&self) -> &[ComponentTraceEvent] {
        self.payload().as_slice()
    }

    pub fn is_empty(&self) -> bool {
        self.payload().is_empty()
    }
}

impl ComponentTrace {
    pub fn new(
        engine: EngineIdentifier,
        component: IntrospectionTarget,
        events: impl Into<ComponentTraceEvents>,
    ) -> Self {
        Self {
            engine_identifier: engine,
            introspection_target: component,
            component_trace_events: events.into(),
        }
    }

    pub fn events(&self) -> &[ComponentTraceEvent] {
        self.component_trace_events.as_slice()
    }

    pub fn into_events(self) -> Vec<ComponentTraceEvent> {
        self.component_trace_events.into_payload()
    }
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

impl std::hash::Hash for IntrospectionTarget {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl std::hash::Hash for IntrospectionScope {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl std::hash::Hash for ComponentReadiness {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl std::hash::Hash for DeliveryTraceStatus {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl std::hash::Hash for IntrospectionUnimplementedReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl std::hash::Hash for IntrospectionDeniedReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

impl std::hash::Hash for TraceLayer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
