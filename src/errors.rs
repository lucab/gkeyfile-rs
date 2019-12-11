//! Error handling.

use snafu::Snafu;

/// Result with library-specific error types.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Library errors.
#[derive(Debug, Snafu)]
pub struct Error(IntError);

/// Internal error variants.
#[allow(missing_docs)]
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub(crate) enum IntError {
    /// Failed to read content.
    #[snafu(display("failed to read content at line {}: {}", line, source))]
    FailedRead { line: usize, source: std::io::Error },

    /// Failed to write content.
    #[snafu(display("failed to write content: {}", source))]
    FailedWrite { source: std::io::Error },

    /// Content is not in a valid format.
    #[snafu(display("failed to parse content at line {}: {}", line, reason))]
    Malformed { line: usize, reason: String },

    /// Invalid value.
    #[snafu(display("invalid value: {}", reason))]
    InvalidValue { reason: String },
}
