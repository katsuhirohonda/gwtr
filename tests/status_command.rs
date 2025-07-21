mod common;

use common::TestHelper;
use std::fs::File;
use std::io::Write;

#[test]
fn test_status_shows_all_worktrees() {
    let helper = TestHelper::new().unwrap();
    
    // Create multiple worktrees
    helper.run_gwtr(&["add", "feature-x"]);
    helper.run_gwtr(&["add", "bugfix-y"]);
    
    // Run status command
    let output = helper.run_gwtr(&["status"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Worktrees:"));
    assert!(stdout.contains("(main)"));
    assert!(stdout.contains("feature-x"));
    assert!(stdout.contains("bugfix-y"));
}

#[test]
fn test_status_shows_clean_state() {
    let helper = TestHelper::new().unwrap();
    
    // Create a worktree
    helper.run_gwtr(&["add", "feature-clean"]);
    
    // Run status command
    let output = helper.run_gwtr(&["status"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("clean"));
}

#[test]
fn test_status_shows_uncommitted_changes() {
    let helper = TestHelper::new().unwrap();
    
    // Create a worktree
    helper.run_gwtr(&["add", "feature-dirty"]);
    
    // Create uncommitted changes in the worktree
    let worktree_path = helper.repo_path.parent().unwrap()
        .join(format!("{}_{}", helper.repo_path.file_name().unwrap().to_str().unwrap(), "feature-dirty"));
    
    // Create a new file in the worktree
    let test_file_path = worktree_path.join("test.txt");
    let mut file = File::create(&test_file_path).unwrap();
    writeln!(file, "test content").unwrap();
    
    // Run status command
    let output = helper.run_gwtr(&["status"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("uncommitted changes") || stdout.contains("modified"));
}

#[test]
fn test_status_shows_current_branch() {
    let helper = TestHelper::new().unwrap();
    
    // Create worktrees
    helper.run_gwtr(&["add", "develop"]);
    helper.run_gwtr(&["add", "release"]);
    
    // Run status command
    let output = helper.run_gwtr(&["status"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should show branch names
    assert!(stdout.contains("[main]") || stdout.contains("main"));
    assert!(stdout.contains("[develop]") || stdout.contains("develop"));
    assert!(stdout.contains("[release]") || stdout.contains("release"));
}

#[test]
fn test_status_with_no_worktrees() {
    let helper = TestHelper::new().unwrap();
    
    // Run status command without any worktrees
    let output = helper.run_gwtr(&["status"]);
    assert!(output.status.success());
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Worktrees:"));
    // Should still show main worktree
    assert!(stdout.contains("(main)"));
}