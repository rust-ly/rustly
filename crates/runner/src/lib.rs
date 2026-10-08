//! Talks to the compile service and turns its output into the shared API types.

pub mod diagnostics;
pub mod playground;

pub use diagnostics::parse_diagnostics;
pub use playground::PlaygroundClient;
