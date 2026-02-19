use crate::{language::JinjaLanguage, lexer::token_type::JinjaTokenType};
/// Highlighter module for Jinja
///
/// This module provides syntax highlighting support for Jinja templates.
use oak_core::{Language, TokenType};
use oak_highlight::{HighlightTheme, Highlighter};

/// Highlighter for Jinja templates
#[derive(Debug, Clone)]
pub struct JinjaHighlighter {
    theme: HighlightTheme,
}

impl JinjaHighlighter {
    /// Creates a new Jinja highlighter with the given theme
    pub fn new(theme: HighlightTheme) -> Self {
        Self { theme }
    }
}

impl Highlighter for JinjaHighlighter {
    type Lang = JinjaLanguage;

    fn highlight_token(&self, token: &oak_core::Token<Self::Lang>) -> oak_highlight::HighlightResult {
        use JinjaTokenType::*;

        let token_type = token.kind;
        let highlight = match token_type {
            DoubleLeftBrace | DoubleRightBrace | LeftBracePercent | PercentRightBrace => oak_highlight::HighlightGroup::Punctuation,
            Identifier => oak_highlight::HighlightGroup::Identifier,
            String => oak_highlight::HighlightGroup::String,
            Number => oak_highlight::HighlightGroup::Number,
            Boolean => oak_highlight::HighlightGroup::Keyword,
            Comment => oak_highlight::HighlightGroup::Comment,
            _ => oak_highlight::HighlightGroup::Text,
        };

        Ok((highlight, None))
    }

    fn theme(&self) -> &HighlightTheme {
        &self.theme
    }
}
