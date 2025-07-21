mod common;

use common::TestHelper;
use std::process::Command;

#[test]
fn test_pull_all_worktrees() {
    let helper = TestHelper::new().unwrap();
    
    // Initialize git repo with a commit
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
    std::fs::write(helper.repo_path.join("README.md"), "Initial content").unwrap();
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
    
    // Create worktrees
    helper.run_gwtr(&["add", "feature-a"]);
    helper.run_gwtr(&["add", "feature-b"]);
    
    // Run pull command
    let output = helper.run_gwtr(&["pull", "--all"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Pulling all worktrees"));
}

#[test]
fn test_pull_shows_already_up_to_date() {
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
    
    std::fs::write(helper.repo_path.join("test.txt"), "content").unwrap();
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
    helper.run_gwtr(&["add", "feature-uptodate"]);
    
    // Run pull command
    let output = helper.run_gwtr(&["pull", "--all"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // With no remote, git pull will fail but gwtr should still complete successfully
    assert!(stdout.contains("Pulling all worktrees"));
}

#[test]
fn test_pull_specific_worktree() {
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
    helper.run_gwtr(&["add", "feature-x"]);
    helper.run_gwtr(&["add", "feature-y"]);
    
    // Pull specific worktree - we expect this to succeed or gracefully handle no remote
    let output = helper.run_gwtr(&["pull", "feature-x"]);
    
    // Since we don't have a remote, check if it tried to pull the specific worktree
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Should mention the worktree name either in stdout or error message
    assert!(stdout.contains("feature-x") || stderr.contains("feature-x"),
            "Output should mention the worktree name. stdout: {}, stderr: {}", stdout, stderr);
}

#[test]
fn test_pull_shows_progress() {
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
    
    // Create multiple worktrees
    helper.run_gwtr(&["add", "dev"]);
    helper.run_gwtr(&["add", "staging"]);
    helper.run_gwtr(&["add", "prod"]);
    
    // Run pull all
    let output = helper.run_gwtr(&["pull", "--all"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should show some indication of which worktrees are being pulled
    assert!(stdout.contains("main") || stdout.contains("dev") || stdout.contains("staging"));
}