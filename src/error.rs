use std::error;

#[derive(Debug)]
pub struct TemplateError {
	kind:  TemplateErrorKind,
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
