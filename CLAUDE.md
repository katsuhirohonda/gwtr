# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

gwtr (Git Worktree Manager) is a CLI tool written in Rust that simplifies Git worktree management by creating worktrees in a consistent location alongside the main repository. The project follows Test-Driven Development (TDD) practices.

## Common Development Commands

### Building and Testing
```bash
# Build the project
cargo build

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

### Running the Application
```bash
# Run with debug output
cargo run -- --help

# Run specific commands
cargo run -- add feature-x
cargo run -- list
cargo run -- status
```

## Architecture

The codebase is organized as follows:

- **src/lib.rs**: Core library containing all worktree management functions
  - `ensure_git_repository()`: Validates current directory is a git repo
  - `create_worktree()`: Creates new worktrees with branch
  - `list_worktrees()`: Lists all worktrees
  - `remove_worktree()`: Removes worktrees (with --force for uncommitted changes)
  - `show_worktrees_status()`: Shows status of all worktrees
  - `pull_*_worktree()`: Pull functions for updating worktrees
  - `prune_merged_worktrees()`: Removes merged worktrees

- **src/main.rs**: CLI entry point using clap
  - Defines command structure (add, list, remove, status, pull, prune)
  - Handles argument parsing and delegates to lib functions

- **tests/**: Integration tests organized by command
  - Uses `TestHelper` from `tests/common/mod.rs` for test setup
  - Creates temporary git repositories for testing

## Key Implementation Details

1. **Worktree Naming Convention**: Worktrees are created as `{repository_name}_{worktree_name}` in the parent directory of the main repository.

2. **Git Command Usage**: The project uses `std::process::Command` to execute git commands rather than libgit2's worktree API for better reliability.

3. **Error Handling**: Uses anyhow for error propagation with context messages.

4. **Output Formatting**: Uses the colored crate for terminal output formatting.

## Testing Approach

Tests use the `TestHelper` struct which:
- Creates temporary directories with git repositories
- Provides a `run_gwtr()` method to execute the binary
- Automatically cleans up on drop

Run tests with `cargo test` to ensure changes work correctly.

## Release Process

The project is published to both crates.io and npm:
- GitHub Actions automatically publish when a release is created
- npm packages include pre-compiled binaries for all platforms
- Version is managed in Cargo.toml and npm/package.json files

When releasing a new version:
1. Update version in `Cargo.toml`
2. Update version in `npm/package.json`
3. Update version in all platform-specific package.json files:
   - `npm/platforms/darwin-x64/package.json`
   - `npm/platforms/darwin-arm64/package.json`
   - `npm/platforms/linux-x64/package.json`
   - `npm/platforms/linux-arm64/package.json`
   - `npm/platforms/win32-x64/package.json`
4. Update the version references in optionalDependencies in `npm/package.json`
5. Create a git tag and GitHub release to trigger automatic publishing