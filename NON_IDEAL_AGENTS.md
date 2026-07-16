# Non-ideal agent guidance — signal-introspect

This file names required temporary debt. Treat each item as a future fix target, not as a pattern to copy.

- The worktree contains parked, incomplete TrueSchema consumer migration work from an over-scope rollout attempt. Do not continue or consume this WIP until the schema daemon and renamed schema library dependency shape has stabilized. Remove this item when the parked WIP is completed, discarded, or replaced.
- The current `main` contract is authored in `src/lib.rs` plus concern modules and emitted by `signal_channel!`; it has no schema input or build-time generator. A previously parked TrueSchema migration is not on `main`, while downstream `introspect` consumes the current hand-written public names. New contract nouns must therefore be added to this one canonical contract surface rather than mirrored in consumers. The proper fix is a coordinated whole-contract TrueSchema migration after the generator dependency set is stable, preserving the compatibility fixtures and public constructors; do not introduce parallel wire structs in the daemon meanwhile.
