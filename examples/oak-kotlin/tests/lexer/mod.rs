use oak_core::Lexer;
use oak_kotlin::KotlinLexer;

#[test]
fn test_kotlin_lexer() {
    // Simple test to ensure lexer compiles and runs
    let source = "data class Person(val name: String, val age: Int)";
    let lexer = KotlinLexer;
    let source_text = oak_core::source::SourceText::new(source);
    let result = lexer.lex(&source_text, &[], &mut oak_core::lexer::NoLexerCache);
    assert!(result.result.is_ok());
}
