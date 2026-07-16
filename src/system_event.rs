use std::hash::{Hash, Hasher};

use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Maximum retained payload size after structured extraction and redaction.
pub const MAXIMUM_BOUNDED_PAYLOAD_BYTES: usize = 512;

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct BootIdentifier {
    high: u64,
    low: u64,
}

impl BootIdentifier {
    pub const fn new(high: u64, low: u64) -> Self {
        Self { high, low }
    }

    pub const fn high(&self) -> u64 {
        self.high
    }

    pub const fn low(&self) -> u64 {
        self.low
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
    PartialOrd,
    Ord,
    Hash,
)]
pub struct EventIdentifier(u64);

impl EventIdentifier {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
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
    PartialOrd,
    Ord,
    Hash,
)]
pub struct EventInstant(u64);

impl EventInstant {
    pub const fn new(monotonic_microseconds: u64) -> Self {
        Self(monotonic_microseconds)
    }

    pub const fn value(self) -> u64 {
        self.0
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
pub struct ExtractorRevision(u32);

impl ExtractorRevision {
    pub const fn new(value: u32) -> Self {
        Self(value)
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
pub struct PolicyRevision(u32);

impl PolicyRevision {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

/// A short UTF-8 payload that has already passed structured allowlisting and redaction.
///
/// The constructor retains no unbounded fallback. `text` is always at most 512 bytes,
/// and truncation stops before a UTF-8 continuation byte.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct BoundedPayload {
    text: String,
    truncated: bool,
    original_byte_length: u64,
}

impl BoundedPayload {
    /// Bound text produced by a domain extractor after allowlisting and redaction.
    pub fn from_redacted_allowlisted(text: impl AsRef<str>) -> Self {
        let text = text.as_ref();
        let original_byte_length = text.len() as u64;
        let mut retained_byte_length = text.len().min(MAXIMUM_BOUNDED_PAYLOAD_BYTES);
        while !text.is_char_boundary(retained_byte_length) {
            retained_byte_length -= 1;
        }
        Self {
            text: text[..retained_byte_length].to_owned(),
            truncated: retained_byte_length < text.len(),
            original_byte_length,
        }
    }

    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }

    pub const fn truncated(&self) -> bool {
        self.truncated
    }

    pub const fn original_byte_length(&self) -> u64 {
        self.original_byte_length
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
pub enum EventSeverity {
    Debug,
    Information,
    Notice,
    Warning,
    Error,
    Critical,
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
pub enum ProvenanceTrust {
    TrustedJournalMetadata,
    TrustedConnectionMetadata,
    UntrustedApplicationMetadata,
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
pub enum JournalSource {
    ExecutableBluetoothDaemon,
    SystemdBluetoothService,
    MessageIdentifierBluetooth,
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
pub enum ApplicationSource {
    BluetoothDaemon,
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
pub enum EventSource {
    Journal(JournalSource),
    BusConnection,
    Application(ApplicationSource),
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
pub struct EventProvenance {
    pub source: EventSource,
    pub trust: ProvenanceTrust,
}

impl EventProvenance {
    pub const fn trusted_journal(source: JournalSource) -> Self {
        Self {
            source: EventSource::Journal(source),
            trust: ProvenanceTrust::TrustedJournalMetadata,
        }
    }

    pub const fn untrusted_application(source: ApplicationSource) -> Self {
        Self {
            source: EventSource::Application(source),
            trust: ProvenanceTrust::UntrustedApplicationMetadata,
        }
    }

    fn is_consistent(self) -> bool {
        matches!(
            (self.source, self.trust),
            (
                EventSource::Journal(_),
                ProvenanceTrust::TrustedJournalMetadata
            ) | (
                EventSource::BusConnection,
                ProvenanceTrust::TrustedConnectionMetadata
            ) | (
                EventSource::Application(_),
                ProvenanceTrust::UntrustedApplicationMetadata
            )
        )
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
pub enum SystemEventDomain {
    Hardware,
    ServiceControl,
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
pub enum BluetoothTarget {
    Controller,
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
pub enum BluetoothPowerEvent {
    RequestedOn,
    RequestedOff,
    ObservedOn,
    ObservedOff,
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
pub enum BluetoothPowerError {
    RequestRejected,
    StateUnavailable,
    ControllerUnavailable,
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
pub enum TargetedUnclassifiedStatus {
    Counted,
    Redacted,
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
pub enum BluetoothPowerObservation {
    Event(BluetoothPowerEvent),
    Error(BluetoothPowerError),
    Unclassified(TargetedUnclassifiedStatus),
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
pub enum BluetoothTopic {
    Power(BluetoothPowerObservation),
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
pub struct BluetoothSystemEvent {
    pub target: BluetoothTarget,
    pub topic: BluetoothTopic,
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
pub enum SystemdTarget {
    BluetoothService,
    IntrospectService,
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
pub enum ServiceLifecycleEvent {
    Starting,
    Started,
    Stopping,
    Stopped,
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
pub enum ServiceLifecycleError {
    StartFailed,
    StopFailed,
    WatchdogExpired,
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
pub enum ServiceLifecycleObservation {
    Event(ServiceLifecycleEvent),
    Error(ServiceLifecycleError),
    Unclassified(TargetedUnclassifiedStatus),
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
pub enum SystemdTopic {
    Lifecycle(ServiceLifecycleObservation),
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
pub struct SystemdSystemEvent {
    pub target: SystemdTarget,
    pub topic: SystemdTopic,
}

/// Recursive domain → target → topic → curated event/error classification.
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
pub enum TargetedSystemEvent {
    Bluetooth(BluetoothSystemEvent),
    Systemd(SystemdSystemEvent),
}

impl TargetedSystemEvent {
    pub const fn domain(self) -> SystemEventDomain {
        match self {
            Self::Bluetooth(_) => SystemEventDomain::Hardware,
            Self::Systemd(_) => SystemEventDomain::ServiceControl,
        }
    }

    pub const fn is_unclassified(self) -> bool {
        matches!(
            self,
            Self::Bluetooth(BluetoothSystemEvent {
                topic: BluetoothTopic::Power(BluetoothPowerObservation::Unclassified(_)),
                ..
            }) | Self::Systemd(SystemdSystemEvent {
                topic: SystemdTopic::Lifecycle(ServiceLifecycleObservation::Unclassified(_)),
                ..
            })
        )
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SystemEvent {
    pub identifier: EventIdentifier,
    pub boot: BootIdentifier,
    pub observed_at: EventInstant,
    pub classification: TargetedSystemEvent,
    pub severity: EventSeverity,
    pub provenance: EventProvenance,
    pub extractor_revision: ExtractorRevision,
    pub policy_revision: PolicyRevision,
    pub payload: Option<BoundedPayload>,
}

impl SystemEvent {
    pub fn validate(&self) -> Result<(), SystemEventValidationError> {
        if !self.provenance.is_consistent() {
            return Err(SystemEventValidationError::InconsistentProvenance);
        }
        if self.classification.is_unclassified() && self.payload.is_some() {
            return Err(SystemEventValidationError::UnclassifiedPayload);
        }
        Ok(())
    }

    pub fn exact_duplicate_identity(&self) -> ExactDuplicateIdentity {
        ExactDuplicateIdentity {
            boot: self.boot.clone(),
            classification: self.classification,
            severity: self.severity,
            provenance: self.provenance,
            extractor_revision: self.extractor_revision,
            policy_revision: self.policy_revision,
            payload: self.payload.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum SystemEventValidationError {
    #[error("event source and trust classification are inconsistent")]
    InconsistentProvenance,
    #[error("targeted unclassified events may retain counters/status only, never payload text")]
    UnclassifiedPayload,
}

/// Exact duplicate identity. Event identifier and time are deliberately excluded.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExactDuplicateIdentity {
    boot: BootIdentifier,
    classification: TargetedSystemEvent,
    severity: EventSeverity,
    provenance: EventProvenance,
    extractor_revision: ExtractorRevision,
    policy_revision: PolicyRevision,
    payload: Option<BoundedPayload>,
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
pub enum CoalescingClosure {
    Active,
    Interval,
    ExplicitFlush,
    Shutdown,
    Eviction,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CoalescedSystemEvent {
    pub representative: SystemEvent,
    pub count: u64,
    pub first_seen: EventInstant,
    pub last_seen: EventInstant,
    pub suppressed_count: u64,
    pub policy_revision: PolicyRevision,
    pub closure: CoalescingClosure,
}

impl CoalescedSystemEvent {
    pub fn new(representative: SystemEvent, closure: CoalescingClosure) -> Self {
        let observed_at = representative.observed_at;
        let policy_revision = representative.policy_revision;
        Self {
            representative,
            count: 1,
            first_seen: observed_at,
            last_seen: observed_at,
            suppressed_count: 0,
            policy_revision,
            closure,
        }
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
)]
pub struct ExactCoalescingStatus {
    pub active_keys: u64,
    pub evictions: u64,
    pub maximum_active_keys: u64,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RecordSystemEvent {
    pub event: SystemEvent,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SystemEventsQuery {
    pub boot: BootIdentifier,
    pub domain: Option<SystemEventDomain>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct FlushSystemEvents {
    pub boot: BootIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SystemEventSummaries(Vec<CoalescedSystemEvent>);

impl SystemEventSummaries {
    pub fn new(summaries: Vec<CoalescedSystemEvent>) -> Self {
        Self(summaries)
    }

    pub fn as_slice(&self) -> &[CoalescedSystemEvent] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<CoalescedSystemEvent> {
        self.0
    }
}

impl From<Vec<CoalescedSystemEvent>> for SystemEventSummaries {
    fn from(summaries: Vec<CoalescedSystemEvent>) -> Self {
        Self::new(summaries)
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SystemEvents {
    pub boot: BootIdentifier,
    pub summaries: SystemEventSummaries,
    pub coalescing: ExactCoalescingStatus,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SystemEventAccepted {
    pub representative_identifier: EventIdentifier,
    pub count: u64,
    pub suppressed_count: u64,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SystemEventsFlushed {
    pub boot: BootIdentifier,
    pub summaries: SystemEventSummaries,
}

impl Hash for SystemEvent {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.exact_duplicate_identity().hash(state);
    }
}
