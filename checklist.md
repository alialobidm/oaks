# Checklist: Fix Missing Documentation for Oaks Project

## Pre-Implementation
- [ ] Review all files with missing documentation warnings
- [ ] Understand the context and purpose of each item

## Implementation

### oak-mojo/src/parser/element_type.rs
- [ ] Document `MojoElementType` enum
- [ ] Document token variants (Fn, Struct, Var, Let, etc.)
- [ ] Document statement variants (FunctionDef, StructDef, etc.)
- [ ] Document expression variants (BinaryExpr, UnaryExpr, etc.)
- [ ] Document component variants (ParamList, ArgList, Block)
- [ ] Document special variants (Root, Grouping, Error)

### oak-hlsl/src/parser/element_type.rs
- [ ] Document `HlslElementType` enum
- [ ] Document whitespace/comment variants
- [ ] Document literal variants
- [ ] Document data type variants (Bool, Int, Float, etc.)
- [ ] Document vector type variants (Bool2, Int3, Float4, etc.)
- [ ] Document matrix type variants (Float2x2, Double4x4, etc.)
- [ ] Document texture type variants
- [ ] Document sampler type variants
- [ ] Document buffer type variants
- [ ] Document control flow variants
- [ ] Document modifier variants
- [ ] Document preprocessor directive variants
- [ ] Document operator variants
- [ ] Document separator variants
- [ ] Document special token variants

## Post-Implementation
- [ ] Run `cargo check` - no missing documentation warnings
- [ ] Verify all documentation is in English
- [ ] Verify no postfix comments used
- [ ] Verify no code logic changes
