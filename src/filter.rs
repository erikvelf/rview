use crate::config::Config;
use glob::Pattern;
use std::path::Path;

/// Handles file filtering based on exclude patterns
pub struct FileFilter {
    patterns: Vec<Pattern>,
}

impl FileFilter {
    /// Creates a new FileFilter from configuration
    pub fn from_config(config: &Config) -> Self {
        let mut patterns = Vec::new();

        // Always exclude the output file to prevent infinite loops
        if let Ok(pattern) = Pattern::new(&config.output_file) {
            patterns.push(pattern);
        }

        // Add user-provided exclude patterns
        for pattern_str in &config.exclude_patterns {
            if let Ok(pattern) = Pattern::new(pattern_str) {
                patterns.push(pattern);
            }
        }

        Self { patterns }
    }

    /// Returns true if the file should be excluded
    pub fn should_exclude(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();

        for pattern in &self.patterns {
            if pattern.matches(&path_str) {
                return true;
            }
        }

        false
    }
}
