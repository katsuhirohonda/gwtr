# gwtr - Git Worktree Manager

[![Crates.io](https://img.shields.io/crates/v/gwtr.svg)](https://crates.io/crates/gwtr)
[![Documentation](https://docs.rs/gwtr/badge.svg)](https://docs.rs/gwtr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple Git worktree manager that creates worktrees in a consistent location alongside your main repository.

## Features

- **Consistent Naming**: Creates worktrees with the pattern `{repository_name}_{worktree_name}`
- **Automatic Branch Creation**: Creates a new branch when adding a worktree
- **Simple Commands**: Easy-to-remember commands for common worktree operations
- **Colored Output**: Clear, colored terminal output for better readability
- **Git Integration**: Works seamlessly with existing Git repositories
- **Status Overview**: View all worktrees and their states at a glance
- **Batch Updates**: Pull latest changes from origin/main to all worktrees
- **Smart Cleanup**: Automatically remove merged worktrees to keep workspace tidy

## How It Works

When you run `gwtr add feature-x` in a repository named `myproject`, it creates a new worktree at `../myproject_feature-x`. This keeps all related worktrees organized at the same directory level as your main repository.

## Installation

### From Crates.io

```bash
cargo install gwtr
```

### From npm

```bash
npm install -g gwtr
```

### From Source

```bash
git clone https://github.com/katsuhirohonda/gwtr.git
cd gwtr
cargo install --path .
```

## Usage

```bash
# Create a new worktree
gwtr add feature-x

# List all worktrees
gwtr list

# Show status of all worktrees
gwtr status

# Pull latest changes from origin/main
gwtr pull --all           # All worktrees
gwtr pull feature-x       # Specific worktree
gwtr pull                 # Current worktree

# Remove merged worktrees
gwtr prune               # Interactive mode
gwtr prune --dry-run     # Preview what would be removed
gwtr prune --force       # Skip confirmation

# Remove a specific worktree
gwtr remove feature-x
```

### Examples

```bash
# In a repository called "myapp"
$ gwtr add new-feature
Created worktree 'new-feature' at "../myapp_new-feature"
cd '/Users/you/dev/myapp_new-feature'

$ gwtr list
Worktrees:
  /Users/you/dev/myapp (main) [main]
  /Users/you/dev/myapp_new-feature [new-feature]

$ gwtr status
Worktrees:
  /Users/you/dev/myapp (main) [main] - clean
  /Users/you/dev/myapp_new-feature [new-feature] - 2 uncommitted changes

$ gwtr pull --all
Pulling all worktrees from origin/main...
  main [main]: Already up to date
  myapp_new-feature [new-feature]: Updated

$ gwtr prune
Found 1 merged worktree to prune:
  old-feature [old-feature] at /Users/you/dev/myapp_old-feature

Prune these worktrees? [y/N] y
Pruning old-feature... done

Pruned 1 worktree

$ gwtr remove new-feature
Removed worktree 'new-feature' at "../myapp_new-feature"
```

## Prerequisites

- Git 2.5.0 or later (for worktree support)
- Rust 1.80.0 or later (for building from source)

## Development

This project follows Test-Driven Development (TDD) practices.

```bash
# Run tests
cargo test

# Build
cargo build

# Run with debug output
cargo run -- --help

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to:
- Update tests as appropriate
- Follow the existing code style
- Run `cargo fmt` and `cargo clippy` before submitting

## Release Process

This project uses GitHub Actions for automated releases:
- **crates.io**: Automatically published when a GitHub release is created
- **npm**: Automatically published with pre-compiled binaries for all platforms

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by Git's worktree feature
- Built with Rust and the excellent [clap](https://github.com/clap-rs/clap) CLI framework