//! Predefined names.

include!(concat!(env!("OUT_DIR"), "/consts_generated.rs"));

/// Unregistered subtypes starting with `x-`.
pub mod x_ {
    include!(concat!(env!("OUT_DIR"), "/consts_generated.x_.rs"));
}
