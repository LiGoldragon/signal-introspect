#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum IntrospectionTarget {
    EngineManager,
    Mind,
    Message,
    Router,
    Spirit,
    System,
    Harness,
    Terminal,
    Introspect,
    Signal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum IntrospectionScope {
    EngineSnapshot,
    ComponentSnapshot,
    DeliveryTrace,
    PrototypeWitness,
}
#[rustfmt::skip]
pub type ObservedComponents = std::vec::Vec<IntrospectionTarget>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EngineSnapshotObservation {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub observed_components: ObservedComponents,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ComponentReadiness {
    Ready,
    NotReady,
}
#[rustfmt::skip]
pub type OptionalComponentReadiness = std::option::Option<ComponentReadiness>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentSnapshotObservation {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub introspection_target: IntrospectionTarget,
    pub optional_component_readiness: OptionalComponentReadiness,
}
#[rustfmt::skip]
pub type TraceEventName = String;
#[rustfmt::skip]
pub type TraceSequence = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TraceLayer {
    Signal,
    Nexus,
    Sema,
    Authorization,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentTraceEvent {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub introspection_target: IntrospectionTarget,
    pub trace_layer: TraceLayer,
    pub trace_event_name: TraceEventName,
    pub trace_sequence: TraceSequence,
}
#[rustfmt::skip]
pub type OptionalTraceEventName = std::option::Option<TraceEventName>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentTraceQuery {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub introspection_target: IntrospectionTarget,
    pub optional_trace_event_name: OptionalTraceEventName,
}
#[rustfmt::skip]
pub type ComponentTraceEvents = std::vec::Vec<ComponentTraceEvent>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentTrace {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub introspection_target: IntrospectionTarget,
    pub component_trace_events: ComponentTraceEvents,
}
#[rustfmt::skip]
pub type HopIndex = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryTraceObservationJoinKey {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub message_slot: signal_message::MessageSlot,
    pub component_name: signal_persona::ComponentName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryTraceObservationKey {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub message_slot: signal_message::MessageSlot,
    pub component_name: signal_persona::ComponentName,
    pub hop_index: HopIndex,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeliveryTraceObservationStatus {
    Accepted,
    Routed,
    Delivered,
    Deferred,
    Failed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryTraceObservationEvent {
    pub delivery_trace_observation_key: DeliveryTraceObservationKey,
    pub component_name: signal_persona::ComponentName,
    pub delivery_trace_observation_status: DeliveryTraceObservationStatus,
}
#[rustfmt::skip]
pub type DeliveryTraceObservationEvents = std::vec::Vec<DeliveryTraceObservationEvent>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryTraceObservationQuery {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub message_slot: signal_message::MessageSlot,
    pub component_name: signal_persona::ComponentName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DeliveryTraceObservation {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub message_slot: signal_message::MessageSlot,
    pub component_name: signal_persona::ComponentName,
    pub delivery_trace_observation_events: DeliveryTraceObservationEvents,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PrototypeWitnessObservationQuery {
    pub engine_identifier: signal_persona::EngineIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PrototypeWitnessObservation {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub first_optional_component_readiness: OptionalComponentReadiness,
    pub second_optional_component_readiness: OptionalComponentReadiness,
    pub third_optional_component_readiness: OptionalComponentReadiness,
    pub delivery_trace_observation_status_option: Option<DeliveryTraceObservationStatus>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct IntrospectionUnimplemented {
    pub introspection_scope: IntrospectionScope,
    pub unimplemented_reason: UnimplementedReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum UnimplementedReason {
    NotInPrototypeScope,
    ComponentObservationMissing,
    SubscriptionNotImplemented,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct IntrospectionDenied {
    pub introspection_scope: IntrospectionScope,
    pub denied_reason: DeniedReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DeniedReason {
    NotAuthorized,
    Redacted,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BootIdentifier {
    pub first_integer: i64,
    pub second_integer: i64,
}
#[rustfmt::skip]
pub type EventIdentifier = i64;
#[rustfmt::skip]
pub type EventInstant = i64;
#[rustfmt::skip]
pub type ExtractorRevision = i64;
#[rustfmt::skip]
pub type PolicyRevision = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BoundedPayload {
    pub string: String,
    pub boolean: bool,
    pub integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EventSeverity {
    Debug,
    Information,
    Notice,
    Warning,
    Error,
    Critical,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ProvenanceTrust {
    TrustedJournalMetadata,
    TrustedConnectionMetadata,
    UntrustedApplicationMetadata,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum JournalSource {
    ExecutableBluetoothDaemon,
    SystemdBluetoothService,
    MessageIdentifierBluetooth,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ApplicationSource {
    BluetoothDaemon,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EventSource {
    Journal(JournalSource),
    BusConnection,
    Application(ApplicationSource),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EventProvenance {
    pub event_source: EventSource,
    pub provenance_trust: ProvenanceTrust,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SystemEventDomain {
    Hardware,
    ServiceControl,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BluetoothTarget {
    Controller,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BluetoothPowerEvent {
    RequestedOn,
    RequestedOff,
    ObservedOn,
    ObservedOff,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BluetoothPowerError {
    RequestRejected,
    StateUnavailable,
    ControllerUnavailable,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TargetedUnclassifiedStatus {
    Counted,
    Redacted,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BluetoothPowerObservation {
    Event(BluetoothPowerEvent),
    Error(BluetoothPowerError),
    Unclassified(TargetedUnclassifiedStatus),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BluetoothTopic {
    Power(BluetoothPowerObservation),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BluetoothSystemEvent {
    pub bluetooth_target: BluetoothTarget,
    pub bluetooth_topic: BluetoothTopic,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SystemdTarget {
    BluetoothService,
    IntrospectService,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ServiceLifecycleEvent {
    Starting,
    Started,
    Stopping,
    Stopped,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ServiceLifecycleError {
    StartFailed,
    StopFailed,
    WatchdogExpired,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ServiceLifecycleObservation {
    Event(ServiceLifecycleEvent),
    Error(ServiceLifecycleError),
    Unclassified(TargetedUnclassifiedStatus),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SystemdTopic {
    Lifecycle(ServiceLifecycleObservation),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemdSystemEvent {
    pub systemd_target: SystemdTarget,
    pub systemd_topic: SystemdTopic,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TargetedSystemEvent {
    Bluetooth(BluetoothSystemEvent),
    Systemd(SystemdSystemEvent),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemEvent {
    pub event_identifier: EventIdentifier,
    pub boot_identifier: BootIdentifier,
    pub event_instant: EventInstant,
    pub targeted_system_event: TargetedSystemEvent,
    pub event_severity: EventSeverity,
    pub event_provenance: EventProvenance,
    pub extractor_revision: ExtractorRevision,
    pub policy_revision: PolicyRevision,
    pub bounded_payload_option: Option<BoundedPayload>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CoalescingClosure {
    Active,
    Interval,
    ExplicitFlush,
    Shutdown,
    Eviction,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CoalescedSystemEvent {
    pub system_event: SystemEvent,
    pub first_integer: i64,
    pub first_event_instant: EventInstant,
    pub second_event_instant: EventInstant,
    pub second_integer: i64,
    pub policy_revision: PolicyRevision,
    pub coalescing_closure: CoalescingClosure,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ExactCoalescingStatus {
    pub first_integer: i64,
    pub second_integer: i64,
    pub third_integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RecordSystemEvent {
    pub system_event: SystemEvent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemEventsQuery {
    pub boot_identifier: BootIdentifier,
    pub system_event_domain_option: Option<SystemEventDomain>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FlushSystemEvents {
    pub boot_identifier: BootIdentifier,
}
#[rustfmt::skip]
pub type SystemEventSummaries = std::vec::Vec<CoalescedSystemEvent>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemEvents {
    pub boot_identifier: BootIdentifier,
    pub system_event_summaries: SystemEventSummaries,
    pub exact_coalescing_status: ExactCoalescingStatus,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemEventAccepted {
    pub event_identifier: EventIdentifier,
    pub first_integer: i64,
    pub second_integer: i64,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SystemEventsFlushed {
    pub boot_identifier: BootIdentifier,
    pub system_event_summaries: SystemEventSummaries,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EngineSnapshotObservationQuery {
    pub engine_identifier: signal_persona::EngineIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentSnapshotObservationQuery {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub introspection_target: IntrospectionTarget,
}
#[rustfmt::skip]
pub type WirePath = String;
#[rustfmt::skip]
pub type SocketMode = i64;
#[rustfmt::skip]
pub type IntrospectSocketPath = WirePath;
#[rustfmt::skip]
pub type IntrospectSocketMode = SocketMode;
#[rustfmt::skip]
pub type SupervisionSocketPath = WirePath;
#[rustfmt::skip]
pub type SupervisionSocketMode = SocketMode;
#[rustfmt::skip]
pub type StorePath = WirePath;
#[rustfmt::skip]
pub type ManagerSocketPath = WirePath;
#[rustfmt::skip]
pub type RouterSocketPath = WirePath;
#[rustfmt::skip]
pub type TerminalSocketPath = WirePath;
#[rustfmt::skip]
pub type TraceSocketPath = WirePath;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct IntrospectDaemonConfiguration {
    pub introspect_socket_path: IntrospectSocketPath,
    pub introspect_socket_mode: IntrospectSocketMode,
    pub supervision_socket_path: SupervisionSocketPath,
    pub supervision_socket_mode: SupervisionSocketMode,
    pub store_path: StorePath,
    pub manager_socket_path: ManagerSocketPath,
    pub router_socket_path: RouterSocketPath,
    pub terminal_socket_path: TerminalSocketPath,
    pub trace_socket_path: TraceSocketPath,
    pub owner_identity: signal_persona::OwnerIdentity,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    EngineSnapshotObservation(EngineSnapshotObservationQuery),
    ComponentSnapshotObservation(ComponentSnapshotObservationQuery),
    DeliveryTraceObservation(DeliveryTraceObservationQuery),
    PrototypeWitnessObservation(PrototypeWitnessObservationQuery),
    ComponentTrace(ComponentTraceQuery),
    RecordSystemEvent(RecordSystemEvent),
    SystemEvents(SystemEventsQuery),
    FlushSystemEvents(FlushSystemEvents),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    EngineSnapshotObservation(EngineSnapshotObservation),
    ComponentSnapshotObservation(ComponentSnapshotObservation),
    DeliveryTraceObservation(DeliveryTraceObservation),
    PrototypeWitnessObservation(PrototypeWitnessObservation),
    ComponentTrace(ComponentTrace),
    SystemEventAccepted(SystemEventAccepted),
    SystemEvents(SystemEvents),
    SystemEventsFlushed(SystemEventsFlushed),
    Unimplemented(IntrospectionUnimplemented),
    Denied(IntrospectionDenied),
}
