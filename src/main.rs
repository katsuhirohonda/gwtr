use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;

/// A simple Git worktree manager
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new worktree
    Add {
        /// Name of the worktree
        name: String,
    },
    /// List all worktrees
    List,
    /// Remove a worktree
    Remove {
        /// Name of the worktree to remove
        name: String,
    },
    /// Switch to a worktree
    Switch {
        /// Name of the worktree to switch to
        name: String,
    },
    /// Show status of all worktrees
    Status,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Commands::Add { name }) => {
            // Validate git repository
            let current_dir = env::current_dir()?;
            let repo = gwtr::ensure_git_repository(&current_dir)?;
            
            // Create worktree
            gwtr::create_worktree(&repo, name)?;
        }
        Some(Commands::List) => {
            // Validate git repository
            let current_dir = env::current_dir()?;
            let repo = gwtr::ensure_git_repository(&current_dir)?;
            
            // List worktrees
            gwtr::list_worktrees(&repo)?;
        }
        Some(Commands::Remove { name }) => {
            // Validate git repository
            let current_dir = env::current_dir()?;
            let repo = gwtr::ensure_git_repository(&current_dir)?;
            
            // Remove worktree
            gwtr::remove_worktree(&repo, name)?;
        }
        Some(Commands::Switch { name }) => {
            // Validate git repository
            let current_dir = env::current_dir()?;
            let repo = gwtr::ensure_git_repository(&current_dir)?;
            
            // Switch to worktree
            gwtr::switch_to_worktree(&repo, name)?;
        }
        Some(Commands::Status) => {
            // Validate git repository
            let current_dir = env::current_dir()?;
            let repo = gwtr::ensure_git_repository(&current_dir)?;
            
            // Show worktrees status
            gwtr::show_worktrees_status(&repo)?;
        }
        None => {
            // This shouldn't happen with arg_required_else_help
        }
    }
    
    Ok(())
}