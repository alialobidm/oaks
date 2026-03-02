/// Valkyrie root node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieRoot {
    /// The collection of top-level items in the Valkyrie module.
    pub items: Vec<Item>,
}

/// Source code span
pub type Span = oak_core::Range<usize>;

/// Loop keyword kind for deprecation warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LoopKind {
    /// Using `loop` keyword (preferred).
    #[default]
    Loop,
    /// Using `for` keyword (deprecated, use `loop` instead).
    For,
}

/// Structure keyword kind for class-like definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StructureKind {
    /// Using `class` keyword.
    #[default]
    Class,
    /// Using `struct` keyword (deprecated, use `class` instead).
    Struct,
    /// Using `structure` keyword.
    Structure,
    /// Using `widget` keyword.
    Widget,
    /// Using `trait` keyword.
    Trait,
}

/// Enums keyword kind for deprecation warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnumsKind {
    /// Using `enums` keyword (preferred).
    #[default]
    Enums,
    /// Using `enum` keyword (deprecated, use `unity` instead).
    Enum,
    /// Using `unity` keyword (preferred alternative).
    Unity,
}

/// An identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// The identifier name as a string.
    pub name: String,
    /// The source code span where this identifier appears.
    pub span: Span,
}

impl Default for Identifier {
    fn default() -> Self {
        Self { name: String::new(), span: Span::default() }
    }
}

/// A name path (e.g., `std::collections::HashMap`)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NamePath {
    /// The individual identifier parts of the path.
    pub parts: Vec<Identifier>,
    /// The source code span covering the entire name path.
    pub span: Span,
}

/// An item in a Valkyrie module
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    /// A namespace declaration.
    Namespace(Namespace),
    /// A class declaration.
    Class(Class),
    /// A value type structure (immutable, copied on assignment).
    Structure(Class),
    /// A singleton declaration.
    Singleton(Singleton),
    /// A flags (bitflags) declaration.
    Flags(Flags),
    /// An enum declaration.
    Enums(Enums),
    /// A trait declaration.
    Trait(Trait),
    /// A widget declaration.
    Widget(Widget),
    /// A using (import) statement.
    Using(Using),
    /// A micro (small function) declaration.
    Micro(MicroDefinition),
    /// A type function declaration.
    TypeFunction(TypeFunction),
    /// A statement at module level.
    Statement(Statement),
    /// A variant declaration.
    Variant(Variant),
    /// An effect declaration.
    Effect(Effect),
    /// A property declaration (getter or setter).
    Property(Property),
    /// Template text content.
    TemplateText {
        /// The text content.
        content: String,
        /// The source code span.
        span: Span,
    },
    /// Template control structure.
    TemplateControl {
        /// The items within the control structure.
        items: Vec<Item>,
        /// The source code span.
        span: Span,
    },
    /// Template interpolation expression.
    TemplateInterpolation {
        /// The interpolated expression.
        expr: Expr,
        /// The source code span.
        span: Span,
    },
}

/// A namespace declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    /// The name path of the namespace.
    pub name: NamePath,
    /// Annotations applied to the namespace.
    pub annotations: Vec<Attribute>,
    /// Items declared within the namespace.
    pub items: Vec<Item>,
    /// The source code span.
    pub span: Span,
}

/// A parent class with optional alias for renamed inheritance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Parent {
    /// Optional alias for disambiguation (e.g., "primary" in "primary: Parent1").
    pub alias: Option<Identifier>,
    /// Parent class name path.
    pub name: NamePath,
    /// Source span.
    pub span: Span,
}

/// A class declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Class {
    /// The keyword kind used for this class (class, struct, structure, widget, trait).
    pub kind: StructureKind,
    /// The class name.
    pub name: Identifier,
    /// Generic parameters for the class.
    pub generics: Vec<GenericParam>,
    /// Parent classes or traits this class inherits from.
    pub parents: Vec<Parent>,
    /// Items (fields, methods) declared within the class.
    pub items: Vec<Item>,
    /// Annotations applied to the class.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
    /// Whether this class is abstract (cannot be instantiated directly).
    pub is_abstract: bool,
    /// Whether this class is sealed (restricted inheritance).
    pub is_sealed: bool,
    /// Whether this class is final (cannot be inherited).
    pub is_final: bool,
}

/// A flags (bitflags) declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Flags {
    /// The flags name.
    pub name: Identifier,
    /// The flag variants.
    pub variants: Vec<EnumVariant>,
    /// Annotations applied to the flags.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// An enum declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Enums {
    /// The keyword kind used for this enum (enums, enum, unity).
    pub kind: EnumsKind,
    /// The enum name.
    pub name: Identifier,
    /// Generic parameters for the enum.
    pub generics: Vec<GenericParam>,
    /// The enum variants.
    pub variants: Vec<EnumVariant>,
    /// Annotations applied to the enum.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// A trait declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Trait {
    /// The trait name.
    pub name: Identifier,
    /// Generic parameters for the trait.
    pub generics: Vec<GenericParam>,
    /// Methods declared in the trait.
    pub methods: Vec<Function>,
    /// Associated types declared in the trait.
    pub associated_types: Vec<AssociatedType>,
    /// Annotations applied to the trait.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// An associated type declaration in a trait.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AssociatedType {
    /// The associated type name.
    pub name: Identifier,
    /// Type bounds that the associated type must satisfy.
    pub bounds: Vec<Type>,
    /// Default type for the associated type, if any.
    pub default: Option<Type>,
    /// Annotations applied to the associated type.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// A widget declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Widget {
    /// The widget name.
    pub name: Identifier,
    /// Generic parameters for the widget.
    pub generics: Vec<GenericParam>,
    /// Items (properties, methods) declared within the widget.
    pub items: Vec<Item>,
    /// Annotations applied to the widget.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// A singleton declaration.
///
/// Singletons are classes that have exactly one instance globally.
/// They are useful for managing global state, configuration, or resources.
///
/// # Example
///
/// ```v
/// singleton GlobalConfig {
///     host: String = "localhost"
///     port: i32 = 8080
///
///     micro get_url(self) -> String {
///         f"{self.host}:{self.port}"
///     }
/// }
/// ```
///
/// # Semantics
///
/// - A singleton has exactly one global instance
/// - The instance is lazily initialized on first access
/// - Singleton members are accessed through the singleton name directly
/// - Singletons cannot be instantiated with constructors
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Singleton {
    /// The singleton name.
    pub name: Identifier,
    /// Generic parameters for the singleton.
    pub generics: Vec<GenericParam>,
    /// Parent traits this singleton implements.
    pub parents: Vec<Parent>,
    /// Items (fields, methods) declared within the singleton.
    pub items: Vec<Item>,
    /// Annotations applied to the singleton.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// A using (import) statement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Using {
    /// The path to import.
    pub path: NamePath,
    /// Optional alias for the import.
    pub alias: Option<Identifier>,
    /// The source code span.
    pub span: Span,
}

/// A micro (small function) declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MicroDefinition {
    /// The micro name.
    pub name: Identifier,
    /// Generic parameters for the micro.
    pub generics: Vec<GenericParam>,
    /// Parameters for the micro.
    pub params: Vec<Param>,
    /// Return type annotation, if any.
    pub return_type: Option<Type>,
    /// The body of the micro.
    pub body: Block,
    /// Annotations applied to the micro.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
    /// Whether this function is abstract (has no body implementation).
    pub is_abstract: bool,
    /// Whether this function is final (cannot be overridden).
    pub is_final: bool,
}

/// A type function declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFunction {
    /// The type function name.
    pub name: Identifier,
    /// Generic parameters for the type function.
    pub generics: Vec<GenericParam>,
    /// Parameters for the type function.
    pub params: Vec<Param>,
    /// Return type annotation, if any.
    pub return_type: Option<Type>,
    /// The body of the type function.
    pub body: Block,
    /// Annotations applied to the type function.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// A variant declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    /// The variant name.
    pub name: Identifier,
    /// Generic parameters for the variant.
    pub generics: Vec<GenericParam>,
    /// The variant cases.
    pub cases: Vec<VariantCase>,
    /// Annotations applied to the variant.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// An effect declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Effect {
    /// The effect name.
    pub name: Identifier,
    /// Operations defined by the effect.
    pub operations: Vec<Function>,
    /// Annotations applied to the effect.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// The kind of a property (getter or setter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyKind {
    /// A getter property.
    Getter,
    /// A setter property.
    Setter,
}

/// A property declaration (getter or setter).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property {
    /// The name of the property.
    pub name: Identifier,
    /// Whether this is a getter or setter.
    pub kind: PropertyKind,
    /// Generic parameters for the property.
    pub generics: Vec<GenericParam>,
    /// Annotations on the property.
    pub annotations: Vec<Attribute>,
    /// Parameters for the property (self for getter, self + value for setter).
    pub params: Vec<Param>,
    /// Return type for getter, None for setter.
    pub return_type: Option<Type>,
    /// The body of the property.
    pub body: Block,
    /// Source span.
    pub span: Span,
    /// Whether this property is abstract (has no body implementation).
    ///
    /// Abstract properties are declared without a body in abstract classes
    /// and must be implemented by concrete subclasses.
    pub is_abstract: bool,
    /// Whether this property is final (cannot be overridden).
    pub is_final: bool,
}

/// A statement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    /// A let binding statement.
    Let {
        /// Whether the binding is mutable.
        is_mutable: bool,
        /// The pattern to bind to.
        pattern: Pattern,
        /// The expression being bound.
        expr: Expr,
        /// Optional type annotation.
        ty: Option<Type>,
        /// Annotations applied to the statement.
        annotations: Vec<Attribute>,
        /// The source code span.
        span: Span,
    },
    /// An expression statement.
    ExprStmt {
        /// The expression.
        expr: Expr,
        /// Whether the statement ends with a semicolon.
        semi: bool,
        /// Annotations applied to the statement.
        annotations: Vec<Attribute>,
        /// The source code span.
        span: Span,
    },
}

/// An expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    /// An identifier expression.
    Ident(Identifier),
    /// A name path expression (e.g., `std::collections::HashMap`).
    Path(NamePath),
    /// A string literal expression.
    StringLiteral(StringLiteral),
    /// A boolean literal expression.
    Bool {
        /// The boolean value.
        value: bool,
        /// The source code span.
        span: Span,
    },
    /// A binary operation expression.
    Binary {
        /// The left operand.
        left: Box<Expr>,
        /// The binary operator.
        op: crate::kind::ValkyrieSyntaxKind,
        /// The right operand.
        right: Box<Expr>,
        /// The source code span.
        span: Span,
    },
    /// A unary operation expression.
    Unary {
        /// The unary operator.
        op: crate::kind::ValkyrieSyntaxKind,
        /// The operand expression.
        expr: Box<Expr>,
        /// The source code span.
        span: Span,
    },
    /// A function call expression.
    Call {
        /// The callee expression.
        callee: Box<Expr>,
        /// The call arguments.
        args: Vec<Expr>,
        /// The source code span.
        span: Span,
    },
    /// A field access expression.
    Field {
        /// The receiver expression.
        receiver: Box<Expr>,
        /// The field name.
        field: Identifier,
        /// The source code span.
        span: Span,
    },
    /// An index expression.
    Index {
        /// The receiver expression.
        receiver: Box<Expr>,
        /// The index expression.
        index: Box<Expr>,
        /// The source code span.
        span: Span,
    },
    /// A parenthesized expression.
    Paren {
        /// The inner expression.
        expr: Box<Expr>,
        /// The source code span.
        span: Span,
    },
    /// A block expression.
    Block(Block),
    /// A lambda expression.
    Lambda(LambdaExpr),
    /// An object expression.
    Object {
        /// The callee expression.
        callee: Box<Expr>,
        /// The object body block.
        block: Block,
        /// The source code span.
        span: Span,
    },
    /// Anonymous class expression.
    ///
    /// ```v
    /// let obj = class { x: 10, y: 20 }
    /// let impl_trait = class: Trait { ... }
    /// ```
    AnonymousClass {
        /// Parent traits or classes to implement/extend.
        parents: Vec<String>,
        /// Fields and methods defined in the anonymous class.
        items: Vec<Item>,
        /// Variables captured from the enclosing scope.
        captures: Vec<Identifier>,
        /// Source span.
        span: Span,
    },
    /// An if expression.
    If {
        /// Optional pattern for pattern-matching the condition.
        pattern: Option<Pattern>,
        /// The condition expression.
        condition: Box<Expr>,
        /// The then branch block.
        then_branch: Block,
        /// The optional else branch block.
        else_branch: Option<Block>,
        /// The source code span.
        span: Span,
    },
    /// A match expression.
    Match {
        /// The expression being matched.
        scrutinee: Box<Expr>,
        /// The match arms.
        arms: Vec<MatchArm>,
        /// The source code span.
        span: Span,
    },
    /// A loop expression.
    Loop {
        /// The loop keyword kind.
        kind: LoopKind,
        /// Optional label for the loop.
        label: Option<String>,
        /// Optional pattern for loop variable binding.
        pattern: Option<Pattern>,
        /// Optional condition for conditional loops.
        condition: Option<Box<Expr>>,
        /// The loop body.
        body: Block,
        /// The source code span.
        span: Span,
    },
    /// A return expression.
    Return {
        /// The optional return value expression.
        expr: Option<Box<Expr>>,
        /// The source code span.
        span: Span,
    },
    /// A break expression.
    Break {
        /// Optional label of the loop to break from.
        label: Option<String>,
        /// Optional value to break with.
        expr: Option<Box<Expr>>,
        /// The source code span.
        span: Span,
    },
    /// A continue expression.
    Continue {
        /// Optional label of the loop to continue.
        label: Option<String>,
        /// The source code span.
        span: Span,
    },
    /// A yield expression.
    Yield {
        /// The optional value to yield.
        expr: Option<Box<Expr>>,
        /// Whether this is a yield from expression.
        yield_from: bool,
        /// The source code span.
        span: Span,
    },
    /// A raise (throw) expression.
    Raise {
        /// The expression to raise.
        expr: Box<Expr>,
        /// The source code span.
        span: Span,
    },
    /// A catch (try-catch) expression.
    Catch {
        /// The expression to try.
        expr: Box<Expr>,
        /// The catch arms.
        arms: Vec<MatchArm>,
        /// The source code span.
        span: Span,
    },
    /// With expression for functional record updates.
    ///
    /// Creates a new record by copying an existing one and updating specified fields.
    ///
    /// ```v
    /// let p2 = p1.with { x: 20.0, y: 30.0 }
    /// let updated = config.with { timeout: 60 }
    /// ```
    With {
        /// The base expression to copy from.
        base: Box<Expr>,
        /// Field updates to apply.
        updates: Vec<(Identifier, Expr)>,
        /// Source span.
        span: Span,
    },
}

/// A block of statements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    /// The statements in the block.
    pub statements: Vec<Statement>,
    /// The source code span.
    pub span: Span,
}

/// A lambda expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaExpr {
    /// The lambda parameters.
    pub params: Vec<Param>,
    /// Optional return type annotation.
    pub return_type: Option<Type>,
    /// The lambda body.
    pub body: Block,
    /// The source code span.
    pub span: Span,
}

/// A match arm
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {
    /// The pattern to match against.
    pub pattern: Pattern,
    /// Optional guard expression.
    pub guard: Option<Expr>,
    /// The body expression of the arm.
    pub body: Expr,
    /// The source code span.
    pub span: Span,
}

/// A pattern for matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    /// A wildcard pattern that matches anything.
    Wildcard {
        /// The source code span.
        span: Span,
    },
    /// A variable pattern that binds the matched value.
    Variable {
        /// The variable name.
        name: Identifier,
        /// The source code span.
        span: Span,
    },
    /// A literal pattern.
    Literal {
        /// The literal value as a string.
        value: String,
        /// The source code span.
        span: Span,
    },
    /// A type pattern for matching types.
    Type {
        /// The type name path.
        name: NamePath,
        /// The source code span.
        span: Span,
    },
    /// A class pattern for destructuring.
    Class {
        /// The class name path.
        name: NamePath,
        /// The field patterns.
        fields: Vec<(Identifier, Pattern)>,
        /// The source code span.
        span: Span,
    },
    /// An else pattern (catch-all).
    Else {
        /// The source code span.
        span: Span,
    },
}

/// A type expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    /// A named type (e.g., `String`, `i32`).
    Named {
        /// The type name path.
        path: NamePath,
        /// The source code span.
        span: Span,
    },
    /// A generic type parameter.
    Generic {
        /// The generic parameter name.
        name: Identifier,
        /// The source code span.
        span: Span,
    },
    /// A tuple type.
    Tuple {
        /// The element types.
        elements: Vec<Type>,
        /// The source code span.
        span: Span,
    },
    /// A function type.
    Function {
        /// The parameter types.
        params: Vec<Type>,
        /// The return type.
        return_type: Box<Type>,
        /// The source code span.
        span: Span,
    },
    /// An optional type.
    Optional {
        /// The inner type.
        inner: Box<Type>,
        /// The source code span.
        span: Span,
    },
    /// An associated type projection (e.g., `Self::Item`, `T::Output`).
    AssociatedType {
        /// The base type (e.g., `Self` or a type parameter name).
        base: Identifier,
        /// The associated type name.
        name: Identifier,
        /// The source code span.
        span: Span,
    },
    /// A qualified associated type (e.g., `<T as Trait>::Item`).
    QualifiedAssociatedType {
        /// The type being projected from.
        ty: Box<Type>,
        /// The trait providing the associated type.
        trait_path: NamePath,
        /// The associated type name.
        name: Identifier,
        /// The source code span.
        span: Span,
    },
}

/// A generic parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericParam {
    /// The generic parameter name.
    pub name: Identifier,
    /// Type constraints (bounds) for the generic parameter.
    pub constraints: Vec<Type>,
    /// Default type for the generic parameter.
    pub default: Option<Type>,
    /// The source code span.
    pub span: Span,
}

/// A function parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    /// The parameter name.
    pub name: Identifier,
    /// Optional type annotation.
    pub ty: Option<Type>,
    /// Optional default value expression.
    pub default: Option<Expr>,
    /// The source code span.
    pub span: Span,
}

/// A field in a class or struct
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    /// The field name.
    pub name: Identifier,
    /// The field type.
    pub ty: Type,
    /// Optional default value expression.
    pub default: Option<Expr>,
    /// Annotations applied to the field.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
}

/// A function definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    /// The function name.
    pub name: Identifier,
    /// Generic parameters for the function.
    pub generics: Vec<GenericParam>,
    /// The function parameters.
    pub params: Vec<Param>,
    /// Optional return type annotation.
    pub return_type: Option<Type>,
    /// The optional function body.
    pub body: Option<Block>,
    /// Annotations applied to the function.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
    /// Whether this function is abstract (has no body implementation).
    pub is_abstract: bool,
    /// Whether this function is final (cannot be overridden).
    pub is_final: bool,
}

/// An enum variant
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumVariant {
    /// The variant name.
    pub name: Identifier,
    /// The variant fields.
    pub fields: Vec<Field>,
    /// Annotations applied to the variant.
    pub annotations: Vec<Attribute>,
    /// The source code span.
    pub span: Span,
    /// Optional value expression for flags (e.g., `READ = 1` or `ALL = READ | WRITE`).
    pub value: Option<Expr>,
}

/// A variant case
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantCase {
    /// The pattern for this case.
    pub pattern: Pattern,
    /// The body expression for this case.
    pub body: Expr,
    /// The source code span.
    pub span: Span,
}

/// An attribute
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    /// The attribute name.
    pub name: Identifier,
    /// The attribute arguments.
    pub args: Vec<Expr>,
    /// The source code span.
    pub span: Span,
}

/// A string literal node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringLiteral {
    /// DSL prefix (e.g., `s`, `f`, `r`, `sql`).
    pub prefix: Option<Identifier>,
    /// Number of quotes (1, 2, 3, 4, ...).
    pub quote_count: u8,
    /// String segments.
    pub segments: Vec<StringSegment>,
    /// The source code span.
    pub span: Span,
}

/// A string segment.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringSegment {
    /// Text content.
    Text {
        /// The text content.
        content: String,
        /// The source code span.
        span: Span,
    },
    /// Interpolation expression.
    Interpolation {
        /// The interpolation expression.
        expr: Box<Expr>,
        /// Whether this is a Fluent variable (with the ߷ marker).
        is_fluent: bool,
        /// The source code span.
        span: Span,
    },
}
