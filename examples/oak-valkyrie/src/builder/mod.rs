mod build_class;
mod build_expr;
mod build_micro;
mod build_namespace;
mod build_root;
mod build_stmt;

use crate::{ValkyrieLanguage, ValkyrieParser};
use crate::ast::Span;
use oak_core::{Builder, BuilderCache, OakDiagnostics, Parser, SourceText, TextEdit, builder::BuildOutput, source::Source};

/// Extracts text from source using a span.
pub fn text(source: &SourceText, span: Span) -> String {
    source.slice(span).to_string()
}

/// Valkyrie builder.
pub struct ValkyrieBuilder<'config> {
    config: &'config ValkyrieLanguage,
}

impl<'config> ValkyrieBuilder<'config> {
    /// Create a new Valkyrie builder.
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
        Self { config }
    }
}

impl<'config> Builder<ValkyrieLanguage> for ValkyrieBuilder<'config> {
    fn build<'a, S: Source + ?Sized>(&self, source: &S, edits: &[TextEdit], _cache: &'a mut impl BuilderCache<ValkyrieLanguage>) -> BuildOutput<ValkyrieLanguage> {
        let parser = ValkyrieParser::new(self.config);

        let mut parse_cache = oak_core::parser::session::ParseSession::<ValkyrieLanguage>::default();
        let parse_result = parser.parse(source, edits, &mut parse_cache);

        match parse_result.result {
            Ok(green_tree) => {
                let source_text = SourceText::new(source.get_text_in((0..source.length()).into()).into_owned());
                match parser.build_root(green_tree, &source_text) {
                    Ok(ast_root) => OakDiagnostics { result: Ok(ast_root), diagnostics: parse_result.diagnostics },
                    Err(build_error) => {
                        let mut diagnostics = parse_result.diagnostics;
                        diagnostics.push(build_error.clone());
                        OakDiagnostics { result: Err(build_error), diagnostics }
                    }
                }
            }
            Err(e) => OakDiagnostics { result: Err(e), diagnostics: parse_result.diagnostics },
        }
    }
}
