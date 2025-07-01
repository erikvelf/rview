mod cli;

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// This function appends the contents of a file to the code review file,
/// with a header indicating the file's path.
fn append_file_to_review(review_file: &mut File, file_path: &Path) -> io::Result<()> {
    // Write a header with the file path
    writeln!(
        review_file,
        "================== {} ==================",
        file_path.display()
    )?;

    // Open the file to be appended
    let mut file = File::open(file_path)?;
    io::copy(&mut file, review_file)?;

    // Write a newline for separation
    writeln!(review_file)?;
    Ok(())
}

/// This function recursively appends all files in a directory to the code review file.
fn append_dir_to_review(review_file: &mut File, dir_path: &Path) -> io::Result<()> {
    // Walk through the directory recursively
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            append_file_to_review(review_file, &path)?;
        } else if path.is_dir() {
            append_dir_to_review(review_file, &path)?;
        }
    }
    Ok(())
}

/// This function parses the output of `git status --porcelain` and returns a vector of file paths.
fn get_git_modified_and_untracked_files() -> io::Result<Vec<PathBuf>> {
    // Run `git status --porcelain` and capture its output
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .stdout(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Each line represents a file or directory, status is in the first 2 chars
    let mut paths = Vec::new();
    for line in stdout.lines() {
        if line.len() > 3 {
            // The file path starts at index 3
            let path = line[3..].trim();
            if !path.is_empty() {
                paths.push(PathBuf::from(path));
            }
        }
    }
    Ok(paths)
}

fn main() -> io::Result<()> {
    let args = cli::parse_args();
    
    // Debug output to show what arguments were parsed
    println!("Parsed arguments:");
    println!("  Output: {}", args.output);
    println!("  Exclude: {:?}", args.exclude);

    // Create the output directory if it doesn't exist
    if let Some(parent) = Path::new(&args.output).parent() {
        fs::create_dir_all(parent)?;
    }

    // Open (or create) the output file, truncating it if it exists
    let mut review_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&args.output)?;

    // Get the list of modified and untracked files from git
    let paths = get_git_modified_and_untracked_files()?;

    // For each path, append its contents to the review file
    for path in paths {
        if path.is_file() {
            // If it's a file, append its contents
            append_file_to_review(&mut review_file, &path)?;
        } else if path.is_dir() {
            // If it's a directory, recursively append all files inside
            append_dir_to_review(&mut review_file, &path)?;
        }
    }

    // Print a message indicating completion
    println!("Code review file created at {}", args.output);
    Ok(())
}
