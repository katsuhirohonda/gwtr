pub mod audit;
pub use audit::run_audit;

use anyhow::{Context, Result, bail};
use colored::*;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Default)]
pub struct WorktreeNote {
    pub note: Option<String>,
    pub ref_: Option<String>,
}

fn gwtr_dir(repo: &Repository) -> Result<PathBuf> {
    let git_dir = repo.path();
    Ok(git_dir.join("gwtr"))
}

fn note_path(repo: &Repository, worktree_name: &str) -> Result<PathBuf> {
    Ok(gwtr_dir(repo)?.join(format!("{}.json", worktree_name)))
}

fn load_note(repo: &Repository, worktree_name: &str) -> Result<WorktreeNote> {
    let path = note_path(repo, worktree_name)?;
    if !path.exists() {
        return Ok(WorktreeNote::default());
    }
    let content = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&content)?)
}

fn save_note(repo: &Repository, worktree_name: &str, note: &WorktreeNote) -> Result<()> {
    let dir = gwtr_dir(repo)?;
    std::fs::create_dir_all(&dir)?;
    let path = note_path(repo, worktree_name)?;
    std::fs::write(path, serde_json::to_string_pretty(note)?)?;
    Ok(())
}

fn delete_note(repo: &Repository, worktree_name: &str) -> Result<()> {
    let path = note_path(repo, worktree_name)?;
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

/// Check if the current directory is inside a git repository
pub fn ensure_git_repository(path: &Path) -> Result<Repository> {
    Repository::discover(path)
        .context("Not in a git repository. Please run this command inside a git repository.")
}

/// Get the repository name from the current git repository
pub fn get_repository_name(repo: &Repository) -> Result<String> {
    let workdir = repo
        .workdir()
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
    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;
    let parent_dir = workdir
        .parent()
        .context("Failed to get parent directory of repository")?;

    // Construct worktree path: ../repo-name_worktree-name
    let worktree_path = parent_dir.join(format!("{}_{}", repo_name, worktree_name));

    // Check if worktree already exists
    if worktree_path.exists() {
        bail!(
            "Worktree '{}' already exists at {:?}",
            worktree_name,
            worktree_path
        );
    }

    // Use git command to create worktree
    // This is more reliable than using libgit2's worktree API
    use std::process::Command;

    let output = Command::new("git")
        .args([
            "worktree",
            "add",
            "-b",
            worktree_name,
            worktree_path.to_str().unwrap(),
        ])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Check if branch already exists and retry without -b flag
        if stderr.contains("already exists") {
            let output = Command::new("git")
                .args([
                    "worktree",
                    "add",
                    worktree_path.to_str().unwrap(),
                    worktree_name,
                ])
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

    println!(
        "Created worktree '{}' at {:?}",
        worktree_name, worktree_path
    );
    println!("cd '{}'", worktree_path.display());

    Ok(worktree_path)
}

/// List all worktrees for the current repository
pub fn list_worktrees(repo: &Repository) -> Result<()> {
    use std::process::Command;

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;

    // Use git worktree list command
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
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
                branch = lines[i + 1]
                    .strip_prefix("branch refs/heads/")
                    .unwrap_or("unknown");
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

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;
    let repo_name = get_repository_name(repo)?;
    let parent_dir = workdir
        .parent()
        .context("Failed to get parent directory of repository")?;

    // Construct expected worktree path
    let worktree_path = parent_dir.join(format!("{}_{}", repo_name, worktree_name));

    // Check if worktree exists
    if !worktree_path.exists() {
        bail!(
            "Worktree '{}' not found at {:?}",
            worktree_name,
            worktree_path
        );
    }

    // Use git worktree remove command
    let output = Command::new("git")
        .args(["worktree", "remove", worktree_path.to_str().unwrap()])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree remove command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Try with --force if it contains uncommitted changes
        if stderr.contains("contains modified or untracked files") {
            println!("Worktree contains uncommitted changes, removing with --force");

            let output = Command::new("git")
                .args([
                    "worktree",
                    "remove",
                    "--force",
                    worktree_path.to_str().unwrap(),
                ])
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

    delete_note(repo, worktree_name)?;
    println!(
        "Removed worktree '{}' at {:?}",
        worktree_name, worktree_path
    );

    Ok(())
}

/// Show status of all worktrees
pub fn show_worktrees_status(repo: &Repository) -> Result<()> {
    use std::process::Command;

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;

    // Get list of worktrees
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree list command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to list worktrees: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // Parse worktrees
    let mut worktrees = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("worktree ") {
            let path = lines[i].strip_prefix("worktree ").unwrap_or("");
            let mut branch = "detached";

            // Look for branch info
            if i + 1 < lines.len() && lines[i + 1].starts_with("HEAD ") {
                i += 1;
            }
            if i + 1 < lines.len() && lines[i + 1].starts_with("branch ") {
                branch = lines[i + 1]
                    .strip_prefix("branch refs/heads/")
                    .unwrap_or("unknown");
                i += 1;
            }

            worktrees.push((path.to_string(), branch.to_string()));
        }
        i += 1;
    }

    // Display worktrees with status
    println!("{}", "Worktrees:".bold());
    let main_path = workdir.to_string_lossy().trim_end_matches('/').to_string();

    for (path, branch) in worktrees {
        let normalized_path = path.trim_end_matches('/');
        let is_main = normalized_path == main_path;

        // Check for uncommitted changes
        let status_output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(&path)
            .output();

        let status_msg = if let Ok(output) = status_output {
            if output.status.success() {
                let status_str = String::from_utf8_lossy(&output.stdout);
                if status_str.trim().is_empty() {
                    "clean".green().to_string()
                } else {
                    let change_count = status_str.lines().count();
                    format!("{} uncommitted changes", change_count)
                        .yellow()
                        .to_string()
                }
            } else {
                "unknown".red().to_string()
            }
        } else {
            "error".red().to_string()
        };

        let display_path = if is_main {
            format!("{} (main)", path).green()
        } else {
            path.yellow()
        };

        println!("  {} [{}] - {}", display_path, branch.cyan(), status_msg);
    }

    Ok(())
}

/// Pull changes in all worktrees
pub fn pull_all_worktrees(repo: &Repository) -> Result<()> {
    use std::process::Command;

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;

    println!("Pulling all worktrees from origin/main...");

    // Get list of worktrees
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree list command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to list worktrees: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // Parse worktree paths
    let mut worktree_paths = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("worktree ") {
            let path = lines[i].strip_prefix("worktree ").unwrap_or("");
            let mut branch = "detached";

            // Look for branch info
            if i + 1 < lines.len() && lines[i + 1].starts_with("HEAD ") {
                i += 1;
            }
            if i + 1 < lines.len() && lines[i + 1].starts_with("branch ") {
                branch = lines[i + 1]
                    .strip_prefix("branch refs/heads/")
                    .unwrap_or("unknown");
                i += 1;
            }

            worktree_paths.push((path.to_string(), branch.to_string()));
        }
        i += 1;
    }

    // Pull origin/main in each worktree
    let main_path = workdir.to_string_lossy().trim_end_matches('/').to_string();

    for (path, branch) in worktree_paths {
        let normalized_path = path.trim_end_matches('/');
        let is_main = normalized_path == main_path;

        let worktree_name = if is_main {
            "main".to_string()
        } else {
            // Extract worktree name from path
            std::path::Path::new(&path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&branch)
                .to_string()
        };

        print!("  {} [{}]: ", worktree_name.yellow(), branch.cyan());

        // Pull from origin/main
        let pull_output = Command::new("git")
            .args(["pull", "origin", "main"])
            .current_dir(&path)
            .output();

        match pull_output {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if stdout.contains("Already up to date")
                        || stdout.contains("Already up-to-date")
                    {
                        println!("{}", "Already up to date".green());
                    } else {
                        println!("{}", "Updated".green());
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    if stderr.contains("Could not find remote")
                        || stderr.contains("fatal: 'origin' does not appear to be a git repository")
                    {
                        println!("{}: No remote configured", "Skipped".yellow());
                    } else {
                        println!("{}: {}", "Failed".red(), stderr.trim());
                    }
                }
            }
            Err(e) => {
                println!("{}: {}", "Error".red(), e);
            }
        }
    }

    Ok(())
}

/// Pull changes in a specific worktree
pub fn pull_worktree(repo: &Repository, worktree_name: &str) -> Result<()> {
    use std::process::Command;

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;
    let repo_name = get_repository_name(repo)?;
    let parent_dir = workdir
        .parent()
        .context("Failed to get parent directory of repository")?;

    // Check if pulling main worktree
    let worktree_path = if worktree_name == "main" {
        workdir.to_path_buf()
    } else {
        // Construct expected worktree path
        let path = parent_dir.join(format!("{}_{}", repo_name, worktree_name));

        // Check if worktree exists
        if !path.exists() {
            bail!("Worktree '{}' not found", worktree_name);
        }
        path
    };

    println!("Pulling worktree '{}' from origin/main...", worktree_name);

    // Execute git pull from origin/main
    let output = Command::new("git")
        .args(["pull", "origin", "main"])
        .current_dir(&worktree_path)
        .output()
        .context("Failed to execute git pull command")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Already up to date") || stdout.contains("Already up-to-date") {
            println!(
                "{}: {}",
                worktree_name.yellow(),
                "Already up to date".green()
            );
        } else {
            println!("{}: {}", worktree_name.yellow(), "Updated".green());
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("Could not find remote")
            || stderr.contains("fatal: 'origin' does not appear to be a git repository")
        {
            bail!("No remote 'origin' configured");
        } else {
            bail!("Failed to pull worktree '{}': {}", worktree_name, stderr);
        }
    }

    Ok(())
}

/// Pull changes in the current worktree
pub fn pull_current_worktree(repo: &Repository) -> Result<()> {
    use std::process::Command;

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;

    println!("Pulling current worktree from origin/main...");

    // Execute git pull from origin/main
    let output = Command::new("git")
        .args(["pull", "origin", "main"])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git pull command")?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Already up to date") || stdout.contains("Already up-to-date") {
            println!("{}", "Already up to date".green());
        } else {
            println!("{}", "Updated".green());
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("Could not find remote")
            || stderr.contains("fatal: 'origin' does not appear to be a git repository")
        {
            bail!("No remote 'origin' configured");
        } else {
            bail!("Failed to pull: {}", stderr);
        }
    }

    Ok(())
}

/// Prune merged worktrees
pub fn prune_merged_worktrees(repo: &Repository, dry_run: bool, force: bool) -> Result<()> {
    use std::io::{self, Write};
    use std::process::Command;

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;
    let repo_name = get_repository_name(repo)?;

    // Get list of worktrees
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree list command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to list worktrees: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // Parse worktrees and check which branches are merged
    let main_path = workdir.to_string_lossy().trim_end_matches('/').to_string();
    let mut merged_worktrees = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].starts_with("worktree ") {
            let path = lines[i].strip_prefix("worktree ").unwrap_or("");
            let normalized_path = path.trim_end_matches('/');

            // Skip main worktree
            if normalized_path == main_path {
                i += 1;
                continue;
            }

            // Look for branch info
            if i + 1 < lines.len() && lines[i + 1].starts_with("HEAD ") {
                i += 1;
            }
            if i + 1 < lines.len() && lines[i + 1].starts_with("branch ") {
                let branch = lines[i + 1]
                    .strip_prefix("branch refs/heads/")
                    .unwrap_or("unknown");
                i += 1;

                // Check if branch is merged to main
                let merged_output = Command::new("git")
                    .args(["branch", "--merged", "main"])
                    .current_dir(workdir)
                    .output();

                if let Ok(output) = merged_output
                    && output.status.success()
                {
                    let merged_branches = String::from_utf8_lossy(&output.stdout);
                    if merged_branches
                        .lines()
                        .any(|line| line.trim() == branch || line.trim() == format!("* {}", branch))
                    {
                        // Extract worktree name from path
                        let worktree_name = std::path::Path::new(path)
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or(branch)
                            .to_string();

                        // Remove repo name prefix if present
                        let worktree_name = if worktree_name.starts_with(&format!("{}_", repo_name))
                        {
                            worktree_name
                                .strip_prefix(&format!("{}_", repo_name))
                                .unwrap_or(&worktree_name)
                                .to_string()
                        } else {
                            worktree_name
                        };

                        merged_worktrees.push((
                            path.to_string(),
                            branch.to_string(),
                            worktree_name,
                        ));
                    }
                }
            }
        }
        i += 1;
    }

    if merged_worktrees.is_empty() {
        println!("No worktrees to prune");
        return Ok(());
    }

    if dry_run {
        println!(
            "Would prune {} merged worktree{}:",
            merged_worktrees.len(),
            if merged_worktrees.len() == 1 { "" } else { "s" }
        );
        for (path, branch, name) in &merged_worktrees {
            println!("  {} [{}] at {}", name.yellow(), branch.cyan(), path);
        }
        return Ok(());
    }

    // Show worktrees to be pruned
    println!(
        "Found {} merged worktree{} to prune:",
        merged_worktrees.len(),
        if merged_worktrees.len() == 1 { "" } else { "s" }
    );
    for (path, branch, name) in &merged_worktrees {
        println!("  {} [{}] at {}", name.yellow(), branch.cyan(), path);
    }

    // Ask for confirmation unless --force is used
    if !force {
        print!("\nPrune these worktrees? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled");
            return Ok(());
        }
    }

    // Prune each worktree
    let pruned_count = merged_worktrees.len();
    for (path, _, name) in merged_worktrees {
        print!("Pruning {}... ", name.yellow());

        let output = Command::new("git")
            .args(["worktree", "remove", &path])
            .current_dir(workdir)
            .output()
            .context("Failed to execute git worktree remove command")?;

        if output.status.success() {
            delete_note(repo, &name)?;
            println!("{}", "done".green());
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);

            // Try with --force if it contains uncommitted changes
            if stderr.contains("contains modified or untracked files") {
                print!("has uncommitted changes, removing with --force... ");

                let output = Command::new("git")
                    .args(["worktree", "remove", "--force", &path])
                    .current_dir(workdir)
                    .output()
                    .context("Failed to execute git worktree remove command")?;

                if output.status.success() {
                    delete_note(repo, &name)?;
                    println!("{}", "done".green());
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("{}", "failed".red());
                    eprintln!("  Error: {}", stderr.trim());
                }
            } else {
                println!("{}", "failed".red());
                eprintln!("  Error: {}", stderr.trim());
            }
        }
    }

    println!(
        "\nPruned {} worktree{}",
        pruned_count,
        if pruned_count == 1 { "" } else { "s" }
    );

    Ok(())
}

/// Add or show a note for a worktree
pub fn manage_note(
    repo: &Repository,
    worktree_name: &str,
    text: Option<&str>,
    ref_: Option<&str>,
) -> Result<()> {
    let mut note = load_note(repo, worktree_name)?;

    if text.is_none() && ref_.is_none() {
        // Show current note
        println!("{}", format!("Note for '{}':", worktree_name).bold());
        match &note.note {
            Some(n) => println!("  Note: {}", n),
            None => println!("  Note: {}", "(none)".dimmed()),
        }
        match &note.ref_ {
            Some(r) => println!("  Ref:  {}", r.cyan()),
            None => println!("  Ref:  {}", "(none)".dimmed()),
        }
        return Ok(());
    }

    if let Some(t) = text {
        note.note = Some(t.to_string());
    }
    if let Some(r) = ref_ {
        note.ref_ = Some(r.to_string());
    }

    save_note(repo, worktree_name, &note)?;
    println!("Saved note for '{}'", worktree_name.yellow());
    Ok(())
}

/// Show context of worktrees for AI
pub fn show_context(repo: &Repository, worktree_name: Option<&str>, save: bool) -> Result<()> {
    use std::process::Command;

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;
    let repo_name = get_repository_name(repo)?;
    let parent_dir = workdir
        .parent()
        .context("Failed to get parent directory")?;

    let mut output_lines: Vec<String> = Vec::new();

    let worktrees = collect_worktrees(repo)?;

    let targets: Vec<(String, String)> = if let Some(name) = worktree_name {
        let path = parent_dir.join(format!("{}_{}", repo_name, name));
        if !path.exists() {
            bail!("Worktree '{}' not found", name);
        }
        vec![(name.to_string(), path.to_string_lossy().to_string())]
    } else {
        worktrees
            .iter()
            .filter(|(_, path)| {
                path.trim_end_matches('/') != workdir.to_string_lossy().trim_end_matches('/')
            })
            .map(|(branch, path)| {
                let name = std::path::Path::new(path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .and_then(|n| n.strip_prefix(&format!("{}_", repo_name)))
                    .unwrap_or(branch)
                    .to_string();
                (name, path.clone())
            })
            .collect()
    };

    let detailed = worktree_name.is_some();

    for (name, path) in &targets {
        let note = load_note(repo, name)?;
        output_lines.push(format!("## {}", name));

        if let Some(n) = &note.note {
            output_lines.push(format!("Note: {}", n));
        }
        if let Some(r) = &note.ref_ {
            output_lines.push(format!("Ref: {}", r));
        }

        // Branch
        let branch_out = Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(path)
            .output();
        if let Ok(o) = branch_out {
            let branch = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !branch.is_empty() {
                output_lines.push(format!("Branch: {}", branch));
            }
        }

        // Changed files stat
        let diff_out = Command::new("git")
            .args(["diff", "main", "--stat"])
            .current_dir(path)
            .output();
        if let Ok(o) = diff_out {
            let stat = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !stat.is_empty() {
                output_lines.push(String::new());
                output_lines.push("### Changed files (vs main)".to_string());
                for line in stat.lines() {
                    output_lines.push(format!("  {}", line));
                }
            }
        }

        // Recent commits
        let log_args = if detailed {
            vec!["log", "main..HEAD", "--oneline", "--no-decorate"]
        } else {
            vec!["log", "main..HEAD", "--oneline", "--no-decorate", "-5"]
        };
        let log_out = Command::new("git")
            .args(&log_args)
            .current_dir(path)
            .output();
        if let Ok(o) = log_out {
            let log = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !log.is_empty() {
                output_lines.push(String::new());
                output_lines.push("### Commits (vs main)".to_string());
                for line in log.lines() {
                    output_lines.push(format!("  {}", line));
                }
            }
        }

        // Uncommitted changes
        let status_out = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(path)
            .output();
        if let Ok(o) = status_out {
            let status = String::from_utf8_lossy(&o.stdout).trim().to_string();
            output_lines.push(String::new());
            if status.is_empty() {
                output_lines.push("### Uncommitted changes: none".to_string());
            } else {
                output_lines.push("### Uncommitted changes".to_string());
                for line in status.lines() {
                    output_lines.push(format!("  {}", line));
                }
            }
        }

        output_lines.push(String::new());
    }

    let content = output_lines.join("\n");

    if save {
        let save_path = workdir.join(".gwtr-context.md");
        std::fs::write(&save_path, &content)?;
        println!("{}", format!("Saved to {:?}", save_path).green());
    } else {
        println!("{}", content);
    }

    Ok(())
}

fn collect_worktrees(repo: &Repository) -> Result<Vec<(String, String)>> {
    use std::process::Command;

    let workdir = repo
        .workdir()
        .context("Failed to get repository working directory")?;

    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(workdir)
        .output()
        .context("Failed to execute git worktree list command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to list worktrees: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].starts_with("worktree ") {
            let path = lines[i].strip_prefix("worktree ").unwrap_or("").to_string();
            let mut branch = String::from("detached");

            if i + 1 < lines.len() && lines[i + 1].starts_with("HEAD ") {
                i += 1;
            }
            if i + 1 < lines.len() && lines[i + 1].starts_with("branch ") {
                branch = lines[i + 1]
                    .strip_prefix("branch refs/heads/")
                    .unwrap_or("unknown")
                    .to_string();
                i += 1;
            }

            result.push((branch, path));
        }
        i += 1;
    }

    Ok(result)
}
