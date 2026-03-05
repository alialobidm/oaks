# Oak Pretty Print - Format Config Params - The Implementation Plan

## [ ] Task 1: Modify AsDocument Trait to Accept Parameters
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - Add a new associated type `Params` to the `AsDocument` trait
  - Add a default implementation for `Params` as `()`
  - Modify the `as_document` method to accept `&self, params: &Self::Params`
  - Update all existing implementations to use the new signature
- **Acceptance Criteria Addressed**: AC-1, AC-3
- **Test Requirements**: 
  - `programmatic` TR-1.1: Existing code compiles without changes
  - `programmatic` TR-1.2: New code can pass parameters to `as_document`
- **Notes**: Use default associated types to maintain backward compatibility

## [ ] Task 2: Update FormatContext to Support Parameters
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - Add a `params` field to `FormatContext`
  - Update the `enter` method to propagate parameters
  - Modify the formatter to pass parameters through the formatting process
- **Acceptance Criteria Addressed**: AC-1, AC-2
- **Test Requirements**: 
  - `programmatic` TR-2.1: Parameters are correctly passed through the formatting process
  - `programmatic` TR-2.2: Parameters can override global configuration
- **Notes**: Use a generic type parameter for `Params`

## [ ] Task 3: Implement Inline Configuration Support
- **Priority**: P1
- **Depends On**: Task 2
- **Description**: 
  - Add a mechanism to store inline configuration in the AST
  - Implement a way to parse and apply inline configuration
  - Update the formatter to use inline configuration when available
- **Acceptance Criteria Addressed**: AC-2
- **Test Requirements**: 
  - `programmatic` TR-3.1: Inline configuration overrides global configuration
  - `programmatic` TR-3.2: Multiple inline configurations are handled correctly
- **Notes**: The actual parsing of inline annotations will be language-specific

## [ ] Task 4: Update Documentation
- **Priority**: P2
- **Depends On**: Tasks 1-3
- **Description**: 
  - Update the documentation for the `AsDocument` trait
  - Add examples of using parameters and inline configuration
  - Document the new API features
- **Acceptance Criteria Addressed**: AC-4
- **Test Requirements**: 
  - `human-judgment` TR-4.1: Documentation is clear and complete
  - `human-judgment` TR-4.2: Examples are helpful and correct
- **Notes**: Focus on explaining the new features and how to use them

## [ ] Task 5: Test and Verify
- **Priority**: P1
- **Depends On**: Tasks 1-4
- **Description**: 
  - Write tests for the new features
  - Test backward compatibility
  - Verify that all acceptance criteria are met
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-3
- **Test Requirements**: 
  - `programmatic` TR-5.1: All tests pass
  - `programmatic` TR-5.2: Existing code compiles and works as before
- **Notes**: Use the existing test infrastructure