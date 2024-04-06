//! Module for the Error of this Crate

use std::error;

/// The Error for this Crate
#[derive(Debug)]
pub struct TemplateError {
    kind:  TemplateErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

/// The Error kind for [`TemplateError`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateErrorKind {
    MissingData,
}

impl TemplateError {
    /// Create a new instance of the Error instance for this library
    pub fn new<E>(kind: TemplateErrorKind, error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        return Self {
            kind,
            error: error.into(),
        };
    }

    /// Get the [`TemplateErrorKind`] that his Error instance is
    #[must_use]
    pub const fn kind(&self) -> TemplateErrorKind {
        return self.kind;
    }
}

impl std::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "{}",
            match &self.kind {
                TemplateErrorKind::MissingData => format!("MissingData: {}", self.error),
            }
        );
    }
}

impl error::Error for TemplateError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_display() {
        let missing_data_error = TemplateError::new(TemplateErrorKind::MissingData, "SomeText");

        assert_eq!("MissingData: SomeText", format!("{}", missing_data_error));
    }

    #[test]
    fn test_get_kind() {
        let missing_data_error = TemplateError::new(TemplateErrorKind::MissingData, "SomeText");

        assert_eq!(TemplateErrorKind::MissingData, missing_data_error.kind());
    }
}
