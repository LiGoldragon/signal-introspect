# signal-introspect — architecture

*Central Signal envelope contract for Persona introspection.*

## 0 · TL;DR

`signal-introspect` is the ordinary peer-callable wire contract a
client uses to ask the `introspect` daemon for an engine observation.
It is the central wrapper-and-selector vocabulary: it defines
`Query` and `Response`, the targets and scopes a query may name, and
the typed roll-up records that project peer-component observations
to a human-facing surface. It asks and wraps; the component-specific
observation row types stay in their own owning component contracts,
so this crate never becomes a shared schema bucket. Runtime actors,
the sema-engine store, peer-subscription fan-out, and projection
logic live in `introspect`.

The wire contract — every type, variant, and field name — is
authored once in `ethos/signal.ethos` and generated into
`src/generated/signal.rs` by `ethos-zero`. `build.rs` regenerates the
contract from the `.ethos` source at build time and asserts the
regenerated text matches what is checked in, so the checked-in file
can never silently drift from its source.

This crate owns only wire vocabulary and codecs; it does not own
daemon actors, store tables, sockets, or peer fan-out.

It is **not** a shared row bucket — component-specific observation
records start in the component contract that owns the state
(`signal-terminal`, `signal-router`, etc.). This
crate wraps; it does not redefine.

Wire enums are closed. The "not yet observed" axis lives on
`Option<>` wrappers or an empty vector on carrier records
(`ComponentSnapshotObservation`, `DeliveryTraceObservation`,
`PrototypeWitnessObservation`); the inner status enums
(`ComponentReadiness`, `DeliveryTraceObservationStatus`) stay closed
and never carry an `Unknown` placeholder.

## 1 · Channel

| Side | Component |
|---|---|
| Request side | Introspection clients (CLIs, agent tooling). |
| Reply side | `introspect` |

Today's surface is one-shot observation queries plus the system-event
recording and flush surface. Streaming subscription support lands
when `sema-engine` per-peer commit-then-emit gates allow it; until
then this crate carries no subscription variants.

## 2 · Owned surface

`Query` and `Response` are the two top-level closed enums; every
other type below is a variant payload or a shared building block.

- `Query` variants: `EngineSnapshotObservation`,
  `ComponentSnapshotObservation`, `DeliveryTraceObservation`,
  `PrototypeWitnessObservation`, `ComponentTrace`,
  `RecordSystemEvent`, `SystemEvents`, `FlushSystemEvents`.
- `Response` variants: `EngineSnapshotObservation`,
  `ComponentSnapshotObservation`, `DeliveryTraceObservation`,
  `PrototypeWitnessObservation`, `ComponentTrace`,
  `SystemEventAccepted`, `SystemEvents`, `SystemEventsFlushed`,
  `Unimplemented` (carrying `IntrospectionUnimplemented`), `Denied`
  (carrying `IntrospectionDenied`).
- `IntrospectionTarget` (closed enum of peer-component identities the
  introspect daemon can ask: `EngineManager`, `Mind`, `Message`,
  `Router`, `Spirit`, `System`, `Harness`, `Terminal`, `Introspect`,
  `Signal`).
- `IntrospectionScope` (closed enum of observation shapes:
  `EngineSnapshot`, `ComponentSnapshot`, `DeliveryTrace`,
  `PrototypeWitness`).
- `ComponentReadiness` (closed enum: `Ready` / `NotReady`). The "not
  observed yet" axis lives on `Option<ComponentReadiness>` in carrier
  records.
- `DeliveryTraceObservationStatus` (closed enum: `Accepted` /
  `Routed` / `Delivered` / `Deferred` / `Failed`). Carrier records
  place it on hop-keyed `DeliveryTraceObservationEvent` rows.
- `DeliveryTraceObservationKey` — four-field cross-component
  correlation key: engine identifier, message slot, component name,
  and hop index. The first three fields join one message-delivery
  chain; hop index orders events without clocks.
- `DeliveryTraceObservationJoinKey` — the first-three-field join
  portion of a delivery trace key. Store implementations use it as
  the range-prefix for all hop rows that belong to one delivery.
- `IntrospectionUnimplemented` / `UnimplementedReason` and
  `IntrospectionDenied` / `DeniedReason` (closed positive rejection
  causes).
- `ComponentTraceEvent` / `ComponentTraceQuery` / `ComponentTrace` —
  per-component structured trace events keyed by engine, target, and
  a closed `TraceLayer` (`Signal`, `Nexus`, `Sema`, `Authorization`).
- Query records for engine snapshot, component snapshot, delivery
  trace, and the prototype rollup `PrototypeWitnessObservation`.
- Reply records that wrap or summarize observations for projection.
- A targeted system-event vocabulary: a recursive domain → target →
  topic → curated event/error hierarchy (`TargetedSystemEvent`
  covering `Bluetooth` and `Systemd` targets today); typed
  journal/application provenance and trust (`EventProvenance`,
  `ProvenanceTrust`); extractor and policy revisions; boot-local
  identity (`BootIdentifier`); and a bounded `BoundedPayload` that
  retains a truncation-safe string/boolean/integer payload without a
  free-form fallback body.
- `RecordSystemEvent`, `SystemEventsQuery` / `SystemEvents`,
  `FlushSystemEvents` / `SystemEventsFlushed`, and the
  `CoalescedSystemEvent` / `ExactCoalescingStatus` shapes the daemon
  uses to summarize and flush recorded events per boot.
- `IntrospectDaemonConfiguration` — the typed socket-path and
  socket-mode configuration record for the `introspect` daemon's
  sockets and store path. A typed configuration shape, not daemon
  code.

Typed component targets and trace layers include Spirit authorization
observations: a traced `spirit` daemon exposes the criome
authorization-return point as structured introspection data rather than
an untyped log line.

Request payloads carry the query target and scope only. They do not
mint sequence numbers, snapshot timestamps, or correlation identity
that belongs to the daemon; `introspect` mints those values at the
daemon.

## 3 · Closed-enum integrity

Wire enums on this contract are closed. The "not yet observed" axis
is named on `Option<>` carriers, not by an `Unknown` polling-shape
variant inside the enum.

```text
ComponentReadiness
  | Ready
  | NotReady

DeliveryTraceObservationStatus
  | Accepted
  | Routed
  | Delivered
  | Deferred
  | Failed

UnimplementedReason
  | NotInPrototypeScope
  | ComponentObservationMissing
  | SubscriptionNotImplemented

DeniedReason
  | NotAuthorized
  | Redacted
```

Carrier records that need an observation-not-yet-arrived state wrap
the inner enum as `Option<…>` or use an empty event vector:

```text
ComponentSnapshotObservation | optional_component_readiness: Option<ComponentReadiness>
DeliveryTraceObservation     | delivery_trace_observation_events: Vec<DeliveryTraceObservationEvent>
PrototypeWitnessObservation  | three Option<ComponentReadiness> fields + Option<DeliveryTraceObservationStatus>
```

`None` or `[]` means *"the daemon has not yet collected an
observation from that peer or trace."* `Some(state)` or a
`DeliveryTraceObservationEvent` means *"this is the closed
observation."* The distinction is structural; consumers pattern-match
on the carrier shape, not on a sentinel inside a present value.

## 4 · Sema-class projections (Layer 3)

Each contract-local operation's daemon-side Component Command
projects to a payloadless Sema class. All current operations are
read-shaped:

```text
EngineSnapshotObservation      -> Match
ComponentSnapshotObservation   -> Match
DeliveryTraceObservation       -> Match
PrototypeWitnessObservation    -> Match
ComponentTrace                 -> Match
RecordSystemEvent              -> Assert
SystemEvents                   -> Match
FlushSystemEvents              -> Mutate
Tap (mandatory observability)     -> Subscribe
Untap (mandatory observability)   -> Retract
```

When per-peer commit-then-emit streaming lands, the additional
operation maps to `Subscribe` and opens a typed event stream with a
request-side retraction variant and a reply-side acknowledgement.

The wire form carries the contract-local verb only; the Sema class
label is computed at observation publish time inside the daemon.

## 5 · Constraints

| Constraint | Witness |
|---|---|
| The central contract asks and wraps; it does not define component rows. | Public type review: no router/terminal/manager row vocabulary defined here. |
| Every request/reply travels as a portable rkyv `Signal` frame. | `tests/generated_contract.rs` round-trips `Query`, `Response`, and `ComponentTraceEvent` through `Signalizable` / `Restorable`. |
| The checked-in generated contract never drifts from its `.ethos` source. | `build.rs` regenerates `src/generated/signal.rs` from `ethos/signal.ethos` on every build and asserts byte-for-byte equality with what is checked in. |
| Datom derives live on the same typed records, behind the `datom` feature. | `tests/generated_contract.rs::datom_round_trip_preserves_trace_query` (feature-gated) compiles the `datom_codec::Datomizable` / `Compositional` derives and round-trips canonical datom text. |
| The contract contains no daemon code. | Source scan: no Kameo, Tokio, socket, or storage code. |
| Wire enums contain no `Unknown` variant. | Source review of `ethos/signal.ethos`: `ComponentReadiness` and `DeliveryTraceObservationStatus` are exhaustive, closed enums with no polling-shape variant. |
| Any record name containing the word `Unknown` represents a positive "entity not in our state" rejection, not a polling-shape escape hatch. | This crate has no `Unknown*` record names today; the "not observed yet" axis lives on `Option<>` wrappers on the carrier records. |
| The "not yet observed" axis lives on `Option<>` wrappers, never inside a closed status enum. | `PrototypeWitnessObservation` carries three `Option<ComponentReadiness>` fields and an `Option<DeliveryTraceObservationStatus>` rather than an `Unknown` enum variant. |
| Delivery trace correlation uses the four-field key `(engine, message slot, component, hop index)`. | `DeliveryTraceObservationKey` in `ethos/signal.ethos` names the four fields; `DeliveryTraceObservationJoinKey` names the three-field join prefix. |
| Round trips cover every variant carried through rkyv. | `tests/generated_contract.rs` exercises `Query`/`Response`/`ComponentTraceEvent` through `Signal::from` / `restore`, including a malformed-bytes rejection case. |
| No stringly-typed dispatch (`match s.as_str()`) for closed-set states. | All status/scope/reason fields are typed closed enums. |
| Request payloads carry query target and scope only; they mint no sequence numbers, snapshot timestamps, or correlation identity. | Public type review: request `*Query` records carry no daemon-minted fields; `introspect` supplies those at observation time. |
| Git dependencies resolve to immutable producer identities. | Every Git dependency in `Cargo.toml` declares an exact `rev`; no dependency follows a moving branch. |

## 6 · Datom codec shape

Under the `datom` feature, `datom_codec::Datomizable` and
`datom_codec::Compositional` derive on the same generated records
that carry the rkyv wire form. A `Query::ComponentTrace(...)` value
renders to canonical datom text as a struct-shaped positional record
whose head names the variant, for example
`ComponentTrace{ engine ... }`. The `datom` feature also pulls in the
same feature on `signal-persona` and `signal-message`, since this
contract composes their types (`EngineIdentifier`, `ComponentName`,
`OwnerIdentity`, `MessageSlot`).

## 7 · Data boundary

The targeted event types store no command line, environment, machine identity,
network address, arbitrary path, or free-form correlation identifier. The boot
identifier is a fixed-width typed pair used only to partition a boot. An
unclassified targeted observation validates only with a typed status and no
payload.

## 8 · Status

The crate is the central envelope vocabulary today. `SubscribeComponent`
lands once `sema-engine` per-peer commit-then-emit semantics are
declared; until then this crate has no subscription variants and no
`Unimplemented`-stub variant that would force consumers to write
shadow code for a missing feature.

## 9 · Non-ownership

- No introspection daemon — that is `introspect`.
- No router tables, terminal session records, manager event-log
  rows, message ingress ledgers, harness lifecycle records.
- No component databases, actors, sockets, reducers, or redaction
  policy.

Component-specific observation records start in the component
contract that owns the state. This crate wraps; it does not define.
Split to a sibling introspection contract
(`signal-<X>-introspect`) only when the observation
vocabulary becomes heavy or high-churn — per
`~/primary/skills/contract-repo.md` §"Contracts name a component's
wire surface".

## 10 · Code map

```text
ethos/
└── signal.ethos          — the authored wire contract (types, variants, Query/Response)
src/
├── lib.rs                — Signal<T> / Signalizable / Restorable / ByteViewable, ETHOS constant
└── generated/
    └── signal.rs          — generated from ethos/signal.ethos by ethos-zero (build.rs asserts freshness)
tests/
└── generated_contract.rs  — rkyv round trips per variant + feature-gated datom round trip
```

## See also

- `~/primary/skills/component-triad.md` §"Verbs come in three layers".
- `signal-router/ARCHITECTURE.md` — router observation rows
  this crate wraps as `DeliveryTraceObservationEvent` carriers.
- `signal-terminal/ARCHITECTURE.md` — terminal observation
  rows this crate wraps via `ComponentSnapshotObservation`.
