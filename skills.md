# skills — signal-persona-introspect

*Per-repo agent guide for the central introspection envelope contract.*

---

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/subscription-lifecycle.md` (when adding any
  subscription / event / retract variant)
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- the consumers' `ARCHITECTURE.md` files
  (`persona-introspect/` and whichever component contract is being
  wrapped).

---

## What this repo is for

`signal-persona-introspect` is the wrapper-and-selector contract a
client uses to ask `persona-introspect` for an engine observation.
It carries:

- the request/reply envelope (`IntrospectionRequest` /
  `IntrospectionReply`);
- the targets and scopes a query may name;
- typed roll-up records that project peer-component observations
  to a human-facing surface (the `PrototypeWitness`, the
  `ComponentSnapshot`, the `DeliveryTrace`).

It is **not** a shared row bucket. Component-specific observation
records start in the component contract that owns the state
(`signal-persona-terminal`, `signal-persona-router`, etc.). This
crate wraps; it does not redefine.

---

## What this repo owns

- The central introspection request/reply envelope.
- Query selectors, projection wrappers, and future subscription
  handles.
- Frame aliases over `signal_core::Frame<IntrospectionRequest,
  IntrospectionReply>`.

## What this repo does not own

- Router, terminal, manager, harness, message, system, or mind row
  types.
- Daemon code, Kameo actors, sockets, redb access, or projection
  policy.
- A shared schema bucket for every component.

Component observation records live in the component contract that
owns the observed state. This crate asks and wraps.

---

## Load-bearing invariants

- **Wire enums are closed.** `ComponentReadiness` and
  `DeliveryTraceStatus` carry no `Unknown` variant. The
  "not-yet-observed" axis lives on `Option<>` wrappers on the carrier
  records (`ComponentSnapshot.readiness`, `DeliveryTrace.status`,
  `PrototypeWitness.{manager,router,terminal}_seen`,
  `PrototypeWitness.delivery_status`). `None` means "no observation
  yet"; `Some(state)` carries the closed observation.
- **Every request variant declares a Signal root verb.** The
  `signal_channel!` declaration is the source of truth; the macro
  generates `IntrospectionRequest::signal_verb()` and round-trip
  tests assert every variant. Today every variant is `Match`.
- **The contract asks and wraps; it does not define component rows.**
  When a peer-component observation lands, the row vocabulary lives
  in the component's own contract crate; this crate wraps via
  `ComponentObservationResult` and similar wrapper enums.
- **No runtime code.** No Kameo, Tokio, socket, redb, or daemon glue
  in this crate.
- **Round trips cover every variant.** rkyv length-prefixed frame
  round trips in `tests/round_trip.rs`; canonical NOTA examples in
  `examples/canonical.nota` with a parser test.
- **Pin upstream contracts via a named API reference.** Cargo deps
  declare `git = "..."` with a named branch/bookmark, never raw
  `rev = "..."`.

---

## Editing patterns

### Adding a new query

1. Decide whether the answer lives in this crate (a roll-up across
   peers) or in the owning component's contract crate (a peer-owned
   observation). The default is "owning component"; this crate wraps.
2. If it lives here: write the canonical NOTA example for the
   request and the expected reply in `examples/canonical.nota`.
3. Declare the payload and reply variant in `src/lib.rs`.
4. Add the variant to the `signal_channel!` declaration with its
   root verb (`Match` today).
5. Add the rkyv and NOTA round-trip witnesses.
6. Update `ARCHITECTURE.md`.

### Modeling "not yet observed"

The "not observed" axis is the `Option` wrapper on the carrier
record, never an `Unknown` variant inside the inner enum. Inner
enums stay closed; the carrier names the absence via `None`.

```text
Wrong:                              Right:
  status: DeliveryTraceStatus         status: Option<DeliveryTraceStatus>
  | ...
  | Unknown                           pub enum DeliveryTraceStatus {
                                          Accepted, Routed, Delivered, Deferred, Failed,
                                      }
```

### Adding a SubscribeComponent variant

Wait until `sema-engine` per-peer commit-then-emit gates land
(Slice 3 territory). When it does:

1. Read `~/primary/skills/subscription-lifecycle.md` end-to-end.
2. Declare the `stream` block in `signal_channel!` with both a
   request-side `Retract <Name>Retraction(<Token>)` variant and a
   reply-side `SubscriptionRetracted` variant.
3. Witness the full subscribe → event → retract → ack → end
   lifecycle.

Do not add an `Unimplemented`-stub `SubscribeComponent` variant in
the meantime — consumers would write shadow code against a
non-functional feature.

---

## NOTA codec quirk

The `signal_channel!` macro emits a request variant's NOTA head as
the **payload's record head**, not the Rust variant name. For
example, `IntrospectionRequest::PrototypeWitness(PrototypeWitnessQuery { .. })`
encodes as `(PrototypeWitnessQuery (...))`, not
`(PrototypeWitness (...))`. Canonical examples and round-trip tests
use the payload heads.

---

## See also

- this workspace's `skills/contract-repo.md`.
- this workspace's `skills/subscription-lifecycle.md`.
- this workspace's `skills/architectural-truth-tests.md`.
- this workspace's `ESSENCE.md` §"Perfect specificity at
  boundaries" — the rule the closed-enum discipline implements.
- `signal-persona-router`'s `skills.md` and
  `signal-persona-terminal`'s `skills.md` — sibling contract
  conventions.
