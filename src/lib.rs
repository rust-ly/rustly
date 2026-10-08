//! The logic behind the functions in `api/`. Each of those is a thin wrapper:
//! read the request, call into this crate, write the response. Keeping the
//! logic here means it can be tested without Vercel or a network.

mod endpoints;
mod http;
mod redact;

pub use endpoints::Api;
pub use http::{read_json, respond};

/// Largest `code` the API accepts, in bytes.
pub const MAX_CODE_BYTES: usize = 20 * 1024;

/// A request the API turns down, sent back as an [`shared::ApiError`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub status: u16,
    pub message: String,
}

impl Failure {
    pub fn new(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }

    pub fn too_large() -> Self {
        Self::new(
            413,
            format!(
                "Your code is over {} KB. Please make it shorter.",
                MAX_CODE_BYTES / 1024
            ),
        )
    }
}

impl From<runner::playground::Error> for Failure {
    fn from(e: runner::playground::Error) -> Self {
        use runner::playground::Error;
        let message = match e {
            Error::Timeout => "The compile service took too long to answer. Please try again.",
            Error::Status(_) | Error::Request(_) => {
                "The compile service isn't answering right now. Please try again in a minute."
            }
        };
        Self::new(502, message)
    }
}

/// Turns down `code` over [`MAX_CODE_BYTES`].
pub fn check_size(code: &str) -> Result<(), Failure> {
    if code.len() > MAX_CODE_BYTES {
        Err(Failure::too_large())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_code_up_to_the_limit() {
        assert_eq!(check_size(&"a".repeat(MAX_CODE_BYTES)), Ok(()));
    }

    #[test]
    fn rejects_code_over_the_limit_with_413() {
        let err = check_size(&"a".repeat(MAX_CODE_BYTES + 1)).unwrap_err();
        assert_eq!(err.status, 413);
    }
}
