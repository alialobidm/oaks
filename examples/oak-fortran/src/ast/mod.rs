#![doc = include_str!("readme.md")]
use crate::{FortranLanguage, parser::element_type::FortranElementType};
use core::range::Range;
use oak_core::{GreenNode, RedNode};
use std::{boxed::Box, string::String, vec::Vec};
type SyntaxKind = FortranElementType;
type SyntaxNode<'a> = RedNode<'a, FortranLanguage>;
type FortranKind = FortranElementType;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A placeholder statement node in the Fortran AST.
pub struct Statement {
    // Placeholder fields
}

impl Statement {
    /// Attempts to cast a syntax node to a `Statement`.
    ///
    /// # Arguments
    ///
    /// * `_node` - The syntax node to cast.
    ///
    /// # Returns
    ///
    /// `Some(Statement)` if the cast succeeds, `None` otherwise.
    pub fn cast<'a>(_node: SyntaxNode<'a>) -> Option<Self> {
        Some(Statement {})
    }
}

/// The root node of the Fortran AST.
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FortranRoot {
    /// The optional name of the program.
    pub name: Option<String>,
    /// The list of program units contained in this root.
    pub units: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

impl FortranRoot {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == FortranKind::Program
    }

    fn cast(syntax: SyntaxNode<'_>) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            // TODO: Implement casting for units
            Some(Self { name: None, units: Vec::new(), span: syntax.span() })
        }
        else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode<'_> {
        // Implementation requires holding the SyntaxNode. Omitted for simplicity.
        unimplemented!("FortranRoot::syntax")
    }
}

/// Fortran program unit kinds
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ProgramUnitKind {
    /// Main program
    MainProgram(MainProgramNode),
    /// Subroutine
    Subroutine(SubroutineNode),
    /// Function
    Function(FunctionNode),
    /// Module
    Module(ModuleNode),
    /// Submodule
    Submodule(SubmoduleNode),
    /// Block data
    BlockData(BlockDataNode),
}

/// Main program node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MainProgramNode {
    /// The optional name of the main program.
    pub name: Option<String>,
    /// The specification statements in the main program.
    pub specification_part: Vec<SpecificationStmt>,
    /// The executable statements in the main program.
    pub execution_part: Vec<ExecutableStmt>,
    /// Internal subprograms defined within the main program.
    pub internal_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Subroutine node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubroutineNode {
    /// The name of the subroutine.
    pub name: String,
    /// The list of parameter names.
    pub parameters: Vec<String>,
    /// The specification statements in the subroutine.
    pub specification_part: Vec<SpecificationStmt>,
    /// The executable statements in the subroutine.
    pub execution_part: Vec<ExecutableStmt>,
    /// Internal subprograms defined within the subroutine.
    pub internal_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionNode {
    /// The name of the function.
    pub name: String,
    /// The list of parameter names.
    pub parameters: Vec<String>,
    /// The optional name of the result variable.
    pub result_name: Option<String>,
    /// The optional return type specification.
    pub return_type: Option<TypeSpec>,
    /// The specification statements in the function.
    pub specification_part: Vec<SpecificationStmt>,
    /// The executable statements in the function.
    pub execution_part: Vec<ExecutableStmt>,
    /// Internal subprograms defined within the function.
    pub internal_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ModuleNode {
    /// The name of the module.
    pub name: String,
    /// The specification statements in the module.
    pub specification_part: Vec<SpecificationStmt>,
    /// Subprograms defined within the module.
    pub module_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Submodule node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SubmoduleNode {
    /// The name of the parent module.
    pub parent_name: String,
    /// The name of the submodule.
    pub name: String,
    /// The specification statements in the submodule.
    pub specification_part: Vec<SpecificationStmt>,
    /// Subprograms defined within the submodule.
    pub module_subprograms: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Block data node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockDataNode {
    /// The optional name of the block data.
    pub name: Option<String>,
    /// The specification statements in the block data.
    pub specification_part: Vec<SpecificationStmt>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Specification statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpecificationStmt {
    /// Type declaration
    TypeDeclaration(TypeDeclarationNode),
    /// Parameter declaration
    Parameter(ParameterNode),
    /// Implicit declaration
    Implicit(ImplicitNode),
    /// Use statement
    Use(UseNode),
    /// Import statement
    Import(ImportNode),
    /// Interface declaration
    Interface(InterfaceNode),
    /// Procedure declaration
    Procedure(ProcedureNode),
    /// Generic declaration
    Generic(GenericNode),
}

/// Executable statement
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExecutableStmt {
    /// Assignment statement
    Assignment(AssignmentNode),
    /// Call statement
    Call(CallNode),
    /// If construct
    IfConstruct(IfConstructNode),
    /// Do construct
    DoConstruct(DoConstructNode),
    /// Select Case
    SelectCase(SelectCaseNode),
    /// Where construct
    WhereConstruct(WhereConstructNode),
    /// Forall construct
    ForallConstruct(ForallConstructNode),
    /// Associate construct
    AssociateConstruct(AssociateConstructNode),
    /// Block construct
    BlockConstruct(BlockConstructNode),
    /// Critical construct
    CriticalConstruct(CriticalConstructNode),
    /// Allocate statement
    Allocate(AllocateNode),
    /// Deallocate statement
    Deallocate(DeallocateNode),
    /// Nullify statement
    Nullify(NullifyNode),
    /// Stop statement
    Stop(StopNode),
    /// Return statement
    Return(ReturnNode),
    /// Continue statement
    Continue,
    /// Cycle statement
    Cycle(Option<String>),
    /// Exit statement
    Exit(Option<String>),
    /// Read statement
    Read(ReadNode),
    /// Write statement
    Write(WriteNode),
    /// Print statement
    Print(PrintNode),
}

/// Type specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TypeSpec {
    /// Integer type
    Integer(Option<KindSelector>),
    /// Real type
    Real(Option<KindSelector>),
    /// Double precision type
    DoublePrecision,
    /// Complex type
    Complex(Option<KindSelector>),
    /// Character type
    Character(Option<CharacterSelector>),
    /// Logical type
    Logical(Option<KindSelector>),
    /// Derived type
    Derived(String),
    /// Class type
    Class(String),
    /// Type star
    TypeStar,
}

/// Kind selector
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KindSelector {
    /// Expression
    Expression(Box<ExprNode>),
}

/// Character selector
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CharacterSelector {
    /// Length
    Length(Box<ExprNode>),
    /// Length and kind
    LengthAndKind(Box<ExprNode>, Box<ExprNode>),
}

/// Type declaration node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TypeDeclarationNode {
    /// The type specification for the declared entities.
    pub type_spec: TypeSpec,
    /// The attributes applied to the declared entities.
    pub attributes: Vec<Attribute>,
    /// The list of entity declarations.
    pub entities: Vec<EntityDecl>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Attribute
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Attribute {
    /// Allocatable
    Allocatable,
    /// Asynchronous
    Asynchronous,
    /// Bind
    Bind(String),
    /// Dimension
    Dimension(Vec<Dimension>),
    /// External
    External,
    /// Intent
    Intent(Intent),
    /// Intrinsic
    Intrinsic,
    /// Optional
    Optional,
    /// Parameter
    Parameter,
    /// Pointer
    Pointer,
    /// Protected
    Protected,
    /// Private
    Private,
    /// Public
    Public,
    /// Save
    Save,
    /// Target
    Target,
    /// Value
    Value,
    /// Volatile
    Volatile,
}

/// Intent
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Intent {
    /// In
    In,
    /// Out
    Out,
    /// InOut
    InOut,
}

/// Dimension
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Dimension {
    /// Explicit shape
    Explicit(Box<ExprNode>, Box<ExprNode>),
    /// Assumed shape
    Assumed(Option<Box<ExprNode>>),
    /// Deferred shape
    Deferred,
    /// Assumed size
    AssumedSize(Option<Box<ExprNode>>),
    /// Assumed rank
    AssumedRank,
}

/// Entity declaration
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EntityDecl {
    /// The name of the entity.
    pub name: String,
    /// The optional array specification.
    pub array_spec: Option<Vec<Dimension>>,
    /// The optional character length specification.
    pub char_length: Option<Box<ExprNode>>,
    /// The optional initialization expression.
    pub initialization: Option<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Parameter node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ParameterNode {
    /// The list of parameter entity declarations.
    pub entities: Vec<EntityDecl>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Implicit node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ImplicitNode {
    /// None
    None,
    /// Spec
    Spec(Vec<ImplicitSpec>),
}

/// Implicit specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImplicitSpec {
    /// The type specification for the implicit declaration.
    pub type_spec: TypeSpec,
    /// The letter ranges for the implicit declaration.
    pub letter_ranges: Vec<LetterRange>,
}

/// Letter range
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LetterRange {
    /// The starting letter of the range.
    pub start: char,
    /// The optional ending letter of the range.
    pub end: Option<char>,
}

/// Use node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UseNode {
    /// The name of the module to use.
    pub module_name: String,
    /// The optional module nature (intrinsic or non-intrinsic).
    pub nature: Option<ModuleNature>,
    /// The list of rename specifications.
    pub rename_list: Vec<Rename>,
    /// The list of only specifications.
    pub only_list: Vec<Only>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Module nature
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ModuleNature {
    /// Intrinsic
    Intrinsic,
    /// Non-intrinsic
    NonIntrinsic,
}

/// Rename specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rename {
    /// The local name to use.
    pub local_name: String,
    /// The original name in the module.
    pub use_name: String,
}

/// Only
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Only {
    /// Generic
    Generic(String),
    /// Rename
    Rename(Rename),
}

/// Import node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ImportNode {
    /// The list of names to import.
    pub import_names: Vec<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Interface node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceNode {
    /// The optional generic specification for the interface.
    pub generic_spec: Option<GenericSpec>,
    /// The interface bodies (procedure interfaces).
    pub interface_bodies: Vec<ProgramUnitKind>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Generic specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GenericSpec {
    /// Generic name
    GenericName(String),
    /// Operator
    Operator(String),
    /// Assignment
    Assignment,
    /// Read defined
    ReadDefined,
    /// Write defined
    WriteDefined,
}

/// Procedure node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureNode {
    /// The optional interface name for the procedure.
    pub interface_name: Option<String>,
    /// The attributes applied to the procedure.
    pub attributes: Vec<Attribute>,
    /// The procedure entity declarations.
    pub entities: Vec<ProcedureEntity>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Procedure entity
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcedureEntity {
    /// The name of the procedure.
    pub name: String,
    /// The optional binding name.
    pub binding_name: Option<String>,
}

/// Generic node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericNode {
    /// The generic specification.
    pub generic_spec: GenericSpec,
    /// The optional access specification.
    pub access_spec: Option<Attribute>,
    /// The list of procedure names bound to this generic.
    pub procedure_names: Vec<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Assignment node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssignmentNode {
    /// The left-hand side variable expression.
    pub variable: Box<ExprNode>,
    /// The right-hand side expression to assign.
    pub expression: Box<ExprNode>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Call node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CallNode {
    /// The name of the procedure to call.
    pub procedure_name: String,
    /// The arguments passed to the procedure.
    pub arguments: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// If construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IfConstructNode {
    /// The condition expression for the if block.
    pub condition: Box<ExprNode>,
    /// The statements in the then block.
    pub then_part: Vec<ExecutableStmt>,
    /// The else-if blocks with their conditions and statements.
    pub else_if_parts: Vec<(Box<ExprNode>, Vec<ExecutableStmt>)>,
    /// The optional else block statements.
    pub else_part: Option<Vec<ExecutableStmt>>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DoConstructNode {
    /// The optional construct name.
    pub name: Option<String>,
    /// The optional loop control (iterative, while, or concurrent).
    pub control: Option<DoControl>,
    /// The statements in the loop body.
    pub body: Vec<ExecutableStmt>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Do control
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DoControl {
    /// Iterative do loop with variable, start, end, and optional step.
    Iterative {
        /// The loop variable name.
        variable: String,
        /// The start value expression.
        start: Box<ExprNode>,
        /// The end value expression.
        end: Box<ExprNode>,
        /// The optional step value expression.
        step: Option<Box<ExprNode>>,
    },
    /// While loop with condition.
    While(Box<ExprNode>),
    /// Concurrent loop with header and locality specs.
    Concurrent {
        /// The concurrent header with control list and optional mask.
        header: ConcurrentHeader,
        /// The locality specifications.
        locality: Vec<LocalitySpec>,
    },
}

/// Concurrent header
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcurrentHeader {
    /// The list of concurrent controls.
    pub control_list: Vec<ConcurrentControl>,
    /// The optional mask expression.
    pub mask: Option<Box<ExprNode>>,
}

/// Concurrent control
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConcurrentControl {
    /// The index variable name.
    pub name: String,
    /// The start value expression.
    pub start: Box<ExprNode>,
    /// The end value expression.
    pub end: Box<ExprNode>,
    /// The optional step value expression.
    pub step: Option<Box<ExprNode>>,
}

/// Locality specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LocalitySpec {
    /// Local
    Local(Vec<String>),
    /// Local init
    LocalInit(Vec<String>),
    /// Shared
    Shared(Vec<String>),
    /// Default none
    DefaultNone,
}

/// Select Case node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SelectCaseNode {
    /// The expression to match against cases.
    pub expression: Box<ExprNode>,
    /// The list of case blocks.
    pub cases: Vec<CaseConstruct>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Case construct
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CaseConstruct {
    /// The case selector (specific values or default).
    pub selector: CaseSelector,
    /// The statements in this case block.
    pub body: Vec<ExecutableStmt>,
}

/// Case selector
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CaseSelector {
    /// Case
    Case(Vec<CaseValue>),
    /// Default
    Default,
}

/// Case value
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CaseValue {
    /// Single value
    Single(Box<ExprNode>),
    /// Range
    Range(Option<Box<ExprNode>>, Option<Box<ExprNode>>),
}

/// Where construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhereConstructNode {
    /// The mask expression for the where block.
    pub mask: Box<ExprNode>,
    /// The statements in the where block.
    pub where_body: Vec<ExecutableStmt>,
    /// The else-where blocks with optional masks and statements.
    pub else_where_parts: Vec<(Option<Box<ExprNode>>, Vec<ExecutableStmt>)>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Forall construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForallConstructNode {
    /// The concurrent header with control list and optional mask.
    pub header: ConcurrentHeader,
    /// The statements in the forall body.
    pub body: Vec<ExecutableStmt>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Associate construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssociateConstructNode {
    /// The list of associations.
    pub associates: Vec<Associate>,
    /// The statements in the associate body.
    pub body: Vec<ExecutableStmt>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Associate
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Associate {
    /// The name of the associate.
    pub name: String,
    /// The expression to associate with.
    pub expression: Box<ExprNode>,
}

/// Block construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BlockConstructNode {
    /// The specification statements in the block.
    pub specification_part: Vec<SpecificationStmt>,
    /// The executable statements in the block.
    pub execution_part: Vec<ExecutableStmt>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Critical construct node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CriticalConstructNode {
    /// The statements in the critical section.
    pub body: Vec<ExecutableStmt>,
    /// The optional construct name.
    pub name: Option<String>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Allocate node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AllocateNode {
    /// The objects to allocate.
    pub objects: Vec<Allocation>,
    /// The allocation options.
    pub options: Vec<AllocOpt>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Allocation
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Allocation {
    /// The variable to allocate.
    pub variable: Box<ExprNode>,
    /// The optional array specification.
    pub array_spec: Option<Vec<Dimension>>,
}

/// Allocation option
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AllocOpt {
    /// Stat
    Stat(Box<ExprNode>),
    /// Error message
    Errmsg(Box<ExprNode>),
    /// Source
    Source(Box<ExprNode>),
    /// Mold
    Mold(Box<ExprNode>),
}

/// Deallocate node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeallocateNode {
    /// The objects to deallocate.
    pub objects: Vec<Box<ExprNode>>,
    /// The deallocation options.
    pub options: Vec<DeallocOpt>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Deallocation option
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeallocOpt {
    /// Stat
    Stat(Box<ExprNode>),
    /// Error message
    Errmsg(Box<ExprNode>),
}

/// Nullify node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NullifyNode {
    /// The pointer objects to nullify.
    pub pointers: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Stop node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StopNode {
    /// The optional stop code expression.
    pub stop_code: Option<Box<ExprNode>>,
    /// The optional quiet expression.
    pub quiet: Option<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Return node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReturnNode {
    /// The optional return expression.
    pub expression: Option<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Read node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReadNode {
    /// The I/O control specifications.
    pub io_control_spec: Vec<IoControlSpec>,
    /// The input items to read into.
    pub input_items: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Write node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WriteNode {
    /// The I/O control specifications.
    pub io_control_spec: Vec<IoControlSpec>,
    /// The output items to write.
    pub output_items: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Print node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PrintNode {
    /// The optional format expression.
    pub format: Option<Box<ExprNode>>,
    /// The output items to print.
    pub output_items: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// IO control specification
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IoControlSpec {
    /// Unit
    Unit(Box<ExprNode>),
    /// Format
    Format(Box<ExprNode>),
    /// Nml
    Nml(Box<ExprNode>),
    /// Iomsg
    Iomsg(Box<ExprNode>),
    /// Iostat
    Iostat(Box<ExprNode>),
    /// Advance
    Advance(Box<ExprNode>),
    /// Other
    Other(String, Box<ExprNode>),
}

/// Expression node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ExprNode {
    /// Literal
    Literal(LiteralNode),
    /// Name
    Name(String),
    /// Array element
    ArrayElement(ArrayElementNode),
    /// Function reference
    FunctionReference(FunctionReferenceNode),
    /// Unary operation
    UnaryOp(UnaryOpNode),
    /// Binary operation
    BinaryOp(BinaryOpNode),
    /// Parenthesized expression
    ParenExpr(Box<ExprNode>),
    /// Structure constructor
    StructureConstructor(StructureConstructorNode),
}

/// Literal node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LiteralNode {
    /// The literal value as a string.
    pub value: String,
    /// The kind of literal (integer, real, complex, character, logical).
    pub kind: LiteralKind,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Literal kind
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LiteralKind {
    /// Integer
    Integer,
    /// Real
    Real,
    /// Complex
    Complex,
    /// Character
    Character,
    /// Logical
    Logical,
}

/// Array element node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ArrayElementNode {
    /// The name of the array.
    pub name: String,
    /// The subscript expressions.
    pub subscripts: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Function reference node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FunctionReferenceNode {
    /// The name of the function being referenced.
    pub name: String,
    /// The arguments passed to the function.
    pub arguments: Vec<Box<ExprNode>>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Unary operation node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnaryOpNode {
    /// The unary operator.
    pub operator: UnaryOperator,
    /// The operand expression.
    pub operand: Box<ExprNode>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Unary operator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnaryOperator {
    /// Not
    Not,
    /// Plus
    Plus,
    /// Minus
    Minus,
}

/// Binary operation node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BinaryOpNode {
    /// The binary operator.
    pub operator: BinaryOperator,
    /// The left-hand side expression.
    pub left: Box<ExprNode>,
    /// The right-hand side expression.
    pub right: Box<ExprNode>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}

/// Binary operator
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BinaryOperator {
    /// Add
    Add,
    /// Subtract
    Subtract,
    /// Multiply
    Multiply,
    /// Divide
    Divide,
    /// Power
    Power,
    /// Concat
    Concat,
    /// Equal
    Equal,
    /// Not equal
    NotEqual,
    /// Less than
    LessThan,
    /// Less than or equal
    LessThanOrEqual,
    /// Greater than
    GreaterThan,
    /// Greater than or equal
    GreaterThanOrEqual,
    /// And
    And,
    /// Or
    Or,
    /// Eqv
    Eqv,
    /// Neqv
    Neqv,
}

/// Structure constructor node
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StructureConstructorNode {
    /// The name of the derived type being constructed.
    pub type_name: String,
    /// The component initializers as (name, value) pairs.
    pub args: Vec<(Option<String>, Box<ExprNode>)>,
    /// The byte range of this node in the source text.
    #[cfg_attr(feature = "serde", serde(with = "oak_core::serde_range"))]
    pub span: Range<usize>,
}
