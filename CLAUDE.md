# CLAUDE.md

**CRITICAL**
Always document the code well with doc comments and explain what it does.

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`rview` is a simple CLI tool designed for AI-assisted code review. It aggregates modified files from git into a single review file for consumption by AI reviewers. The tool targets solo developers using AI coding assistants.

## Core Architecture

The current implementation (`src/main.rs`) provides basic functionality:

- Uses `git status --porcelain` to detect modified/untracked files
- Reads file contents and aggregates them into `docs/code_review.txt`
- Uses simple separator format with file headers

**Planned Architecture** (from `add.md`):

- CLI interface with `clap` for argument parsing
- Configuration system using TOML files
- Pattern-based file exclusion system
- Configurable output formatting

## Development Commands

```bash
# Build the project
cargo build

# Run the tool
cargo run

# Build optimized release
cargo build --release

# Run tests (when added)
cargo test
```

## Key Design Principles

- **Simplicity**: Minimal feature set focused on AI code review
- **AI-First**: Output format optimized for AI consumption, not human readability
- **Solo Developer Workflow**: Designed for individual developers, not teams
- **Git Integration**: Works with standard git workflow to detect changed files

## Current Limitations

The existing code lacks the planned features:

- No CLI argument parsing (hardcoded output path)
- No file exclusion capabilities
- No configuration system
- No binary file detection
- No error handling for unreadable files

## Implementation Status

See `add.md` for detailed implementation plan. The current code is a basic prototype that needs to be enhanced with proper CLI interface and configuration system.

