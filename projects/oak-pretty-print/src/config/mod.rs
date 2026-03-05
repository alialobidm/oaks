use alloc::borrow::Cow;

/// Indent style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum IndentStyle {
    /// Use spaces
    Spaces(u8),
    /// Use tabs
    Tabs,
}

impl Default for IndentStyle {
    fn default() -> Self {
        IndentStyle::Spaces(4)
    }
}

/// Line ending
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum LineEnding {
    /// Unix style (\n)
    Unix,
    /// Windows style (\r\n)
    Windows,
    /// Auto detect
    Auto,
}

impl Default for LineEnding {
    fn default() -> Self {
        LineEnding::Auto
    }
}

/// Formatter configuration trait
/// 
/// Each language should implement this trait for its specific configuration structure.
pub trait FormatterConfig {
    /// The type of state used during formatting
    type State: Default + Clone;
    
    /// Creates a new default configuration
    fn new() -> Self;
    
    /// Creates a default state from this configuration
    fn state(&self) -> Self::State;
    
    /// Gets the indent style
    fn indent_style(&self) -> IndentStyle;
    
    /// Gets the line ending
    fn line_ending(&self) -> LineEnding;
    
    /// Gets the maximum line length
    fn max_width(&self) -> usize;
    
    /// Gets the line ending string
    fn line_ending_string(&self) -> &'static str;
}

/// Default formatter configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct FormatConfig {
    /// Indent style
    pub indent_style: IndentStyle,
    /// Indent text (cached single-level indent string)
    pub indent_text: Cow<'static, str>,
    /// Line ending
    pub line_ending: LineEnding,
    /// Maximum line length
    pub max_width: usize,
    /// Whether to insert a final newline at the end of the file
    pub insert_final_newline: bool,
    /// Whether to trim trailing whitespace
    pub trim_trailing_whitespace: bool,
    /// Whether to preserve blank lines
    pub preserve_blank_lines: bool,
    /// Maximum consecutive blank lines
    pub max_blank_lines: usize,
    /// Whether to format comments
    pub format_comments: bool,
    /// Whether to format strings
    pub format_strings: bool,
    /// Indent size (used for column calculation)
    pub indent_size: usize,
    /// Whether to align arguments in function calls
    pub align_function_args: bool,
    /// Whether to align fields in struct definitions
    pub align_struct_fields: bool,
    /// Whether to use spaces around operators
    pub spaces_around_operators: bool,
    /// Whether to use spaces inside parentheses
    pub spaces_inside_parentheses: bool,
    /// Whether to use spaces inside brackets
    pub spaces_inside_brackets: bool,
    /// Whether to use spaces inside braces
    pub spaces_inside_braces: bool,
    /// Whether to prefer single quotes for strings
    pub prefer_single_quotes: bool,
    /// Whether to sort imports
    pub sort_imports: bool,
    /// Whether to remove unused imports
    pub remove_unused_imports: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        let indent_style = IndentStyle::default();
        let (indent_text, indent_size) = match indent_style {
            IndentStyle::Spaces(count) => (" ".repeat(count as usize).into(), count as usize),
            IndentStyle::Tabs => ("\t".into(), 4), // Default tab size for column calculation
        };

        Self {
            indent_style,
            indent_text,
            line_ending: LineEnding::default(),
            max_width: 100,
            insert_final_newline: true,
            trim_trailing_whitespace: true,
            preserve_blank_lines: true,
            max_blank_lines: 2,
            format_comments: true,
            format_strings: false,
            indent_size,
            align_function_args: false,
            align_struct_fields: false,
            spaces_around_operators: true,
            spaces_inside_parentheses: false,
            spaces_inside_brackets: false,
            spaces_inside_braces: false,
            prefer_single_quotes: false,
            sort_imports: false,
            remove_unused_imports: false,
        }
    }
}

impl FormatterConfig for FormatConfig {
    type State = crate::state::FormatState;
    
    fn new() -> Self {
        Self::default()
    }
    
    fn state(&self) -> Self::State {
        crate::state::FormatState::default()
    }
    
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }
    
    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }
    
    fn max_width(&self) -> usize {
        self.max_width
    }
    
    fn line_ending_string(&self) -> &'static str {
        match self.line_ending {
            LineEnding::Unix => "\n",
            LineEnding::Windows => "\r\n",
            LineEnding::Auto => {
                // In actual use, it should be detected based on the input file
                #[cfg(windows)]
                return "\r\n";
                #[cfg(not(windows))]
                return "\n";
            }
        }
    }
}

impl FormatConfig {
    /// Sets the indent style
    pub fn with_indent_style(mut self, style: IndentStyle) -> Self {
        self.indent_style = style;
        let (indent_text, indent_size) = match style {
            IndentStyle::Spaces(count) => (" ".repeat(count as usize).into(), count as usize),
            IndentStyle::Tabs => ("\t".into(), 4),
        };
        self.indent_text = indent_text;
        self.indent_size = indent_size;
        self
    }

    /// Sets the line ending
    pub fn with_line_ending(mut self, ending: LineEnding) -> Self {
        self.line_ending = ending;
        self
    }

    /// Sets the maximum line length
    pub fn with_max_width(mut self, length: usize) -> Self {
        self.max_width = length;
        self
    }
}
