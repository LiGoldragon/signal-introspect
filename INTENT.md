# INTENT — signal-introspect

*The wire vocabulary contract for Persona introspection. Defines the typed
request/reply channel a client uses to ask the `introspect` daemon for engine
and component observations and to receive projected roll-up records.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-introspect` contract.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `introspect/INTENT.md`.

## Why this repo exists

`signal-introspect` is the **ordinary peer-callable wire contract** for the
`introspect` daemon. It is the central envelope-and-selector vocabulary a client
uses to ask `introspect` for an engine observation: it defines the targets and
scopes a query may name, and the typed roll-up records that project peer-component
observations to a human-facing surface. It asks and wraps; the component-specific
observation row types stay in their own owning component contracts (e.g.
`signal-router`), so this crate never becomes a shared schema bucket. Runtime
actors, the sema-engine store, peer-subscription fan-out, and projection logic
live in `introspect`.

## The channel shape

The Introspection channel carries:

- **Requests:** `EngineSnapshot`, `ComponentSnapshot`, `DeliveryTrace`,
  `PrototypeWitness` — each a read-shaped observation query carrying a typed
  `*Query` payload.
- **Replies:** `EngineSnapshot`, `ComponentSnapshot`, `DeliveryTrace`,
  `PrototypeWitness` (the projected roll-up records), plus `Unimplemented` and
  `Denied` carrying typed reason enums.

The wire vocabulary is contract-local: the daemon lowers these public operations
into component-local commands and aggregates observations from peer daemons; Sema
classification happens at observation time, not on the wire.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch; unimplemented paths reply
  `Unimplemented` with a typed reason, denied paths reply `Denied`.
- Request payloads do not mint sequence numbers, snapshot timestamps, or
  correlation identity that belongs to the daemon.
- `introspect` mints those values at the daemon; request records carry the
  query target and scope only.
- No stringly-typed dispatch. Targets, scopes, and reason fields are typed
  closed enums.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use contract-local
operation verbs":

- Operation roots are domain verbs in verb form (the read action this crate
  offers reads as `Observe`/`Query`), not Sema class words.
- Reply success variants name the observation kind returned; rejections are
  typed (`Unimplemented`, `Denied`) carrying closed-enum reasons.
- Payload record names are the domain nouns the operation carries
  (`EngineSnapshot`, `ComponentSnapshot`, `DeliveryTrace`, `PrototypeWitness`),
  not `Request`, `Data`, or generic containers.
- The public request roots are the contract-local operation heads declared in
  `src/lib.rs`; Sema class words are forbidden on the public wire.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses.
- No runtime code: no actors, no tokio, no socket binding, no storage, no
  aggregation logic.
- Contract types derive NOTA in this crate. Consumers do not carry shadow types
  that re-derive the text surface.
- Every operation and reply variant round-trips through both rkyv frames and
  NOTA text.
- This contract asks and wraps. Component-specific observation row types stay in
  their owning component contracts; `signal-introspect` must not absorb them.

## Three-layer model

Layer 1 (this crate): contract operations on the wire (the observation/query
verbs and their selectors).
Layer 2 (daemon): component-local `IntrospectCommand` enum (e.g.
`CollectEngineSnapshot`, `CollectComponentSnapshot`) plus a command executor
that aggregates observations from peer daemons.
Layer 3 (observation): payloadless Sema class labels for cross-component
introspection.

The contract names the public action at the boundary; the daemon decides what
internal work and Sema class label each action maps to. Sema classification
never appears on the wire.

## Code map

```text
src/lib.rs                          — query/reply records, NOTA codecs, signal_channel! invocation
schema/signal-introspect.concept.schema — concept-schema source for the contract
examples/                            — canonical NOTA examples per operation/reply variant
tests/round_trip.rs                  — rkyv frame and NOTA round-trip witnesses per variant
```

## Non-ownership

This crate does not own:

- `introspect` daemon runtime, actors, or component lifecycle;
- the introspect sema-engine store or any storage tables;
- socket binding, transport, peer-subscription fan-out, or version handshake;
- aggregation, projection, or observation-collection logic;
- the component-specific observation row types that each peer contract owns;
- NOTA projection policy or surface (CLI formatting, audit wrapping).

## See also

- `ARCHITECTURE.md` — detailed channel shape, per-operation vocabulary, the
  three-layer migration, and closed-enum discipline.
- `../introspect/INTENT.md` — daemon-side intent (schema-driven planes, actors, state).
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire layers.
