# Non-ideal agent guidance — signal-introspect

This file names required temporary debt. Treat each item as a future fix target,
not as a pattern to copy.

- The current bootstrap contract is authored directly in `src/lib.rs` and
  `src/system_event.rs`, with channel behavior emitted by `signal_channel!`.
  Until Logos can generate the behavior, these Rust sources are the single
  contract surface: do not add a shadow schema, generated facade, or parallel
  wire records in consumers. The eventual source schema is Ethos, not the
  discarded historical schema format.
