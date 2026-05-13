# signal-persona-introspect - architecture

*Central Signal envelope contract for Persona introspection.*

## 0. Intent

`signal-persona-introspect` is the contract a client uses to ask
`persona-introspect` for an engine observation. It is a wrapper and selector
contract, not a home for component-owned rows.

The component shape comes from
`~/primary/reports/operator/114-persona-introspect-prototype-impact-survey.md`:
`persona-introspect` is prototype-included as the inspection-plane witness.
The operational delivery path stays outside this crate.

## 1. Owned surface

- `IntrospectionRequest`
- `IntrospectionReply`
- `IntrospectionTarget`
- `IntrospectionScope`
- Query records for engine snapshot, component snapshot, delivery trace, and
  prototype witness.
- Reply records that wrap or summarize observations for projection.

## 2. Non-ownership

This crate does not own:

- Router tables or route-decision records.
- Terminal session or transcript records.
- Manager event-log rows.
- Message ingress ledgers.
- Harness lifecycle records.
- Component databases, actors, sockets, reducers, or redaction policy.

Component-specific observation records start in the component contract that
owns the state. Split to a sibling introspection contract only when the
observation vocabulary becomes heavy or high-churn.

## 3. Constraints

| Constraint | Witness |
|---|---|
| The central contract asks and wraps; it does not define component rows. | Review of public types and tests: no router/terminal/manager row vocabulary here. |
| Every request/reply travels as a Signal frame. | `tests/round_trip.rs` length-prefixed frame tests. |
| NOTA derives live on the same typed records. | Cargo tests compile `NotaRecord`, `NotaEnum`, and `NotaTransparent` derives. |
| The contract contains no daemon code. | Source scan: no Kameo, no Tokio, no socket code. |

## 4. Prototype status

Scaffold exists. The next implementation step is wiring
`persona-introspect` to receive these frames and fan out to manager, router,
and terminal observations.
