use crate::language::JinjaLanguage;
/// Formatter module for Jinja
///
/// This module provides code formatting support for Jinja templates.
use oak_core::{Language, Source};
use oak_pretty_print::{Formatter, FormatterConfig};

/// Formatter for Jinja templates
#[derive(Debug, Clone)]
pub struct JinjaFormatter {
    config: FormatterConfig,
}

impl JinjaFormatter {
    /// Creates a new Jinja formatter with the given config
    pub fn new(config: FormatterConfig) -> Self {
        Self { config }
    }
}

impl Formatter for JinjaFormatter {
    type Lang = JinjaLanguage;

    fn format<S: Source + ?Sized>(&self, source: &S) -> oak_pretty_print::FormatResult {
        // TODO: Implement actual formatting logic
        Ok(source.to_string())
    }

    fn config(&self) -> &FormatterConfig {
        &self.config
    }
}
