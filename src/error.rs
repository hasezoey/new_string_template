use std::error;

#[derive(Debug)]
pub struct TemplateError {
    kind: TemplateErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemplateErrorKind {
    MissingData,
}

impl TemplateError {
    pub fn new<E>(kind: TemplateErrorKind, error: E) -> TemplateError
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        return TemplateError {
            kind,
            error: error.into(),
        };
    }

    pub fn kind(&self) -> TemplateErrorKind {
        return self.kind;
    }
}
