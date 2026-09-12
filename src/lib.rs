pub mod generated;
pub use generated::signal::*;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

/// The portable rkyv Signal frame is one shared type across the estate.
/// A second copy here would be a different Rust type, forking the wire.
pub use signal::{ByteViewable, Restorable, Signal, Signalizable};
