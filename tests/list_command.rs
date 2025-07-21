mod common;

use common::TestHelper;
use std::fs;

#[test]
fn test_list_shows_no_worktrees_initially() {
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
    
    // Run gwtr list
    let output = helper.run_gwtr(&["list"]);
    
    assert!(output.status.success(), 
            "list command should succeed: stderr: {}", 
            String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No worktrees found") || stdout.contains("main"), 
            "Should indicate no worktrees or show only main");
}

#[test]
fn test_list_shows_created_worktrees() {
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
    
    // Create worktrees
    helper.run_gwtr(&["add", "feature-a"]);
    helper.run_gwtr(&["add", "feature-b"]);
    
    // Run gwtr list
    let output = helper.run_gwtr(&["list"]);
    
    assert!(output.status.success(), 
            "list command should succeed: stderr: {}", 
            String::from_utf8_lossy(&output.stderr));
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Should show the created worktrees
    assert!(stdout.contains("feature-a"), 
            "Should show feature-a worktree");
    assert!(stdout.contains("feature-b"), 
            "Should show feature-b worktree");
}

#[test]
fn test_list_shows_worktree_paths() {
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
    helper.run_gwtr(&["add", "feature-x"]);
    
    // Run gwtr list
    let output = helper.run_gwtr(&["list"]);
    
    assert!(output.status.success(), 
            "list command should succeed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let repo_name = helper.repo_path.file_name().unwrap().to_str().unwrap();
    
    // Should show the worktree path
    assert!(stdout.contains(&format!("{}_feature-x", repo_name)) || 
            stdout.contains("feature-x"),
            "Should show worktree path or name");
}