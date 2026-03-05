use alloc::borrow::Cow;
use oak_pretty_print::{IndentStyle, LineEnding};

/// Common formatting configuration that can be shared across languages
/// 
/// This struct provides common formatting options that are applicable to most
/// programming languages. Language-specific formatters can use this as a base
/// and add their own specific options.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct CommonFormatterConfig {
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
}

impl Default for CommonFormatterConfig {
    fn default() -> Self {
        let indent_style = IndentStyle::default();
        let (indent_text, indent_size) = match indent_style {
            IndentStyle::Spaces(count) => (" ".repeat(count as usize).into(), count as usize),
            IndentStyle::Tabs => ("\t".into(), 4),
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
        }
    }
}

impl CommonFormatterConfig {
    /// Returns the appropriate line ending string based on the configuration
    pub fn line_ending_string(&self) -> &'static str {
        match self.line_ending {
            LineEnding::Unix => "\n",
            LineEnding::Windows => "\r\n",
            LineEnding::Auto => {
                #[cfg(windows)]
                return "\r\n";
                #[cfg(not(windows))]
                return "\n";
            }
        }
    }
}
