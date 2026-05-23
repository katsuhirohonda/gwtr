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
fn test_note_saves_text() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-note"]);

    let output = helper.run_gwtr(&["note", "feat-note", "JWT認証実装中"]);

    assert!(
        output.status.success(),
        "note command should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Saved note"),
        "Should show saved confirmation: {}",
        stdout
    );
}

#[test]
fn test_note_show_displays_saved_text() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-show"]);
    helper.run_gwtr(&["note", "feat-show", "テスト中のメモ"]);

    let output = helper.run_gwtr(&["note", "feat-show"]);

    assert!(output.status.success(), "note show should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("テスト中のメモ"),
        "Should display saved note: {}",
        stdout
    );
}

#[test]
fn test_note_saves_ref() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-ref"]);

    let output = helper.run_gwtr(&["note", "feat-ref", "--ref", "JIRA-123"]);

    assert!(output.status.success(), "note with --ref should succeed");

    let show = helper.run_gwtr(&["note", "feat-ref"]);
    let stdout = String::from_utf8_lossy(&show.stdout);
    assert!(
        stdout.contains("JIRA-123"),
        "Should display saved ref: {}",
        stdout
    );
}

#[test]
fn test_note_saves_text_and_ref_together() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-both"]);

    helper.run_gwtr(&["note", "feat-both", "認証実装", "--ref", "JIRA-456"]);

    let output = helper.run_gwtr(&["note", "feat-both"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("認証実装"), "Should show note text");
    assert!(stdout.contains("JIRA-456"), "Should show ref");
}

#[test]
fn test_note_show_empty_when_no_note() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-empty"]);

    let output = helper.run_gwtr(&["note", "feat-empty"]);

    assert!(output.status.success(), "note show should succeed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("none"),
        "Should show none when no note: {}",
        stdout
    );
}

#[test]
fn test_note_deleted_on_remove() {
    let helper = TestHelper::new().expect("Failed to create test helper");
    setup_repo_with_commit(&helper);
    helper.run_gwtr(&["add", "feat-del"]);
    helper.run_gwtr(&["note", "feat-del", "削除テスト"]);

    // Verify note file exists
    let note_path = helper.repo_path.join(".git/gwtr/feat-del.json");
    assert!(note_path.exists(), "Note file should exist before remove");

    helper.run_gwtr(&["remove", "feat-del"]);

    assert!(
        !note_path.exists(),
        "Note file should be deleted after remove"
    );
}
