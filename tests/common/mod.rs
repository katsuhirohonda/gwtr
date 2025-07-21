use std::process::Command;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper struct for running gwtr commands in tests
pub struct TestHelper {
    _temp_dir: TempDir,  // Keeps the directory alive until dropped
    pub repo_path: PathBuf,
}

impl TestHelper {
    /// Create a new test helper with a temporary git repository
    pub fn new() -> anyhow::Result<Self> {
        let temp_dir = TempDir::new()?;
        let repo_path = temp_dir.path().to_path_buf();
        
        // Initialize git repository
        Command::new("git")
            .args(&["init"])
            .current_dir(&repo_path)
            .output()?;
        
        Ok(Self { _temp_dir: temp_dir, repo_path })
    }
    
    /// Run gwtr command with arguments
    pub fn run_gwtr(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_gwtr"))
            .args(args)
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to execute gwtr")
    }
}