use alloc::borrow::Cow;
use oak_pretty_print::{FormatterConfig, IndentStyle, LineEnding};

/// Rust formatter configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RustFormatterConfig {
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
    /// Whether to sort imports
    pub sort_imports: bool,
    /// Whether to remove unused imports
    pub remove_unused_imports: bool,
}

impl Default for RustFormatterConfig {
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
            indent_size,
            align_function_args: false,
            align_struct_fields: false,
            spaces_around_operators: true,
            spaces_inside_parentheses: false,
            spaces_inside_brackets: false,
            spaces_inside_braces: false,
            sort_imports: false,
            remove_unused_imports: false,
        }
    }
}

impl FormatterConfig for RustFormatterConfig {
    type State = RustFormatterState;
    
    fn new() -> Self {
        Self::default()
    }
    
    fn state(&self) -> Self::State {
        RustFormatterState::default()
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

/// Rust formatter state
#[derive(Debug, Clone, Default)]
pub struct RustFormatterState {
    /// Whether to force single line formatting
    pub force_single_line: bool,
    /// Current indent level
    pub indent_level: usize,
    /// Whether to format comments
    pub format_comments: bool,
    /// Local configuration overrides
    pub local_config: std::collections::HashMap<String, serde_json::Value>,
    /// Custom state values
    pub custom_state: std::collections::HashMap<String, serde_json::Value>,
    /// Whether to align elements
    pub align_elements: bool,
}

impl oak_pretty_print::FormatState for RustFormatterState {
    fn set_local_config(&mut self, key: &str, value: serde_json::Value) {
        self.local_config.insert(key.to_string(), value);
    }
    
    fn get_local_config(&self, key: &str) -> Option<&serde_json::Value> {
        self.local_config.get(key)
    }
    
    fn set_custom_state(&mut self, key: &str, value: serde_json::Value) {
        self.custom_state.insert(key.to_string(), value);
    }
    
    fn get_custom_state(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom_state.get(key)
    }
    
    fn set_indent_level(&mut self, level: usize) {
        self.indent_level = level;
    }
    
    fn get_indent_level(&self) -> usize {
        self.indent_level
    }
    
    fn set_force_single_line(&mut self, force: bool) {
        self.force_single_line = force;
    }
    
    fn get_force_single_line(&self) -> bool {
        self.force_single_line
    }
    
    fn set_align_elements(&mut self, align: bool) {
        self.align_elements = align;
    }
    
    fn get_align_elements(&self) -> bool {
        self.align_elements
    }
}

/// TypeScript formatter configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct TypeScriptFormatterConfig {
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

impl Default for TypeScriptFormatterConfig {
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

impl FormatterConfig for TypeScriptFormatterConfig {
    type State = TypeScriptFormatterState;
    
    fn new() -> Self {
        Self::default()
    }
    
    fn state(&self) -> Self::State {
        TypeScriptFormatterState::default()
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

/// TypeScript formatter state
#[derive(Debug, Clone, Default)]
pub struct TypeScriptFormatterState {
    /// Whether to force single line formatting
    pub force_single_line: bool,
    /// Current indent level
    pub indent_level: usize,
    /// Whether to format comments
    pub format_comments: bool,
    /// Whether to format strings
    pub format_strings: bool,
    /// Local configuration overrides
    pub local_config: std::collections::HashMap<String, serde_json::Value>,
    /// Custom state values
    pub custom_state: std::collections::HashMap<String, serde_json::Value>,
    /// Whether to align elements
    pub align_elements: bool,
}

impl oak_pretty_print::FormatState for TypeScriptFormatterState {
    fn set_local_config(&mut self, key: &str, value: serde_json::Value) {
        self.local_config.insert(key.to_string(), value);
    }
    
    fn get_local_config(&self, key: &str) -> Option<&serde_json::Value> {
        self.local_config.get(key)
    }
    
    fn set_custom_state(&mut self, key: &str, value: serde_json::Value) {
        self.custom_state.insert(key.to_string(), value);
    }
    
    fn get_custom_state(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom_state.get(key)
    }
    
    fn set_indent_level(&mut self, level: usize) {
        self.indent_level = level;
    }
    
    fn get_indent_level(&self) -> usize {
        self.indent_level
    }
    
    fn set_force_single_line(&mut self, force: bool) {
        self.force_single_line = force;
    }
    
    fn get_force_single_line(&self) -> bool {
        self.force_single_line
    }
    
    fn set_align_elements(&mut self, align: bool) {
        self.align_elements = align;
    }
    
    fn get_align_elements(&self) -> bool {
        self.align_elements
    }
}
