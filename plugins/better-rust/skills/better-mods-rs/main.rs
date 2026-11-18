// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

/// 
/// This module contains a top-level struct `Cli` which uses `clap_derive` to implement the
/// Command-Line structure. A `Commands` enum contains the set of commands and their specific
/// argument structs.
///
/// All types implement `OnceCommand` to allow the simple cascade of execution.
/// 
pub(crate) mod cli;
use cli::Cli;

/// 
/// Actual implementation types, if necessary, are here. These are typically structs with their
/// context set by the cli (using explicit `new` constructors) and then their implementation of
/// `OnceCommand::execute` is called.
/// 
pub(crate) mod command;

/// 
/// Implements `Error` using `thiserror`.
/// 
pub(crate) mod error;
use error::Error;

/// 
/// Initializes any/all of tracing, logging, and metrics. A top-level `init()` function performs
/// all necessary initialization.
/// 
mod telemetry;
use telemetry::Tracing;

use std::process::ExitCode;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait OnceCommand {
    /// The type returned on successful execution.
    type Output;
    /// The error type returned on failure.
    type Error: std::error::Error;

    /// Executes the command, consuming self.
    fn execute(self) -> Result<Self::Output, Self::Error>;
}

// ------------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------------

/// CLI functions that propogate errors _should_ use `ExitCode` to denote success/failure
/// even if no explicit errors occurred.
fn main() -> ExitCode {
    Tracing::default().execute()?;
    Cli::parse().execute()?;
    ExitCode::SUCCESS
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------
