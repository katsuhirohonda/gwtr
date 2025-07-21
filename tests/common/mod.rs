use std::process::Command;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper struct for running gwtr commands in tests
pub struct TestHelper {
    pub temp_dir: TempDir,
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
        
        Ok(Self { temp_dir, repo_path })
    }
    
    /// Run gwtr command with arguments
    pub fn run_gwtr(&self, args: &[&str]) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_gwtr"))
            .args(args)
            .current_dir(&self.repo_path)
            .output()
            .expect("Failed to execute gwtr")
    }
    
    /// Run gwtr command and return stdout as string
    pub fn run_gwtr_success(&self, args: &[&str]) -> String {
        let output = self.run_gwtr(args);
        assert!(output.status.success(), "Command failed: {:?}", output);
        String::from_utf8_lossy(&output.stdout).to_string()
    }
}