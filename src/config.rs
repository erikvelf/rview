/// Configuration for rview tool
#[derive(Debug, Clone)]
pub struct Config {
    /// Output file path
    pub output_file: String,
    /// Patterns to exclude from review
    pub exclude_patterns: Vec<String>,
    /// Separator character for file headers
    pub separator_char: String,
    /// Length of separator line
    pub separator_length: usize,
}

impl Config {
    /// Creates a new Config with default values
    pub fn new() -> Self {
        Self {
            output_file: "docs/code-review.txt".to_string(),
            exclude_patterns: Vec::new(),
            separator_char: "=".to_string(),
            separator_length: 10,
        }
    }
    
    /// Updates config with CLI arguments
    pub fn with_cli_args(mut self, output: String, exclude: Vec<String>) -> Self {
        self.output_file = output;
        self.exclude_patterns = exclude;
        self
    }
    
    /// Gets the separator line for file headers
    pub fn get_separator(&self) -> String {
        self.separator_char.repeat(self.separator_length)
    }
}