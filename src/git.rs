use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Gets modified and untracked files from git status
///
/// Uses `git status --porcelain` to get a machine-readable list of files.
/// Returns a vector of file paths that have been modified, added, or are untracked.
pub fn get_modified_files() -> io::Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::other(
            format!("Git command failed: {}", error_msg.trim()),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut paths = Vec::new();

    // Parse git status --porcelain output
    // Format: XY filename
    // X = staged status, Y = unstaged status
    for line in stdout.lines() {
        if line.len() > 3 {
            let file_path = line[3..].trim();
            if !file_path.is_empty() {
                paths.push(PathBuf::from(file_path));
            }
        }
    }

    Ok(paths)
}

/// Checks if the current directory is a git repository
pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}
