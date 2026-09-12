# skills — signal-introspect

*Per-repo agent guide for the central introspection envelope contract.*

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/subscription-lifecycle.md` (when adding any
  subscription / event / retract variant)
- `~/primary/skills/nix-discipline.md`
- `ethos` and `datom` skills (this crate's wire contract is authored
  in `ethos/signal.ethos` and carries optional `datom` derives)
- this repo's `ARCHITECTURE.md`
- the consumers' `ARCHITECTURE.md` files
  (`introspect/` and whichever component contract is being
  wrapped).

## What this repo is for

`signal-introspect` is the wrapper-and-selector contract a
client uses to ask `introspect` for an engine observation.
It carries:

- the request/reply envelope (`Query` / `Response`);
- the targets and scopes a query may name;
- typed roll-up records that project peer-component observations
  to a human-facing surface (`PrototypeWitnessObservation`,
  `ComponentSnapshotObservation`, `DeliveryTraceObservation`);
- the system-event recording, coalescing, and flush surface
  (`RecordSystemEvent`, `SystemEvents`, `FlushSystemEvents`).

It is **not** a shared row bucket. Component-specific observation
records start in the component contract that owns the state
(`signal-persona`, `signal-message`, etc.). This
crate wraps; it does not redefine.

## What this repo owns

- The central introspection request/reply envelope (`Query` /
  `Response`).
- Query selectors, projection wrappers, and future subscription
  handles.
- `Signal<T>` — a portable rkyv frame type, plus the `Signalizable`,
  `Restorable`, and `ByteViewable` traits that move a typed value
  into and out of peer-wire bytes.

## What this repo does not own

- Router, terminal, manager, harness, message, system, or mind row
  types.
- Daemon code, Kameo actors, sockets, storage access, or projection
  policy.
- A shared schema bucket for every component.

Component observation records live in the component contract that
owns the observed state. This crate asks and wraps.

## Load-bearing invariants

- **The wire contract is authored once, in Ethos.** Every type,
  variant, and field lives in `ethos/signal.ethos`. `build.rs`
  regenerates `src/generated/signal.rs` from that source on every
  build with the `ethos-zero` crate and asserts the regenerated text
  is byte-identical to what is checked in — the generated file can
  never silently drift from its source.
- **Wire enums are closed.** `ComponentReadiness` and
  `DeliveryTraceObservationStatus` carry no `Unknown` variant. The
  "not-yet-observed" axis lives on `Option<>` wrappers on the carrier
  records (`ComponentSnapshotObservation`,
  `PrototypeWitnessObservation`'s readiness and delivery-status
  fields) or on an empty vector for
  `DeliveryTraceObservation`'s events. `None` or `[]` means "no
  observation yet"; `Some(state)` or a present event carries the
  closed observation.
- **Delivery traces use a four-field key.** `DeliveryTraceObservationKey`
  is `(engine, message slot, component, hop index)`. The first
  three fields join one delivery chain; hop index orders events
  without clock sorting.
- **The contract asks and wraps; it does not define component rows.**
  When a peer-component observation lands, the row vocabulary lives
  in the component's own contract crate; this crate wraps via
  `ComponentSnapshotObservation` and similar wrapper types.
- **No runtime code.** No Kameo, Tokio, socket, storage, or daemon glue
  in this crate.
- **Round trips cover every variant carried through rkyv.**
  `tests/generated_contract.rs` exercises `Query`, `Response`, and
  `ComponentTraceEvent` through `Signalizable` / `Restorable`,
  including a malformed-bytes rejection case.
- **Datom derives are feature-gated, not a parallel decode path.**
  Under the `datom` feature, the same generated records derive
  `datom_codec::Datomizable` / `Compositional`; there is no separate
  legacy text codec in this crate.
- **Pin every Git producer exactly.** Cargo dependencies use immutable
  `rev = "..."` identities. Moving branches are not build inputs.

## Editing patterns

### Adding a new query

1. Decide whether the answer lives in this crate (a roll-up across
   peers) or in the owning component's contract crate (a peer-owned
   observation). The default is "owning component"; this crate wraps.
2. If it lives here: declare the new type(s) and the `Query` /
   `Response` variant head in `ethos/signal.ethos`.
3. Run the ethos-zero generator against `ethos/signal.ethos` to
   regenerate `src/generated/signal.rs` (`build.rs` will otherwise
   fail the freshness assertion).
4. Add rkyv round-trip witnesses (and, if the type should be
   `datom`-encodable, exercise the feature-gated datom round trip) in
   `tests/generated_contract.rs`.
5. Update `ARCHITECTURE.md`.

### Modeling "not yet observed"

The "not observed" axis is the `Option` wrapper on the carrier
record, never an `Unknown` variant inside the inner enum. Inner
enums stay closed; the carrier names the absence via `None`.

```text
Wrong:                                       Right:
  status: DeliveryTraceObservationStatus       status: Option<DeliveryTraceObservationStatus>
  | ...
  | Unknown                                    pub enum DeliveryTraceObservationStatus {
                                                    Accepted, Routed, Delivered, Deferred, Failed,
                                                }
```

For delivery traces, the carrier is
`DeliveryTraceObservation`'s event vector rather than
`Option<DeliveryTraceObservationStatus>`. An empty vector means no
correlated events have arrived for the join key; a populated vector
is ordered by `DeliveryTraceObservationKey`'s hop index.

### Adding a SubscribeComponent variant

Wait until `sema-engine` per-peer commit-then-emit gates land
(Slice 3 territory). When it does:

1. Read `~/primary/skills/subscription-lifecycle.md` end-to-end.
2. Declare the subscription request/reply shape in
   `ethos/signal.ethos` with both a request-side retraction variant
   and a reply-side `SubscriptionRetracted` variant.
3. Witness the full subscribe → event → retract → ack → end
   lifecycle.

Do not add an `Unimplemented`-stub `SubscribeComponent` variant in
the meantime — consumers would write shadow code against a
non-functional feature.

## See also

- this workspace's `skills/contract-repo.md`.
- this workspace's `skills/subscription-lifecycle.md`.
- this workspace's `skills/architectural-truth-tests.md`.
- this workspace's `ESSENCE.md` §"Perfect specificity at
  boundaries" — the rule the closed-enum discipline implements.
- `signal-persona`'s `skills.md` and `signal-message`'s `skills.md`
  — sibling contract conventions.
