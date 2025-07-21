mod common;

use common::TestHelper;

#[test]
fn test_switch_to_existing_worktree() {
    let helper = TestHelper::new().unwrap();
    
    // Create a worktree first
    let output = helper.run_gwtr(&["add", "feature-x"]);
    assert!(output.status.success());
    
    // Switch to the worktree
    let output = helper.run_gwtr(&["switch", "feature-x"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cd"));
    assert!(stdout.contains("feature-x"));
}

#[test]
fn test_switch_to_nonexistent_worktree() {
    let helper = TestHelper::new().unwrap();
    
    // Try to switch to a worktree that doesn't exist
    let output = helper.run_gwtr(&["switch", "nonexistent"]);
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Worktree 'nonexistent' not found"));
}

#[test]
fn test_switch_from_main_to_worktree() {
    let helper = TestHelper::new().unwrap();
    
    // Create a worktree
    let output = helper.run_gwtr(&["add", "develop"]);
    assert!(output.status.success());
    
    // Switch from main to develop
    let output = helper.run_gwtr(&["switch", "develop"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("cd"));
    assert!(stdout.contains("develop"));
}

#[test]
fn test_switch_shows_shell_command() {
    let helper = TestHelper::new().unwrap();
    
    // Create a worktree
    let output = helper.run_gwtr(&["add", "bugfix"]);
    assert!(output.status.success());
    
    // Switch should show the cd command that user needs to run
    let output = helper.run_gwtr(&["switch", "bugfix"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Check that it outputs a cd command
    assert!(stdout.contains("cd "));
    // Check that the path includes the worktree name
    assert!(stdout.contains("bugfix"));
}