use oak_core::{Language, LexerCache, SourceText, Token, parser::ParseSession};
use oak_kotlin::{Kotlin, KotlinLexer, KotlinParser, NoLexerCache};

#[test]
fn test_data_class_parsing() {
    let source = SourceText::new("data class Person(val name: String, val age: Int) {}");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_sealed_class_parsing() {
    let source = SourceText::new("sealed class Result<T> {}");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_extension_function_parsing() {
    let source = SourceText::new("fun String.reverse(): String { return this.reversed() }");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_suspend_function_parsing() {
    let source = SourceText::new("suspend fun fetchData(): String { return \"data\" }");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_inline_function_parsing() {
    let source = SourceText::new("inline fun <T> withLock(lock: Lock, action: () -> T): T { lock.lock(); try { return action() } finally { lock.unlock() } }");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_reified_function_parsing() {
    let source = SourceText::new("inline fun <reified T> T.printType() { println(T::class.simpleName) }");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_when_expression_parsing() {
    let source = SourceText::new("fun evaluate(x: Int): String = when (x) { 1 -> \"one\"; 2 -> \"two\"; else -> \"other\" }");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_if_expression_parsing() {
    let source = SourceText::new("fun max(a: Int, b: Int): Int = if (a > b) a else b");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_try_expression_parsing() {
    let source = SourceText::new("fun divide(a: Int, b: Int): Int? = try { a / b } catch (e: ArithmeticException) { null } finally { println(\"Done\") }");
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);

    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);

    assert!(parse_output.result.is_ok());
}

#[test]
fn test_regular_class_parsing() {
    let source = SourceText::new(
        "class MyClass<T>(val value: T) : MyInterface<T> {
    fun doSomething() {}
}",
    );
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_interface_parsing() {
    let source = SourceText::new(
        "interface MyInterface<T> {
    fun process(value: T): T
}",
    );
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_object_parsing() {
    let source = SourceText::new(
        "object Singleton {
    val instance = Singleton
}",
    );
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_companion_object_parsing() {
    let source = SourceText::new(
        "class MyClass {
    companion object Factory {
        fun create(): MyClass = MyClass()
    }
}",
    );
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_property_parsing() {
    let source = SourceText::new(
        "class MyClass {
    private val constantValue = 42
    public var mutableValue: String = \"default\"
    lateinit var lateInitValue: String
    const val CONSTANT = \"constant\"
}",
    );
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_null_safety_parsing() {
    let source = SourceText::new(
        "fun processValue(value: String?) {
    val length = value?.length ?: 0
    println(length)
}",
    );
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}

#[test]
fn test_error_recovery() {
    let source = SourceText::new(
        "class MyClass {
    fun brokenFunction() {
        // Missing closing brace

    val validProperty = 42
}",
    );
    let lexer = KotlinLexer;
    let lex_output = lexer.lex(&source, &[], &mut NoLexerCache);
    assert!(lex_output.result.is_ok());

    let parser = KotlinParser;
    let mut cache = ParseSession::new(16);
    let parse_output = parser.parse(&source, &[], &mut cache);
    assert!(parse_output.result.is_ok());
}
