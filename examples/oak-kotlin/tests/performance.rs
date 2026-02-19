use oak_core::{Parser, parser::ParseSession};
use oak_kotlin::{Kotlin, KotlinParser};
use std::time::Instant;

#[test]
fn test_kotlin_parser_performance() {
    // Generate a large Kotlin file for testing
    let mut large_kotlin_file = String::new();

    // Add package declaration
    large_kotlin_file.push_str(
        "package com.example

",
    );

    // Add imports
    large_kotlin_file.push_str(
        "import java.util.ArrayList
import java.util.List
import java.util.Map
import java.util.HashMap

",
    );

    // Add a large class with many methods
    large_kotlin_file.push_str(
        "class LargeClass {
",
    );

    // Add many properties
    for i in 0..1000 {
        large_kotlin_file.push_str(&format!(
            "    private var field{}: Int = 0
",
            i
        ));
    }

    // Add many methods
    for i in 0..1000 {
        large_kotlin_file.push_str(&format!(
            "    fun method{}() {{
        for (j in 0 until 100) {{
            field{} += j
        }}
    }}

",
            i, i
        ));
    }

    large_kotlin_file.push_str(
        "}
",
    );

    println!("Generated Kotlin file size: {} bytes", large_kotlin_file.len());

    let parser = KotlinParser;
    let mut session = ParseSession::new(1024);

    // Measure parsing time
    let start = Instant::now();
    let result = parser.parse(large_kotlin_file.as_str(), &[], &mut session);
    let duration = start.elapsed();

    println!("Parsing took: {:?}", duration);
    println!("Parse result: {:?}", result.result.is_ok());

    // Ensure the parser doesn't panic
    assert!(result.result.is_ok());
}
