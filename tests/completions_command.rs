mod common;

use common::TestHelper;

#[test]
fn test_completions_zsh_outputs_script() {
    let helper = TestHelper::new().expect("Failed to create test helper");

    let output = helper.run_gwtr(&["completions", "zsh"]);

    assert!(
        output.status.success(),
        "completions zsh should succeed: stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.is_empty(),
        "completions zsh should produce non-empty output"
    );
    assert!(
        stdout.contains("gwtr"),
        "zsh completion script should reference the binary name. Actual output: {}",
        stdout
    );
    assert!(
        stdout.contains("#compdef gwtr"),
        "zsh completion script should declare #compdef. Actual output: {}",
        stdout
    );
}

#[test]
fn test_completions_bash_outputs_script() {
    let helper = TestHelper::new().expect("Failed to create test helper");

    let output = helper.run_gwtr(&["completions", "bash"]);

    assert!(
        output.status.success(),
        "completions bash should succeed: stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.is_empty(),
        "completions bash should produce non-empty output"
    );
    assert!(
        stdout.contains("gwtr"),
        "bash completion script should reference the binary name. Actual output: {}",
        stdout
    );
    assert!(
        stdout.contains("_gwtr"),
        "bash completion script should define _gwtr function. Actual output: {}",
        stdout
    );
}

#[test]
fn test_completions_outside_git_repo_succeeds() {
    // Completions generation must not require a git repository.
    // We verify by running in a fresh tempdir without a .git directory.
    let temp = tempfile::TempDir::new().expect("Failed to create tempdir");
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_gwtr"))
        .args(["completions", "zsh"])
        .current_dir(temp.path())
        .output()
        .expect("Failed to execute gwtr");

    assert!(
        output.status.success(),
        "completions should not require a git repository: stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_completions_requires_shell_argument() {
    let helper = TestHelper::new().expect("Failed to create test helper");

    let output = helper.run_gwtr(&["completions"]);

    assert!(
        !output.status.success(),
        "completions without shell argument should fail"
    );
}
