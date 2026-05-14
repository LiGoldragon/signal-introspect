# signal-persona-introspect - architecture

*Central Signal envelope contract for Persona introspection.*

## 0. Intent

`signal-persona-introspect` is the contract a client uses to ask
`persona-introspect` for an engine observation. It is a wrapper and selector
contract, not a home for component-owned rows.

`persona-introspect` is prototype-included as the inspection-plane witness.
The operational delivery path stays outside this crate.

## 1. Owned surface

- `IntrospectionRequest`
- `IntrospectionReply`
- `IntrospectionTarget`
- `IntrospectionScope`
- `IntrospectionUnimplementedReason` (including `PeerSocketMissing`,
  `PeerSocketUnreachable`, `ComponentObservationMissing`,
  `AwaitingCorrelationCache`).
- Query records for engine snapshot, component snapshot, delivery trace,
  prototype witness, **component observations** (record-carrying batches
  wrapping component-owned typed observations), and **list record
  kinds** (catalog-style schema introspection — capability flags +
  record-kind names + contract-crate identifiers — not field-level
  schemas).
- Reply records that wrap or summarize observations for projection.
- `ComponentObservationResult` closed enum that wraps the *owning
  component's* batch types (`signal_persona_terminal::TerminalObservationBatch`,
  `signal_persona_router::RouterObservationBatch`, etc.) — wrapping,
  not redefining.
- Signal root-verb mapping for every request variant. All current variants
  map to `Match`. Future `SubscribeComponent` (Slice 3) maps to
  `Subscribe`.

## 2. Non-ownership

This crate does not own:

- Router tables or route-decision records.
- Terminal session or transcript records.
- Manager event-log rows.
- Message ingress ledgers.
- Harness lifecycle records.
- Component databases, actors, sockets, reducers, or redaction policy.

Component-specific observation records start in the component contract
that owns the state (`signal-persona-terminal::TerminalObservation`,
etc.). This crate **wraps**; it does not define. Split to a sibling
introspection contract (`signal-persona-<X>-introspect`) only when the
observation vocabulary becomes heavy or high-churn — per
`~/primary/skills/contract-repo.md` §"Contracts name a component's wire
surface".

## 3. Constraints

| Constraint | Witness |
|---|---|
| The central contract asks and wraps; it does not define component rows. | Review of public types and tests: no router/terminal/manager row vocabulary defined here. Source-scan witness: `central_contract_does_not_define_terminal_rows`. |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests. |
| **Every `IntrospectionRequest` variant declares a Signal root verb.** | The `signal_channel!` declaration names each root verb; `signal-core` generates `IntrospectionRequest::signal_verb()` and `IntrospectionRequest::into_signal_request()`. Round-trip tests assert verb+payload alignment. |
| Read-shaped payloads use `Match` or `Subscribe`; write-shaped payloads use `Assert`/`Mutate`/`Retract`/`Atomic`. Read-algebra (`Project`/`Aggregate`/`Constrain`/`Infer`/`Recurse`) appears inside `Match`/`Subscribe`/`Validate` payloads via `sema-engine`'s `ReadPlan`, never as a root verb. | The `signal_channel!` root declarations enforce this; introspect is read-only inspection plane so all current variants are `Match`. |
| NOTA derives live on the same typed records. | Cargo tests compile `NotaRecord`, `NotaEnum`, and `NotaTransparent` derives. |
| The contract contains no daemon code. | Source scan: no Kameo, no Tokio, no socket code. |

## 4. Status

Operator-assistant is implementing Slice 1: verb-mapping witness +
envelope extension (`ComponentObservations`, `ListRecordKinds`,
`AwaitingCorrelationCache`). The `ComponentObservationResult`
wrapping types compile against component-owned terminal + router
observation types (added in `signal-persona-terminal` and
`signal-persona-router` as part of the same slice).

`SubscribeComponent` lands in Slice 3 (gated on `sema-engine`
Package 4 + per-peer commit-then-emit). Its envelope variant is
out of scope until then; adding an `Unimplemented`-stub variant
would give every consumer contract debt with no working feature.
