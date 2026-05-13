# skills - signal-persona-introspect

*Per-repo agent guide.*

## Checkpoint - read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`
- the consumers' `ARCHITECTURE.md` files
  (`persona-introspect/` and whichever component contract
  is being wrapped)

## What this repo owns

- The central introspection request/reply envelope.
- Query selectors, projection wrappers, and future subscription handles.
- Frame aliases over `signal_core::Frame<IntrospectionRequest,
  IntrospectionReply>`.

## What this repo does not own

- Router, terminal, manager, harness, message, system, or mind row types.
- Daemon code, Kameo actors, sockets, redb access, or projection policy.
- A shared schema bucket for every component.

Component observation records live in the component contract that owns the
observed state. This crate asks and wraps.
