# signal-introspect — architecture

*Central Signal envelope contract for Persona introspection.*

## 0 · TL;DR

`signal-introspect` is the ordinary peer-callable wire contract a
client uses to ask the `introspect` daemon for an engine observation.
It is the central wrapper-and-selector vocabulary: it defines
`IntrospectionRequest`, `IntrospectionReply`, the targets
and scopes a query may name, and the typed roll-up records that
project peer-component observations to a human-facing surface. It asks
and wraps; the component-specific observation row types stay in their
own owning component contracts, so this crate never becomes a shared
schema bucket. Runtime actors, the sema-engine store, peer-subscription
fan-out, and projection logic live in `introspect`.

## Migration history — signal-frame operation heads (2026-06-07)

The public wire no longer carries `SignalVerb::Match`. Introspection reads and
targeted event admission/flush use bare contract-local operation heads. The
system-event slice appends `RecordSystemEvent`, `SystemEvents`, and
`FlushSystemEvents` after the existing operation variants; existing
`ComponentTraceEvent` archives and operation discriminants are unchanged. Sema
classification is daemon-side projection only.

This crate depends on `signal-frame` for length-prefixed rkyv framing.
It still owns only wire vocabulary and codecs; it does not own daemon
actors, store tables, sockets, or peer fan-out.

It is **not** a shared row bucket — component-specific observation
records start in the component contract that owns the state
(`signal-terminal`, `signal-router`, etc.). This
crate wraps; it does not redefine.

Wire enums are closed. The "not yet observed" axis lives on
`Option<>` wrappers or an empty vector on carrier records
(`ComponentSnapshot`, `DeliveryTrace`, `PrototypeWitness`); the inner status enums
(`ComponentReadiness`, `DeliveryTraceStatus`) stay closed and never
carry an `Unknown` placeholder.

## 1 · Channel

| Side | Component |
|---|---|
| Request side | Introspection clients (CLIs, agent tooling). |
| Reply side | `introspect` |

Today's surface is one-shot observation queries. Streaming subscription
support (`SubscribeComponent`) lands when `sema-engine` per-peer
commit-then-emit gates allow it; until then this crate carries no
subscription variants.

## 2 · Owned surface

- `IntrospectionRequest` / `IntrospectionReply` (closed enums).
- `IntrospectionTarget` (closed enum of peer-component identities the
  introspect daemon can ask).
- `IntrospectionScope` (closed enum of observation shapes).
- `ComponentReadiness` (closed enum: `Ready` / `NotReady`). The "not
  observed yet" axis lives on `Option<ComponentReadiness>` in carrier
  records.
- `DeliveryTraceStatus` (closed enum mirroring
  `signal_persona_router::RouterDeliveryStatus`:
  `Accepted` / `Routed` / `Delivered` / `Deferred` / `Failed`). Carrier
  records place it on hop-keyed `DeliveryTraceEvent` rows.
- `DeliveryTraceKey` — four-field cross-component correlation key:
  `engine`, `message_identifier`, `originator`, and `hop_index`. The
  first three fields join one message-delivery chain; `hop_index`
  orders events without clocks.
- `DeliveryTraceJoinKey` — the first-three-field join portion of a
  delivery trace key. Store implementations use it as the range-prefix
  for all hop rows that belong to one delivery.
- `IntrospectionUnimplementedReason` and `IntrospectionDeniedReason`
  (closed positive rejection causes).
- Query records for engine snapshot, component snapshot, delivery
  trace, and the prototype rollup `PrototypeWitness`.
- Reply records that wrap or summarize observations for projection.
- Contract-local verbs declared in the `signal_channel!` invocation;
  Sema classification (Layer 3) is daemon-side projection only.
- Targeted system-event vocabulary: a recursive domain → target → topic →
  curated event/error hierarchy; typed journal/application provenance and trust;
  extractor and policy revisions; boot-local identity; and a 512-byte bounded
  UTF-8 payload that retains truncation status and original byte length without
  retaining a fallback body.
- Exact-duplicate identity and summary records. Identity excludes timestamps and
  representative identifiers; similarity, sampling, cooldown, token-bucket,
  debounce, and recurring-pattern policy are deliberately not part of this type.

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

DeliveryTraceStatus
  | Accepted
  | Routed
  | Delivered
  | Deferred
  | Failed

IntrospectionUnimplementedReason
  | NotInPrototypeScope
  | ComponentObservationMissing
  | SubscriptionNotImplemented

IntrospectionDeniedReason
  | NotAuthorized
  | Redacted
```

Carrier records that need an observation-not-yet-arrived state wrap
the inner enum as `Option<…>` or use an empty event vector:

```text
ComponentSnapshot  | readiness:        Option<ComponentReadiness>
DeliveryTrace      | events:           Vec<DeliveryTraceEvent>
PrototypeWitness   | manager_seen:     Option<ComponentReadiness>
PrototypeWitness   | router_seen:      Option<ComponentReadiness>
PrototypeWitness   | terminal_seen:    Option<ComponentReadiness>
PrototypeWitness   | delivery_status:  Option<DeliveryTraceStatus>
```

`None` or `[]` means *"the daemon has not yet collected an
observation from that peer or trace."* `Some(state)` or a
`DeliveryTraceEvent` means *"this is the closed observation."* The
distinction is structural; consumers pattern-match on the carrier
shape, not on a sentinel inside a present value.

## 4 · Sema-class projections (Layer 3)

Each contract-local operation's daemon-side Component Command
projects to a payloadless Sema class via `ToSemaOperation`. All
current operations are read-shaped:

```text
EngineSnapshot     -> Match
ComponentSnapshot  -> Match
DeliveryTrace      -> Match
PrototypeWitness   -> Match
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
| The central contract asks and wraps; it does not define component rows. | Public type review: no router/terminal/manager row vocabulary defined here. Source-scan witness names "central contract does not define peer rows." |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests per variant. |
| Every `IntrospectionRequest` variant is a contract-local verb in verb form. | The `signal_channel!` declaration names each verb; round-trip tests assert each variant's NOTA head. Sema classification is daemon-side projection only. |
| Read-shaped payloads project to Sema `Match` / `Subscribe`; write-shaped payloads project to `Assert` / `Mutate` / `Retract`. | Daemon-side `ToSemaOperation` impl is the witness; today all read-shaped operations project to `Match`. |
| NOTA derives live on the same typed records. | Cargo tests compile `nota` `NotaEncode` and `NotaDecode` derives; canonical examples round-trip the text form. |
| The contract contains no daemon code. | Source scan: no Kameo, Tokio, socket, or storage code. |
| Wire enums contain no `Unknown` variant. | `tests/round_trip.rs::introspection_status_enums_are_closed_no_unknown_variants` exhaustively matches every `ComponentReadiness` and `DeliveryTraceStatus` variant. Adding an `Unknown` variant breaks the match. |
| Any record name containing the word `Unknown` represents a positive "entity not in our state" rejection, not a polling-shape escape hatch. | This crate has no `Unknown*` record names today; the "not observed yet" axis lives on `Option<>` wrappers on the carrier records. |
| The "not yet observed" axis lives on `Option<>` wrappers, never inside a closed status enum. | `prototype_witness_reply_round_trips_with_no_observations_yet` exercises the all-`None` carrier shape end-to-end through the length-prefixed frame. |
| Delivery trace correlation uses the four-field key `(engine, message_identifier, originator, hop_index)`. | `tests/round_trip.rs::delivery_trace_key_round_trips_with_four_correlation_fields` proves the key rides the contract reply. |
| Each variant's NOTA head matches the contract-local verb declared in `signal_channel!`. | The macro generates the codec; round-trip tests assert each variant's NOTA head. |
| Round-trip witnesses cover every variant in rkyv. | `tests/round_trip.rs` exercises every request and reply variant through `Frame::encode_length_prefixed` / `decode_length_prefixed`. |
| Round-trip witnesses cover every variant in NOTA. | `examples/canonical.nota` holds one canonical text example per request/reply variant; round-trip tests parse and re-emit each. |
| No stringly-typed dispatch (`match s.as_str()`) for closed-set states. | All status/scope/reason fields are typed closed enums. |
| Request payloads carry query target and scope only; they mint no sequence numbers, snapshot timestamps, or correlation identity. | Public type review: request `*Query` records carry no daemon-minted fields; `introspect` supplies those at observation time. |
| Contract crate dependencies use a named API reference (branch or tag), not a raw revision pin. | `Cargo.toml` review: `signal-frame` and downstream contract crates declare `git = "..."` with a named-branch shape; raw `rev = "..."` pins are not used. |

## 6 · NOTA codec shape on `signal_channel!` operation heads

The `signal_channel!` macro emits a request variant's NOTA head as
the operation head. For example,
`IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery { .. })`
encodes as `(PrototypeWitness (...))`. Canonical examples and
round-trip tests carry the operation heads.

## 7 · Compatibility

The rkyv/Signal enums are closed. Appending variants preserves the archive shape
and discriminants of all pre-existing variants, so current
`ComponentTraceEvent` producers and readers continue to exchange their existing
record unchanged. A new producer must not send a new operation to an old reader:
the codec validates and rejects unknown enum discriminants and cannot preserve
or round-trip an unknown future variant. Compatibility tests therefore witness
old-variant and new-variant round trips separately rather than claiming unknown
variant passthrough.

The targeted event types store no command line, environment, machine identity,
network address, arbitrary path, or free-form correlation identifier. The boot
identifier is a fixed-width typed pair used only to partition a boot. An
unclassified targeted observation validates only with a typed status and no
payload.

## 8 · Status

The crate is the central envelope vocabulary today. The
`ComponentObservations` and `ListRecordKinds` envelope extensions
land alongside their owning component contracts. `SubscribeComponent`
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
src/
└── lib.rs                — payloads + signal_channel! invocation
examples/
└── canonical.nota         — one canonical example per request/reply variant
tests/
└── round_trip.rs          — per-variant frame round trips + NOTA witnesses
                             + closed-enum + operation-head witnesses
                             + canonical examples parser
```

## See also

- `signal-frame/macros/src/validate.rs` — the macro.
- `~/primary/skills/component-triad.md` §"Verbs come in three layers".
- `signal-router/ARCHITECTURE.md` — router observation rows
  this crate wraps as `DeliveryTraceEvent` carriers.
- `signal-terminal/ARCHITECTURE.md` — terminal observation
  rows this crate wraps via `ComponentObservationResult`.
