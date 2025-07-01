# Rview Implementation Plan

## Overview
Simple CLI tool for AI-assisted code review that aggregates modified files into a single review file.

## Core Features

### CLI Interface
```bash
rview [OPTIONS]
  -o, --output <file>           Output file (default: docs/code-review.txt)
  -x, --exclude <patterns>      Exclude files/patterns (package.json,*.lock,dist/)
  -h, --help                    Show help
  -V, --version                 Show version
```

### Functionality
1. **Git Integration**: Get modified files from `git status --porcelain`
2. **File Aggregation**: Read and combine file contents
3. **Pattern Exclusion**: Skip files matching exclude patterns
4. **Clean Output**: Simple separator format for AI consumption


### Output Format
```txt
========== src/auth.ts ==========
[file content]

========== src/utils.js ==========
[file content]
```

## Implementation Steps
1. ✅ Set up Cargo.toml with dependencies (clap, glob)  
2. ✅ Implement CLI argument parsing
3. ✅ Create git status integration
4. ✅ Add file reading and aggregation logic
5. ✅ Implement exclude pattern matching
6. ✅ Add modular configuration system
7. Handle binary files and errors gracefully
8. ✅ Test with sample repository

## Dependencies
- `clap` - CLI argument parsing
- `glob` - Pattern matching for excludes
- `std::process` - Git command execution

## Future Considerations
- Configuration file support (`$XDG_CONFIG_HOME/rview.toml` with serde + toml)
- Configurable separator character and length
- Default exclude patterns in config
- Enhanced error handling for edge cases