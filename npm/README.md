# gwtr - Git Worktree Manager

[![npm version](https://img.shields.io/npm/v/gwtr.svg)](https://www.npmjs.com/package/gwtr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple Git worktree manager that creates worktrees in a consistent location alongside your main repository.

## Features

- **Consistent Naming**: Creates worktrees with the pattern `{repository_name}_{worktree_name}`
- **Automatic Branch Creation**: Creates a new branch when adding a worktree
- **Simple Commands**: Easy-to-remember commands for common worktree operations
- **Colored Output**: Clear, colored terminal output for better readability
- **Git Integration**: Works seamlessly with existing Git repositories

## How It Works

When you run `gwtr add feature-x` in a repository named `myproject`, it creates a new worktree at `../myproject_feature-x`. This keeps all related worktrees organized at the same directory level as your main repository.

## Installation

```bash
npm install -g gwtr
```

Or using other package managers:

```bash
# Using yarn
yarn global add gwtr

# Using pnpm
pnpm add -g gwtr
```

## Usage

```bash
# Create a new worktree
gwtr add feature-x

# List all worktrees
gwtr list

# Remove a worktree
gwtr remove feature-x
```

### Examples

```bash
# In a repository called "myapp"
$ gwtr add new-feature
Created worktree 'new-feature' at "../myapp_new-feature"

$ gwtr list
Worktrees:
  /Users/you/dev/myapp (main) [main]
  /Users/you/dev/myapp_new-feature [new-feature]

$ gwtr remove new-feature
Removed worktree 'new-feature' at "../myapp_new-feature"
```

## Prerequisites

- Git 2.5.0 or later (for worktree support)
- Node.js 14.0.0 or later

## Supported Platforms

- macOS (Intel & Apple Silicon)
- Linux (x64 & ARM64)
- Windows (x64)

## Source Code

This is the npm distribution of gwtr. The source code is written in Rust and available at:
https://github.com/katsuhirohonda/gwtr

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to:
- Update tests as appropriate
- Follow the existing code style
- Run `cargo fmt` and `cargo clippy` before submitting

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by Git's worktree feature
- Built with Rust and the excellent [clap](https://github.com/clap-rs/clap) CLI framework