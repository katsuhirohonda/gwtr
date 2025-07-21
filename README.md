# gwtr - Git Worktree Manager

A simple Git worktree manager that creates worktrees in a consistent location.

## Installation

```bash
cargo install gwtr
```

## Usage

```bash
# Create a new worktree
gwtr add feature-x

# List worktrees
gwtr list

# Remove a worktree
gwtr remove feature-x
```

## Development

This project follows Test-Driven Development (TDD) practices.

```bash
# Run tests
cargo test

# Build
cargo build

# Run
cargo run -- --help
```