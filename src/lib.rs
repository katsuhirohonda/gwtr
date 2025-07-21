use anyhow::{Context, Result, bail};
use git2::Repository;
use std::path::{Path, PathBuf};
use colored::*;

/// Check if the current directory is inside a git repository
pub fn ensure_git_repository(path: &Path) -> Result<Repository> {
    Repository::discover(path)
        .context("Not in a git repository. Please run this command inside a git repository.")
}

/// Get the repository name from the current git repository
pub fn get_repository_name(repo: &Repository) -> Result<String> {
    let workdir = repo.workdir()
        .context("Failed to get repository working directory")?;
    
    let repo_name = workdir
        .file_name()
        .context("Failed to get repository directory name")?
        .to_str()
        .context("Repository name contains invalid UTF-8")?;
    
    Ok(repo_name.to_string())
}

/// Create a new worktree with the specified name
pub fn create_worktree(repo: &Repository, worktree_name: &str) -> Result<PathBuf> {
    // Get repository name and parent directory
    let repo_name = get_repository_name(repo)?;
    let workdir = repo.workdir()
        .context("Failed to get repository working directory")?;
    let parent_dir = workdir.parent()
        .context("Failed to get parent directory of repository")?;
    
    // Construct worktree path: ../repo-name_worktree-name
    let worktree_path = parent_dir.join(format!("{}_{}", repo_name, worktree_name));
    
    // Check if worktree already exists
    if worktree_path.exists() {
        bail!("Worktree '{}' already exists at {:?}", worktree_name, worktree_path);
    }
    
    // Use git command to create worktree
    // This is more reliable than using libgit2's worktree API
    use std::process::Command;
    
    let output = Command::new("git")
        .args(&["worktree", "add", "-b", worktree_name, worktree_path.to_str().unwrap()])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree command")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Check if branch already exists and retry without -b flag
        if stderr.contains("already exists") {
            let output = Command::new("git")
                .args(&["worktree", "add", worktree_path.to_str().unwrap(), worktree_name])
                .current_dir(workdir)
                .output()
                .context("Failed to execute git worktree command")?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Failed to create worktree: {}", stderr);
            }
        } else {
            bail!("Failed to create worktree: {}", stderr);
        }
    }
    
    println!("Created worktree '{}' at {:?}", worktree_name, worktree_path);
    
    Ok(worktree_path)
}

/// List all worktrees for the current repository
pub fn list_worktrees(repo: &Repository) -> Result<()> {
    use std::process::Command;
    
    let workdir = repo.workdir()
        .context("Failed to get repository working directory")?;
    
    // Use git worktree list command
    let output = Command::new("git")
        .args(&["worktree", "list", "--porcelain"])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree list command")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to list worktrees: {}", stderr);
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    
    if lines.is_empty() {
        println!("No worktrees found");
        return Ok(());
    }
    
    // Parse the porcelain output
    let mut worktrees = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("worktree ") {
            let path = lines[i].strip_prefix("worktree ").unwrap_or("");
            let mut branch = "detached";
            let mut is_bare = false;
            
            // Look for additional info
            if i + 1 < lines.len() && lines[i + 1].starts_with("HEAD ") {
                i += 1;
            }
            if i + 1 < lines.len() && lines[i + 1].starts_with("branch ") {
                branch = lines[i + 1].strip_prefix("branch refs/heads/").unwrap_or("unknown");
                i += 1;
            }
            if i + 1 < lines.len() && lines[i + 1] == "bare" {
                is_bare = true;
                i += 1;
            }
            
            worktrees.push((path.to_string(), branch.to_string(), is_bare));
        }
        i += 1;
    }
    
    // Display worktrees
    println!("{}", "Worktrees:".bold());
    let main_path = workdir.to_string_lossy().trim_end_matches('/').to_string();
    
    for (path, branch, is_bare) in worktrees {
        let normalized_path = path.trim_end_matches('/');
        let is_main = normalized_path == main_path;
        
        let display_path = if is_main {
            format!("{} (main)", path).green()
        } else {
            path.yellow()
        };
        
        if is_bare {
            println!("  {} [bare]", display_path);
        } else {
            println!("  {} [{}]", display_path, branch.cyan());
        }
    }
    
    Ok(())
}

/// Remove a worktree
pub fn remove_worktree(repo: &Repository, worktree_name: &str) -> Result<()> {
    use std::process::Command;
    
    let workdir = repo.workdir()
        .context("Failed to get repository working directory")?;
    let repo_name = get_repository_name(repo)?;
    let parent_dir = workdir.parent()
        .context("Failed to get parent directory of repository")?;
    
    // Construct expected worktree path
    let worktree_path = parent_dir.join(format!("{}_{}", repo_name, worktree_name));
    
    // Check if worktree exists
    if !worktree_path.exists() {
        bail!("Worktree '{}' not found at {:?}", worktree_name, worktree_path);
    }
    
    // Use git worktree remove command
    let output = Command::new("git")
        .args(&["worktree", "remove", worktree_path.to_str().unwrap()])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree remove command")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Try with --force if it contains uncommitted changes
        if stderr.contains("contains modified or untracked files") {
            println!("Worktree contains uncommitted changes, removing with --force");
            
            let output = Command::new("git")
                .args(&["worktree", "remove", "--force", worktree_path.to_str().unwrap()])
                .current_dir(workdir)
                .output()
                .context("Failed to execute git worktree remove command")?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Failed to remove worktree: {}", stderr);
            }
        } else {
            bail!("Failed to remove worktree: {}", stderr);
        }
    }
    
    println!("Removed worktree '{}' at {:?}", worktree_name, worktree_path);
    
    Ok(())
}

/// Switch to a worktree
pub fn switch_to_worktree(repo: &Repository, worktree_name: &str) -> Result<()> {
    let workdir = repo.workdir()
        .context("Failed to get repository working directory")?;
    let repo_name = get_repository_name(repo)?;
    let parent_dir = workdir.parent()
        .context("Failed to get parent directory of repository")?;
    
    // Construct expected worktree path
    let worktree_path = parent_dir.join(format!("{}_{}", repo_name, worktree_name));
    
    // Check if worktree exists
    if !worktree_path.exists() {
        bail!("Worktree '{}' not found", worktree_name);
    }
    
    // Print the cd command for the user to run
    println!("cd {}", worktree_path.display());
    
    Ok(())
}