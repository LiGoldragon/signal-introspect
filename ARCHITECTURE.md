# signal-persona-introspect — architecture

*Central Signal envelope contract for Persona introspection.*

## 0 · TL;DR

`signal-persona-introspect` is the wrapper-and-selector contract a
client uses to ask `persona-introspect` for an engine observation.
It defines `IntrospectionRequest`, `IntrospectionReply`, the targets
and scopes a query may name, and the typed roll-up records that
project peer-component observations to a human-facing surface.

It is **not** a shared row bucket — component-specific observation
records start in the component contract that owns the state
(`signal-persona-terminal`, `signal-persona-router`, etc.). This
crate wraps; it does not redefine.

Wire enums are closed. The "not yet observed" axis lives on
`Option<>` wrappers on the carrier records (`ComponentSnapshot`,
`DeliveryTrace`, `PrototypeWitness`); the inner status enums
(`ComponentReadiness`, `DeliveryTraceStatus`) stay closed and never
carry an `Unknown` placeholder.

## 1 · Channel

| Side | Component |
|---|---|
| Request side | Introspection clients (CLIs, agent tooling). |
| Reply side | `persona-introspect` |

Today's surface is one-shot `Match` queries. Streaming subscription
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
  records wrap it as `Option<DeliveryTraceStatus>` when no trace has
  arrived.
- `IntrospectionUnimplementedReason` and `IntrospectionDeniedReason`
  (closed positive rejection causes).
- Query records for engine snapshot, component snapshot, delivery
  trace, and the prototype rollup `PrototypeWitness`.
- Reply records that wrap or summarize observations for projection.
- Signal root-verb mapping for every request variant via
  `signal_channel!`-generated `IntrospectionRequest::signal_verb()`.

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
the inner enum as `Option<…>`:

```text
ComponentSnapshot  | readiness:        Option<ComponentReadiness>
DeliveryTrace      | status:           Option<DeliveryTraceStatus>
PrototypeWitness   | manager_seen:     Option<ComponentReadiness>
PrototypeWitness   | router_seen:      Option<ComponentReadiness>
PrototypeWitness   | terminal_seen:    Option<ComponentReadiness>
PrototypeWitness   | delivery_status:  Option<DeliveryTraceStatus>
```

`None` means *"the daemon has not yet collected an observation from
that peer."* `Some(state)` means *"this is the closed observation."*
The distinction is structural; consumers pattern-match on the
`Option` shape, not on a sentinel inside a present value.

## 4 · Signal root verbs

Every `IntrospectionRequest` variant declares its root verb in the
`signal_channel!` declaration. `signal-core` generates
`IntrospectionRequest::signal_verb()` and
`IntrospectionRequest::into_request()`. All current variants are
read-shaped:

```text
EngineSnapshot       -> Match
ComponentSnapshot    -> Match
DeliveryTrace        -> Match
PrototypeWitness     -> Match
```

When `SubscribeComponent` lands, it maps to `Subscribe` and opens a
typed event stream with a request-side retraction variant and a
reply-side acknowledgement.

## 5 · Constraints

| Constraint | Witness |
|---|---|
| The central contract asks and wraps; it does not define component rows. | Public type review: no router/terminal/manager row vocabulary defined here. Source-scan witness names "central contract does not define peer rows." |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests per variant. |
| Every `IntrospectionRequest` variant declares a Signal root verb. | The `signal_channel!` declaration names each root; `signal-core` generates the verb-mapping methods; round-trip tests assert verb+payload alignment. |
| Read-shaped payloads use `Match` or `Subscribe`; write-shaped payloads use `Assert` / `Mutate` / `Retract`. | Today all variants are `Match`; the `signal_channel!` declaration is the witness. |
| NOTA derives live on the same typed records. | Cargo tests compile `NotaRecord`, `NotaEnum`, and `NotaTransparent` derives; canonical examples round-trip the text form. |
| The contract contains no daemon code. | Source scan: no Kameo, Tokio, socket, or redb code. |
| Wire enums contain no `Unknown` variant. | `tests/round_trip.rs::introspection_status_enums_are_closed_no_unknown_variants` exhaustively matches every `ComponentReadiness` and `DeliveryTraceStatus` variant. Adding an `Unknown` variant breaks the match. |
| Any record name containing the word `Unknown` represents a positive "entity not in our state" rejection, not a polling-shape escape hatch. | This crate has no `Unknown*` record names today; the "not observed yet" axis lives on `Option<>` wrappers on the carrier records. |
| The "not yet observed" axis lives on `Option<>` wrappers, never inside a closed status enum. | `prototype_witness_reply_round_trips_with_no_observations_yet` exercises the all-`None` carrier shape end-to-end through the length-prefixed frame. |
| Every `signal_channel!` request variant has a typed `signal_verb()` mapping. | The macro generates `signal_verb()`; round-trip tests assert each variant maps to `SignalVerb::Match` for current scope. |
| Round-trip witnesses cover every variant in rkyv. | `tests/round_trip.rs` exercises every request and reply variant through `Frame::encode_length_prefixed` / `decode_length_prefixed`. |
| Round-trip witnesses cover every variant in NOTA. | `examples/canonical.nota` holds one canonical text example per request/reply variant; round-trip tests parse and re-emit each. |
| No stringly-typed dispatch (`match s.as_str()`) for closed-set states. | All status/scope/reason fields are typed closed enums. |
| Contract crate dependencies use a named API reference (branch or tag), not a raw revision pin. | `Cargo.toml` review: `signal-core` and downstream contract crates declare `git = "..."` with a named-branch shape; raw `rev = "..."` pins are not used. |

## 6 · NOTA codec quirk on `signal_channel!` payload heads

The `signal_channel!` macro emits a request variant's NOTA head as
the **payload's record head**, not the Rust variant name. For
example, `IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery { .. })`
encodes as `(PrototypeWitnessQuery (...))`, not `(PrototypeWitness (...))`.
Canonical examples and round-trip tests carry the payload heads.

## 7 · Status

The crate is the central envelope vocabulary today. The
`ComponentObservations` and `ListRecordKinds` envelope extensions
land alongside their owning component contracts. `SubscribeComponent`
lands once `sema-engine` per-peer commit-then-emit semantics are
declared; until then this crate has no subscription variants and no
`Unimplemented`-stub variant that would force consumers to write
shadow code for a missing feature.

## 8 · Non-ownership

- No introspection daemon — that is `persona-introspect`.
- No router tables, terminal session records, manager event-log
  rows, message ingress ledgers, harness lifecycle records.
- No component databases, actors, sockets, reducers, or redaction
  policy.

Component-specific observation records start in the component
contract that owns the state. This crate wraps; it does not define.
Split to a sibling introspection contract
(`signal-persona-<X>-introspect`) only when the observation
vocabulary becomes heavy or high-churn — per
`~/primary/skills/contract-repo.md` §"Contracts name a component's
wire surface".

## 9 · Code map

```text
src/
└── lib.rs                — payloads + signal_channel! invocation
examples/
└── canonical.nota         — one canonical example per request/reply variant
tests/
└── round_trip.rs          — per-variant frame round trips + NOTA witnesses
                             + closed-enum + verb-mapping witnesses
                             + canonical examples parser
```

## See also

- `signal-core/src/channel.rs` — the macro.
- `signal-persona-router/ARCHITECTURE.md` — router observation rows
  this crate wraps via `Option<DeliveryTraceStatus>` carriers.
- `signal-persona-terminal/ARCHITECTURE.md` — terminal observation
  rows this crate wraps via `ComponentObservationResult`.
