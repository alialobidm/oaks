use crate::{language::CobolLanguage, lexer::CobolLexer};
use oak_core::{
    parser::{ParseCache, ParseOutput, Parser, parse_with_lexer},
    source::{Source, TextEdit},
};

pub mod element_type;

pub use element_type::CobolElementType;

/// COBOL parser.
pub struct CobolParser;

impl CobolParser {
    /// Create a new COBOL parser.
    pub fn new() -> Self {
        Self
    }
}

impl Parser<CobolLanguage> for CobolParser {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<CobolLanguage>) -> ParseOutput<'a, CobolLanguage> {
        let lexer = CobolLexer::new();
        parse_with_lexer(&lexer, text, edits, cache, |state| {
            let cp = state.checkpoint();
            while state.not_at_end() {
                state.bump();
            }
            Ok(state.finish_at(cp, CobolElementType::Root))
        })
    }
}
