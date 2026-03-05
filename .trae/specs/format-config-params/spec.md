# Oak Pretty Print - Format Config Params - Product Requirement Document

## Overview
- **Summary**: Enhance the Oak Pretty Print library to support formatting parameters and inline configuration similar to #[rustfmt].
- **Purpose**: Provide a more flexible and powerful formatting system that allows users to customize formatting behavior per node and through inline annotations.
- **Target Users**: Language developers using the Oak framework to implement formatters for their languages.

## Goals
- Enable passing formatting parameters to the `as_document` method
- Implement support for inline configuration annotations similar to #[rustfmt]
- Maintain backward compatibility with existing code
- Provide a clean and extensible API for format configuration

## Non-Goals (Out of Scope)
- Rewriting the entire pretty-printing system
- Adding support for specific language features not related to formatting configuration
- Implementing a specific parser for inline annotations (this will be language-specific)

## Background & Context
The current Oak Pretty Print library uses a fixed `FormatConfig` passed to the `render` method, which applies globally to the entire document. This design doesn't allow for per-node formatting customization or inline configuration annotations.

## Functional Requirements
- **FR-1**: Modify the `AsDocument` trait to accept formatting parameters
- **FR-2**: Implement a mechanism to pass formatting parameters through the formatting process
- **FR-3**: Add support for inline configuration annotations
- **FR-4**: Provide a way to override global configuration with local parameters

## Non-Functional Requirements
- **NFR-1**: Maintain backward compatibility with existing code
- **NFR-2**: Keep the API clean and easy to use
- **NFR-3**: Ensure performance is not significantly impacted
- **NFR-4**: Provide clear documentation for the new features

## Constraints
- **Technical**: Must work with the existing Document and FormatConfig structures
- **Dependencies**: Must not introduce new dependencies

## Assumptions
- The inline configuration annotations will be parsed by the language-specific parser
- The formatting parameters will be language-agnostic

## Acceptance Criteria

### AC-1: AsDocument Trait Supports Parameters
- **Given**: A type implementing the AsDocument trait
- **When**: The as_document method is called with parameters
- **Then**: The method should use these parameters to customize formatting
- **Verification**: `programmatic`

### AC-2: Inline Configuration Support
- **Given**: A node with inline configuration annotations
- **When**: The node is formatted
- **Then**: The inline configuration should override the global configuration
- **Verification**: `programmatic`

### AC-3: Backward Compatibility
- **Given**: Existing code using the current AsDocument trait
- **When**: The code is compiled with the new version
- **Then**: It should compile without errors
- **Verification**: `programmatic`

### AC-4: API Clarity
- **Given**: A developer using the new API
- **When**: They read the documentation
- **Then**: They should understand how to use the new features
- **Verification**: `human-judgment`

## Open Questions
- [ ] How should inline configuration annotations be represented in the AST?
- [ ] What is the best way to pass parameters through the formatting process?