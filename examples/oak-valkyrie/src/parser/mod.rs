pub use element_type::ValkyrieElementType;

use crate::{ValkyrieLanguage, ValkyrieLexer, kind::ValkyrieSyntaxKind};
use oak_core::{Parser, Source, TextEdit, parser::ParseCache};

pub(crate) type State<'a, S> = oak_core::parser::ParserState<'a, ValkyrieLanguage, S>;

/// Valkyrie parser.
pub struct ValkyrieParser<'config> {
    config: &'config ValkyrieLanguage,
}

impl<'config> Parser<ValkyrieLanguage> for ValkyrieParser<'config> {
    fn parse<'a, S: Source + ?Sized>(&self, text: &'a S, edits: &[TextEdit], cache: &'a mut impl ParseCache<ValkyrieLanguage>) -> oak_core::parser::ParseOutput<'a, ValkyrieLanguage> {
        oak_core::parser::parse_with_lexer(&ValkyrieLexer::new(self.config), text, edits, cache, |state| {
            let checkpoint = state.sink.checkpoint();
            while state.not_at_end() {
                state.advance();
            }
            let root = state.sink.finish_node(checkpoint, ValkyrieElementType::Root);
            Ok(root)
        })
    }
}

impl<'config> ValkyrieParser<'config> {
    /// Create a new Valkyrie parser.
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
        Self { config }
    }

    pub(crate) fn skip_trivia<'a, S: oak_core::Source + ?Sized>(&self, state: &mut oak_core::parser::ParserState<'a, ValkyrieLanguage, S>) {
        state.skip_trivia();
    }
}

/// Element type definitions for the Valkyrie parser.
pub mod element_type;
