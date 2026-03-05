use std::collections::HashMap;
use alloc::string::String;

/// Formatting state
/// 
/// This struct holds the dynamic state used during the formatting process,
/// including local configuration overrides from inline annotations.
#[derive(Debug, Clone, Default)]
pub struct FormatState {
    /// Local configuration overrides
    pub local_config: HashMap<String, serde_json::Value>,
    /// Custom state values
    pub custom_state: HashMap<String, serde_json::Value>,
    /// Current indentation level
    pub indent_level: usize,
    /// Whether to force single line formatting
    pub force_single_line: bool,
    /// Whether to align elements
    pub align_elements: bool,
}

impl FormatState {
    /// Creates a new format state
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a local configuration override
    pub fn set_local_config(&mut self, key: &str, value: serde_json::Value) {
        self.local_config.insert(key.to_string(), value);
    }

    /// Gets a local configuration value
    pub fn get_local_config(&self, key: &str) -> Option<&serde_json::Value> {
        self.local_config.get(key)
    }

    /// Sets a custom state value
    pub fn set_custom_state(&mut self, key: &str, value: serde_json::Value) {
        self.custom_state.insert(key.to_string(), value);
    }

    /// Gets a custom state value
    pub fn get_custom_state(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom_state.get(key)
    }

    /// Sets the indentation level
    pub fn set_indent_level(&mut self, level: usize) {
        self.indent_level = level;
    }

    /// Gets the indentation level
    pub fn get_indent_level(&self) -> usize {
        self.indent_level
    }

    /// Sets whether to force single line formatting
    pub fn set_force_single_line(&mut self, force: bool) {
        self.force_single_line = force;
    }

    /// Gets whether to force single line formatting
    pub fn get_force_single_line(&self) -> bool {
        self.force_single_line
    }

    /// Sets whether to align elements
    pub fn set_align_elements(&mut self, align: bool) {
        self.align_elements = align;
    }

    /// Gets whether to align elements
    pub fn get_align_elements(&self) -> bool {
        self.align_elements
    }

    /// Creates a new state with the same values
    pub fn clone_with(&self) -> Self {
        self.clone()
    }

    /// Creates a new state with updated values
    pub fn with_indent_level(mut self, level: usize) -> Self {
        self.indent_level = level;
        self
    }

    /// Creates a new state with force single line set
    pub fn with_force_single_line(mut self, force: bool) -> Self {
        self.force_single_line = force;
        self
    }

    /// Creates a new state with align elements set
    pub fn with_align_elements(mut self, align: bool) -> Self {
        self.align_elements = align;
        self
    }
}
