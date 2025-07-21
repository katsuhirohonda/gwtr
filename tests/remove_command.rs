mod common;

use common::TestHelper;
use std::fs;

#[test]
fn test_remove_removes_worktree() {
    // Setup
    let helper = TestHelper::new().expect("Failed to create test helper");
    
    // Create initial commit
    fs::write(helper.repo_path.join("README.md"), "# Test Repo").unwrap();
    std::process::Command::new("git")
        .args(&["add", "."])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    // Create a worktree
    let output = helper.run_gwtr(&["add", "feature-remove"]);
    assert!(output.status.success(), "Failed to create worktree");
    
    // Verify worktree exists
    let repo_name = helper.repo_path.file_name().unwrap().to_str().unwrap();
    let worktree_path = helper.repo_path
        .parent()
        .unwrap()
        .join(format!("{}_feature-remove", repo_name));
    assert!(worktree_path.exists(), "Worktree should exist before removal");
    
    // Remove the worktree
    let output = helper.run_gwtr(&["remove", "feature-remove"]);
    
    assert!(output.status.success(), 
            "remove command should succeed: stderr: {}", 
            String::from_utf8_lossy(&output.stderr));
    
    // Verify worktree is removed
    assert!(!worktree_path.exists(), 
            "Worktree directory should be removed");
}

#[test]
fn test_remove_fails_if_worktree_not_exists() {
    // Setup
    let helper = TestHelper::new().expect("Failed to create test helper");
    
    // Create initial commit
    fs::write(helper.repo_path.join("README.md"), "# Test Repo").unwrap();
    std::process::Command::new("git")
        .args(&["add", "."])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    // Try to remove non-existent worktree
    let output = helper.run_gwtr(&["remove", "non-existent"]);
    
    assert!(!output.status.success(), 
            "remove command should fail for non-existent worktree");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("does not exist"), 
            "Error message should indicate worktree not found: {}", stderr);
}

#[test]
fn test_remove_shows_confirmation_message() {
    // Setup
    let helper = TestHelper::new().expect("Failed to create test helper");
    
    // Create initial commit
    fs::write(helper.repo_path.join("README.md"), "# Test Repo").unwrap();
    std::process::Command::new("git")
        .args(&["add", "."])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(&["commit", "-m", "Initial commit"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    
    // Create and remove a worktree
    helper.run_gwtr(&["add", "feature-confirm"]);
    let output = helper.run_gwtr(&["remove", "feature-confirm"]);
    
    assert!(output.status.success(), "remove command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Removed worktree") || stdout.contains("feature-confirm"),
            "Should show confirmation message");
}