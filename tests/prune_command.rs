mod common;

use common::TestHelper;
use std::process::Command;

#[test]
fn test_prune_merged_worktrees() {
    let helper = TestHelper::new().unwrap();
    
    // Initialize git repo
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    // Create initial commit
    std::fs::write(helper.repo_path.join("README.md"), "Initial").unwrap();
    Command::new("git")
        .args(&["add", "."])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    // Create a worktree
    helper.run_gwtr(&["add", "feature-merged"]);
    
    // Run prune command
    let output = helper.run_gwtr(&["prune"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should show status about merged worktrees
    assert!(stdout.contains("merged") || stdout.contains("No worktrees to prune"));
}

#[test]
fn test_prune_with_dry_run() {
    let helper = TestHelper::new().unwrap();
    
    // Initialize git repo
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    std::fs::write(helper.repo_path.join("file.txt"), "content").unwrap();
    Command::new("git")
        .args(&["add", "."])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["commit", "-m", "Init"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    // Create worktrees
    helper.run_gwtr(&["add", "feature-a"]);
    helper.run_gwtr(&["add", "feature-b"]);
    
    // Run prune with dry-run
    let output = helper.run_gwtr(&["prune", "--dry-run"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Would prune") || stdout.contains("No worktrees to prune"));
}

#[test]
fn test_prune_force() {
    let helper = TestHelper::new().unwrap();
    
    // Initialize git repo
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    std::fs::write(helper.repo_path.join("init.txt"), "init").unwrap();
    Command::new("git")
        .args(&["add", "."])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["commit", "-m", "Initial"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    // Create a worktree
    helper.run_gwtr(&["add", "feature-old"]);
    
    // Prune with force (skip confirmation)
    let output = helper.run_gwtr(&["prune", "--force"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pruned") || stdout.contains("No worktrees to prune"));
}

#[test]
fn test_prune_shows_what_would_be_removed() {
    let helper = TestHelper::new().unwrap();
    
    // Initialize git repo
    Command::new("git")
        .args(&["config", "user.email", "test@example.com"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["config", "user.name", "Test User"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    std::fs::write(helper.repo_path.join("test.txt"), "test").unwrap();
    Command::new("git")
        .args(&["add", "."])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    Command::new("git")
        .args(&["commit", "-m", "Test"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    // Create multiple worktrees
    helper.run_gwtr(&["add", "feature-1"]);
    helper.run_gwtr(&["add", "feature-2"]);
    helper.run_gwtr(&["add", "bugfix-1"]);
    
    // Run prune to see what would be removed
    let output = helper.run_gwtr(&["prune"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should list worktrees or say nothing to prune
    assert!(stdout.contains("worktree") || stdout.contains("No worktrees to prune"));
}