//! Talks to the compile service and turns its output into the shared API types.

pub mod diagnostics;
pub mod playground;
pub mod tests_output;

pub use diagnostics::parse_diagnostics;
pub use playground::PlaygroundClient;
pub use tests_output::parse_tests;
