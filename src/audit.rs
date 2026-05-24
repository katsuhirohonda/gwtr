use anyhow::{Context, Result, bail};
use colored::*;
use serde::Deserialize;
use std::path::Path;
use std::process::Command;
use strsim::levenshtein;

const POPULAR_CRATES: &[&str] = &[
    "actix-web", "anyhow", "async-trait", "axum", "base64", "bytes", "chrono",
    "clap", "colored", "console", "crossbeam", "csv", "diesel", "dirs",
    "env_logger", "futures", "git2", "glob", "hex", "home", "http", "hyper",
    "indicatif", "itertools", "lazy_static", "log", "mime", "once_cell",
    "openssl", "parking_lot", "proc-macro2", "quote", "rand", "rayon",
    "redis", "regex", "reqwest", "ring", "rocket", "rusqlite", "rustls",
    "serde", "serde_json", "sha2", "similar", "sqlx", "strsim", "syn",
    "tempfile", "thiserror", "tokio", "toml", "tower", "tracing", "url",
    "uuid", "walkdir", "warp", "which",
];

#[derive(Deserialize)]
struct CargoLock {
    package: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
}

pub fn run_audit(project_path: &Path, typo_only: bool, all: bool) -> Result<()> {
    if all {
        run_audit_all_worktrees(project_path, typo_only)
    } else {
        run_audit_single(project_path, typo_only)
    }
}

fn run_audit_all_worktrees(repo_path: &Path, typo_only: bool) -> Result<()> {
    println!("{}", "====== gwtr audit --all ======".bold());

    let worktrees = collect_worktree_paths(repo_path)?;

    if worktrees.is_empty() {
        println!("No worktrees found.");
        return Ok(());
    }

    let mut total_vuln_errors = 0usize;
    let mut total_typo_warnings = 0usize;

    for (name, path) in &worktrees {
        println!("\n{}", format!("=== worktree: {} ===", name).bold().cyan());
        let wt_path = std::path::Path::new(path);
        let lockfile = wt_path.join("Cargo.lock");

        if !lockfile.exists() {
            println!("  {} No Cargo.lock found, skipping.", "SKIP".yellow().bold());
            continue;
        }

        let (vuln_errors, typo_warnings) = audit_single_inner(wt_path, typo_only)?;
        total_vuln_errors += vuln_errors;
        total_typo_warnings += typo_warnings;
    }

    println!("\n{}", "====== Summary ======".bold());
    println!("Worktrees scanned: {}", worktrees.len());
    if total_typo_warnings > 0 {
        println!("Typosquatting warnings: {}", total_typo_warnings.to_string().yellow());
    }
    if total_vuln_errors > 0 {
        println!("Vulnerability errors:   {}", total_vuln_errors.to_string().red());
    }
    if total_typo_warnings == 0 && total_vuln_errors == 0 {
        println!("{}", "All clear!".green().bold());
    }

    Ok(())
}

fn run_audit_single(project_path: &Path, typo_only: bool) -> Result<()> {
    let lockfile_path = project_path.join("Cargo.lock");

    if !lockfile_path.exists() {
        bail!("Cargo.lock not found at {:?}", lockfile_path);
    }

    println!("{}", "====== gwtr audit ======".bold());
    println!("Scanning: {}", lockfile_path.display());

    let (vuln_errors, typo_warnings) = audit_single_inner(project_path, typo_only)?;

    println!("\n{}", "========================".bold());
    if typo_warnings > 0 {
        println!("Typosquatting warnings: {}", typo_warnings.to_string().yellow());
    }
    if vuln_errors > 0 {
        println!("Vulnerability errors:   {}", vuln_errors.to_string().red());
    }
    if typo_warnings == 0 && vuln_errors == 0 {
        println!("{}", "All clear!".green().bold());
    }

    Ok(())
}

fn audit_single_inner(project_path: &Path, typo_only: bool) -> Result<(usize, usize)> {
    let lockfile_path = project_path.join("Cargo.lock");
    let content = std::fs::read_to_string(&lockfile_path)
        .context("Failed to read Cargo.lock")?;
    let lock: CargoLock = toml::from_str(&content)
        .context("Failed to parse Cargo.lock")?;

    println!("  Packages: {}", lock.package.len());

    let vuln_errors = if !typo_only {
        run_cargo_audit(project_path)?
    } else {
        0
    };

    println!("  {}", "[Typosquatting Check]".bold());
    let mut typo_warnings = 0;

    for pkg in &lock.package {
        if let Some((similar, dist)) = check_typosquatting(&pkg.name) {
            println!(
                "    {} {} {} - similar to \"{}\" (distance: {})",
                "WARN".yellow().bold(),
                pkg.name.yellow(),
                pkg.version,
                similar.cyan(),
                dist
            );
            typo_warnings += 1;
        }
    }

    if typo_warnings == 0 {
        println!("    {}  No suspicious package names found", "OK".green().bold());
    }

    Ok((vuln_errors, typo_warnings))
}

fn collect_worktree_paths(repo_path: &Path) -> Result<Vec<(String, String)>> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .current_dir(repo_path)
        .output()
        .context("Failed to list worktrees")?;

    if !output.status.success() {
        bail!("git worktree list failed");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut result = Vec::new();
    let mut i = 0;
    let lines: Vec<&str> = stdout.lines().collect();

    while i < lines.len() {
        if lines[i].starts_with("worktree ") {
            let path = lines[i].strip_prefix("worktree ").unwrap_or("").to_string();
            let mut name = std::path::Path::new(&path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            if i + 1 < lines.len() && lines[i + 1].starts_with("HEAD ") {
                i += 1;
            }
            if i + 1 < lines.len() && lines[i + 1].starts_with("branch ") {
                let branch = lines[i + 1]
                    .strip_prefix("branch refs/heads/")
                    .unwrap_or(&name);
                name = format!("{} ({})", name, branch);
                i += 1;
            }

            result.push((name, path));
        }
        i += 1;
    }

    Ok(result)
}

fn run_cargo_audit(project_path: &Path) -> Result<usize> {
    println!("  {}", "[Vulnerability Check (cargo audit)]".bold());

    let installed = Command::new("cargo")
        .args(["audit", "--version"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !installed {
        println!("    {}  cargo audit is not installed.", "SKIP".yellow().bold());
        println!("    Install: {}", "cargo install cargo-audit".cyan());
        return Ok(0);
    }

    let output = Command::new("cargo")
        .args(["audit"])
        .current_dir(project_path)
        .output()
        .context("Failed to run cargo audit")?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    for line in stdout.lines() {
        println!("    {}", line);
    }
    for line in stderr.lines().filter(|l| !l.is_empty()) {
        println!("    {}", line);
    }

    let errors = if output.status.success() { 0 } else { 1 };
    Ok(errors)
}

fn check_typosquatting(name: &str) -> Option<(&'static str, usize)> {
    if POPULAR_CRATES.contains(&name) {
        return None;
    }

    // Skip very short names - too many false positives
    if name.len() < 4 {
        return None;
    }

    POPULAR_CRATES
        .iter()
        .filter_map(|&popular| {
            // Only compare against crates with similar length (within 3 chars)
            if name.len().abs_diff(popular.len()) > 3 {
                return None;
            }
            let dist = levenshtein(name, popular);
            let threshold = if name.len() <= 5 { 1 } else { 2 };
            if dist > 0 && dist <= threshold {
                Some((popular, dist))
            } else {
                None
            }
        })
        .min_by_key(|&(_, dist)| dist)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match_is_not_flagged() {
        assert!(check_typosquatting("serde").is_none());
        assert!(check_typosquatting("tokio").is_none());
        assert!(check_typosquatting("reqwest").is_none());
    }

    #[test]
    fn typosquatting_detected() {
        // "reqest" is 1 edit away from "reqwest"
        let result = check_typosquatting("reqest");
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, "reqwest");
    }

    #[test]
    fn short_names_are_skipped() {
        assert!(check_typosquatting("cc").is_none());
        assert!(check_typosquatting("yok").is_none());
    }

    #[test]
    fn unrelated_crate_is_not_flagged() {
        assert!(check_typosquatting("my-custom-crate-xyz").is_none());
        assert!(check_typosquatting("completely-different").is_none());
    }

    #[test]
    fn length_difference_too_large_is_skipped() {
        // "se" vs "serde" - length diff is 3, name too short anyway
        assert!(check_typosquatting("se").is_none());
    }
}
