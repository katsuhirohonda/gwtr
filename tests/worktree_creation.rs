mod common;

use common::TestHelper;
use std::fs;

#[test]
fn test_add_creates_worktree_at_correct_location() {
    // Setup: Create a test git repository
    let helper = TestHelper::new().expect("Failed to create test helper");
    
    // Create initial commit (worktree requires at least one commit)
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
    
    // Get the repository name for testing
    let repo_dir = helper.repo_path.file_name().unwrap().to_str().unwrap();
    
    // Run gwtr add to create a worktree
    let output = helper.run_gwtr(&["add", "feature-x"]);
    
    // Should succeed
    assert!(output.status.success(), 
            "add command should succeed: stderr: {}", 
            String::from_utf8_lossy(&output.stderr));
    
    // Check that worktree was created at the correct location
    // Should be at ../test-repo_feature-x relative to the repo
    let expected_worktree = helper.repo_path
        .parent()
        .unwrap()
        .join(format!("{}_feature-x", repo_dir));
    
    assert!(expected_worktree.exists(), 
            "Worktree should be created at {:?}", expected_worktree);
    
    // Verify it's a valid git worktree
    assert!(expected_worktree.join(".git").exists(), 
            "Created directory should be a git worktree");
}

#[test]
fn test_add_creates_worktree_with_new_branch() {
    // Setup
    let helper = TestHelper::new().expect("Failed to create test helper");
    
    // Create initial commit (worktree requires at least one commit)
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
    
    // Run gwtr add
    let output = helper.run_gwtr(&["add", "feature-y"]);
    
    assert!(output.status.success(), 
            "add command should succeed: stderr: {}", 
            String::from_utf8_lossy(&output.stderr));
    
    // Get the worktree path
    let repo_dir = helper.repo_path.file_name().unwrap().to_str().unwrap();
    let worktree_path = helper.repo_path
        .parent()
        .unwrap()
        .join(format!("{}_feature-y", repo_dir));
    
    // Check that the worktree is on a new branch
    let branch_output = std::process::Command::new("git")
        .args(&["branch", "--show-current"])
        .current_dir(&worktree_path)
        .output()
        .unwrap();
    
    let branch_name = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();
    assert_eq!(branch_name, "feature-y", 
               "Worktree should be on branch 'feature-y'");
}

#[test]
fn test_add_fails_if_worktree_already_exists() {
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
    
    // Create worktree for the first time
    let output = helper.run_gwtr(&["add", "feature-z"]);
    assert!(output.status.success(), "First add should succeed");
    
    // Try to create the same worktree again
    let output = helper.run_gwtr(&["add", "feature-z"]);
    
    assert!(!output.status.success(), 
            "add command should fail when worktree already exists");
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already exists") || stderr.contains("already"), 
            "Error message should indicate worktree already exists: {}", stderr);
}