mod cli;
mod colors;
mod config;
mod filter;
mod git;

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use colors::{green_if_tty, yellow_if_tty, blue_if_tty};
use config::Config;
use filter::FileFilter;

/// This function appends the contents of a file to the code review file,
/// with a header indicating the file's path.
fn append_file_to_review(
    review_file: &mut File,
    file_path: &Path,
    config: &Config,
) -> io::Result<()> {
    // Write a header with the file path
    writeln!(
        review_file,
        "{} {} {}",
        config.get_separator(),
        file_path.display(),
        config.get_separator()
    )?;

    // Open the file to be appended
    let mut file = File::open(file_path)?;
    io::copy(&mut file, review_file)?;

    // Write a newline for separation
    writeln!(review_file)?;
    Ok(())
}

/// This function recursively appends all files in a directory to the code review file.
fn append_dir_to_review(
    review_file: &mut File,
    dir_path: &Path,
    config: &Config,
    filter: &FileFilter,
) -> io::Result<()> {
    // Walk through the directory recursively
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if filter.should_exclude(&path) {
            continue;
        }

        if path.is_file() {
            append_file_to_review(review_file, &path, config)?;
        } else if path.is_dir() {
            append_dir_to_review(review_file, &path, config, filter)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = cli::parse_args();

    // Check if we're in a git repository
    if !git::is_git_repo() {
        eprintln!("Error: Not in a git repository");
        std::process::exit(1);
    }

    // Create configuration from CLI args
    let config = Config::new().with_cli_args(args.output, args.exclude);

    // Create file filter to prevent infinite loops
    let filter = FileFilter::from_config(&config);

    // Create the output directory if it doesn't exist
    if let Some(parent) = Path::new(&config.output_file).parent() {
        fs::create_dir_all(parent)?;
    }

    // Open (or create) the output file, truncating it if it exists
    let mut review_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&config.output_file)?;

    // Get the list of modified and untracked files from git
    let paths = git::get_modified_files()?;

    if paths.is_empty() {
        println!("No modified files found");
        return Ok(());
    }

    // Filter and categorize files
    let mut added_files = Vec::new();
    let mut excluded_files = Vec::new();

    for path in &paths {
        if filter.should_exclude(path) {
            excluded_files.push(path);
        } else {
            added_files.push(path);
        }
    }

    // Process added files
    for path in &added_files {
        if path.is_file() {
            append_file_to_review(&mut review_file, path, &config)?;
        } else if path.is_dir() {
            append_dir_to_review(&mut review_file, path, &config, &filter)?;
        }
    }

    // Display results
    if !added_files.is_empty() {
        println!("{}:", green_if_tty("Added"));
        for path in &added_files {
            println!("  {}", path.display());
        }
    }

    if args.verbose && !excluded_files.is_empty() {
        println!("\n{}:", yellow_if_tty("Excluded"));
        for path in &excluded_files {
            println!("  {}", path.display());
        }
    }

    println!(
        "\n{} {} ({})",
        blue_if_tty("Code review file created at"),
        config.output_file,
        blue_if_tty(&format!("{} files processed", added_files.len()))
    );
    Ok(())
}
