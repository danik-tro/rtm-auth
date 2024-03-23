use derive_more::{AsRef, Deref, From, FromStr, Into};

use regex::Regex;
use std::sync::OnceLock;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

/// Represents an email address and provides validation through a regular expression.
#[derive(Debug, AsRef, Into, FromStr, Deref, From)]
pub struct Email(String);

impl Email {
    /// # Panics
    ///
    /// Panics if the regex pattern for validating email addresses is invalid.
    pub fn is_email_valid(email: &str) -> bool {
        // unwrap is safe here
        let reg = EMAIL_REGEX.get_or_init(|| {
            Regex::new(r"^([a-zA-Z0-9._-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6})$").unwrap()
        });

        reg.is_match(email)
    }
}

impl<'a> TryFrom<&'a str> for Email {
    type Error = ParseEmailError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if !Self::is_email_valid(value) {
            return Err(ParseEmailError::InvalidEmail(value.into()));
        }

        Ok(Self(value.into()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseEmailError {
    #[error("Failed to parse the email: {0}.")]
    InvalidEmail(String),
}
