use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde_json;

/// Format annotation
/// 
/// This struct represents a formatting annotation extracted from the code.
#[derive(Debug, Clone)]
pub struct FormatAnnotation {
    /// The name of the annotation
    pub name: String,
    /// The parameters of the annotation
    pub params: Vec<AnnotationParam>,
    /// The span of the annotation in the source code
    pub span: core::ops::Range<usize>,
}

/// Annotation parameter
/// 
/// This struct represents a parameter of a formatting annotation.
#[derive(Debug, Clone)]
pub struct AnnotationParam {
    /// The name of the parameter
    pub name: String,
    /// The value of the parameter
    pub value: AnnotationValue,
}

/// Annotation value
/// 
/// This enum represents the value of an annotation parameter.
#[derive(Debug, Clone)]
pub enum AnnotationValue {
    /// A boolean value
    Bool(bool),
    /// An integer value
    Int(i64),
    /// A string value
    String(String),
    /// A float value
    Float(f64),
    /// A list of values
    List(Vec<AnnotationValue>),
    /// A map of key-value pairs
    Map(Vec<(String, AnnotationValue)>),
}

/// Annotation parser
/// 
/// This trait defines the interface for parsing annotations from different languages.
pub trait AnnotationParser {
    /// Parses annotations from the source code
    /// 
    /// # Parameters
    /// - `source`: The source code string
    /// 
    /// # Returns
    /// A vector of `FormatAnnotation` objects
    fn parse(&self, source: &str) -> Vec<FormatAnnotation>;
}

/// Rust annotation parser
/// 
/// This struct parses Rust-specific annotations like `#[rustfmt::xxx]`.
pub struct RustAnnotationParser;

impl AnnotationParser for RustAnnotationParser {
    fn parse(&self, source: &str) -> Vec<FormatAnnotation> {
        use core::ops::Range;
        let mut annotations = Vec::new();
        
        #[cfg(feature = "regex")]
        {
            // Regular expression to match Rust annotations like `#[rustfmt::xxx]` or `#[rustfmt(xxx)]`
            let regex = regex::Regex::new(r"#\[rustfmt(?::([a-zA-Z_][a-zA-Z0-9_]*))?\s*(?:\(([^\)]*)\))?\]").unwrap();
            
            for capture in regex.captures_iter(source) {
                let full_match = capture.get(0).unwrap();
                let span = Range {
                    start: full_match.start(),
                    end: full_match.end(),
                };
                
                let name = capture.get(1).map(|m| m.as_str().to_string()).unwrap_or_else(|| "default".to_string());
                let params_str = capture.get(2).map(|m| m.as_str()).unwrap_or("");
                
                let params = self.parse_params(params_str);
                
                annotations.push(FormatAnnotation {
                    name,
                    params,
                    span,
                });
            }
        }
        
        annotations
    }
}

impl RustAnnotationParser {
    /// Parses annotation parameters
    fn parse_params(&self, params_str: &str) -> Vec<AnnotationParam> {
        let mut params = Vec::new();
        
        // Simple parameter parsing for now
        let param_pairs = params_str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty());
        
        for pair in param_pairs {
            if let Some((name, value_str)) = pair.split_once('=') {
                let name = name.trim().to_string();
                let value = self.parse_value(value_str.trim());
                params.push(AnnotationParam {
                    name,
                    value,
                });
            }
        }
        
        params
    }
    
    /// Parses annotation values
    fn parse_value(&self, value_str: &str) -> AnnotationValue {
        // Simple value parsing for now
        if value_str == "true" {
            AnnotationValue::Bool(true)
        } else if value_str == "false" {
            AnnotationValue::Bool(false)
        } else if let Ok(int) = value_str.parse::<i64>() {
            AnnotationValue::Int(int)
        } else if let Ok(float) = value_str.parse::<f64>() {
            AnnotationValue::Float(float)
        } else if value_str.starts_with('"') && value_str.ends_with('"') {
            let inner = &value_str[1..value_str.len()-1];
            AnnotationValue::String(inner.to_string())
        } else {
            AnnotationValue::String(value_str.to_string())
        }
    }
}

/// TypeScript annotation parser
/// 
/// This struct parses TypeScript-specific annotations like `// @format:xxx`.
pub struct TypeScriptAnnotationParser;

impl AnnotationParser for TypeScriptAnnotationParser {
    fn parse(&self, source: &str) -> Vec<FormatAnnotation> {
        use core::ops::Range;
        let mut annotations = Vec::new();
        
        #[cfg(feature = "regex")]
        {
            // Regular expression to match TypeScript annotations like `// @format:xxx` or `// @format(xxx)`
            let regex = regex::Regex::new(r"//\s*@format(?::([a-zA-Z_][a-zA-Z0-9_]*))?\s*(?:\(([^\)]*)\))?\s*(?:$|\n)").unwrap();
            
            for capture in regex.captures_iter(source) {
                let full_match = capture.get(0).unwrap();
                let span = Range {
                    start: full_match.start(),
                    end: full_match.end(),
                };
                
                let name = capture.get(1).map(|m| m.as_str().to_string()).unwrap_or_else(|| "default".to_string());
                let params_str = capture.get(2).map(|m| m.as_str()).unwrap_or("");
                
                let params = self.parse_params(params_str);
                
                annotations.push(FormatAnnotation {
                    name,
                    params,
                    span,
                });
            }
        }
        
        annotations
    }
}

impl TypeScriptAnnotationParser {
    /// Parses annotation parameters
    fn parse_params(&self, params_str: &str) -> Vec<AnnotationParam> {
        let mut params = Vec::new();
        
        // Simple parameter parsing for now
        let param_pairs = params_str.split(',').map(|s| s.trim()).filter(|s| !s.is_empty());
        
        for pair in param_pairs {
            if let Some((name, value_str)) = pair.split_once('=') {
                let name = name.trim().to_string();
                let value = self.parse_value(value_str.trim());
                params.push(AnnotationParam {
                    name,
                    value,
                });
            }
        }
        
        params
    }
    
    /// Parses annotation values
    fn parse_value(&self, value_str: &str) -> AnnotationValue {
        // Simple value parsing for now
        if value_str == "true" {
            AnnotationValue::Bool(true)
        } else if value_str == "false" {
            AnnotationValue::Bool(false)
        } else if let Ok(int) = value_str.parse::<i64>() {
            AnnotationValue::Int(int)
        } else if let Ok(float) = value_str.parse::<f64>() {
            AnnotationValue::Float(float)
        } else if value_str.starts_with('"') && value_str.ends_with('"') {
            let inner = &value_str[1..value_str.len()-1];
            AnnotationValue::String(inner.to_string())
        } else {
            AnnotationValue::String(value_str.to_string())
        }
    }
}

/// Annotation processor
/// 
/// This struct processes annotations and applies them to the formatting state.
pub struct AnnotationProcessor {
    /// The annotation parser to use
    parser: Box<dyn AnnotationParser>,
}

impl AnnotationProcessor {
    /// Creates a new annotation processor
    /// 
    /// # Parameters
    /// - `parser`: The annotation parser to use
    pub fn new(parser: Box<dyn AnnotationParser>) -> Self {
        Self { parser }
    }

    /// Processes annotations from the source code
    /// 
    /// # Parameters
    /// - `source`: The source code string
    /// 
    /// # Returns
    /// A vector of `FormatAnnotation` objects
    pub fn process(&self, source: &str) -> Vec<FormatAnnotation> {
        self.parser.parse(source)
    }
}

    /// Converts an AnnotationValue to a serde_json::Value
    #[cfg(feature = "serde")]
    fn value_to_json(&self, value: &AnnotationValue) -> serde_json::Value {
        match value {
            AnnotationValue::Bool(v) => serde_json::Value::Bool(*v),
            AnnotationValue::Int(v) => serde_json::Value::Number(serde_json::Number::from(*v)),
            AnnotationValue::Float(v) => {
                if v.is_finite() {
                    serde_json::Value::Number(serde_json::Number::from_f64(*v).unwrap_or(serde_json::Number::from(0)))
                } else {
                    serde_json::Value::Null
                }
            }
            AnnotationValue::String(v) => serde_json::Value::String(v.clone()),
            AnnotationValue::List(values) => {
                serde_json::Value::Array(values.iter().map(|v| self.value_to_json(v)).collect())
            }
            AnnotationValue::Map(pairs) => {
                let json_map: serde_json::Map<String, serde_json::Value> = pairs.iter()
                    .map(|(k, v)| (k.clone(), self.value_to_json(v)))
                    .collect();
                serde_json::Value::Object(json_map)
            }
        }
    }
    
    /// Converts an AnnotationValue to a serde_json::Value (dummy implementation for no serde)
    #[cfg(not(feature = "serde"))]
    fn value_to_json(&self, _value: &AnnotationValue) -> () {
        ()
    }
}
