mod common;

use common::TestHelper;
use std::fs;

fn setup_repo_with_commit(helper: &TestHelper) {
    fs::write(helper.repo_path.join("README.md"), "# Test Repo").unwrap();
    std::process::Command::new("git")
        .args(["add", "."])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
    std::process::Command::new("git")
        .args(["commit", "-m", "Initial commit"])
        .current_dir(&helper.repo_path)
        .output()
        .unwrap();
}

#[test]
fn test_context_shows_all_worktrees() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-ctx-a"]);
    helper.run_gwtr(&["add", "feat-ctx-b"]);

    let output = helper.run_gwtr(&["context"]);

    assert!(
        output.status.success(),
        "context should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("feat-ctx-a"), "Should show feat-ctx-a");
    assert!(stdout.contains("feat-ctx-b"), "Should show feat-ctx-b");
}

#[test]
fn test_context_specific_worktree() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-specific"]);
    helper.run_gwtr(&["add", "feat-other"]);

    let output = helper.run_gwtr(&["context", "feat-specific"]);

    assert!(output.status.success(), "context with name should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("feat-specific"), "Should show target worktree");
}

#[test]
fn test_context_shows_note_when_set() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-noted"]);
    helper.run_gwtr(&["note", "feat-noted", "JWTトークン実装中"]);

    let output = helper.run_gwtr(&["context", "feat-noted"]);

    assert!(output.status.success(), "context should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("JWTトークン実装中"),
        "Should include note text: {}",
        stdout
    );
}

#[test]
fn test_context_shows_ref_when_set() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-refctx"]);
    helper.run_gwtr(&["note", "feat-refctx", "--ref", "JIRA-999"]);

    let output = helper.run_gwtr(&["context", "feat-refctx"]);

    assert!(output.status.success(), "context should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("JIRA-999"), "Should include ref: {}", stdout);
}

#[test]
fn test_context_save_creates_file() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-save"]);

    let output = helper.run_gwtr(&["context", "feat-save", "--save"]);

    assert!(output.status.success(), "context --save should succeed");
    let saved_file = helper.repo_path.join(".gwtr-context.md");
    assert!(saved_file.exists(), "Should create .gwtr-context.md");

    let content = fs::read_to_string(&saved_file).unwrap();
    assert!(content.contains("feat-save"), "Saved file should contain worktree name");
}

#[test]
fn test_context_fails_for_nonexistent_worktree() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);

    let output = helper.run_gwtr(&["context", "nonexistent"]);

    assert!(
        !output.status.success(),
        "context should fail for nonexistent worktree"
    );
}
