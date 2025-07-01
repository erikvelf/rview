use clap::{Arg, Command};

/// Represents the command line arguments for rview
#[derive(Debug)]
pub struct Args {
    /// Output file path
    pub output: String,
    /// Exclude patterns (comma-separated)
    pub exclude: Vec<String>,
}

/// Parses command line arguments and returns Args struct
pub fn parse_args() -> Args {
    let matches = Command::new("rview")
        .version("0.1.0")
        .about("Aggregates modified files into a single review file for AI consumption")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output file path")
                .default_value("docs/code-review.txt")
        )
        .arg(
            Arg::new("exclude")
                .short('x')
                .long("exclude")
                .value_name("PATTERNS")
                .help("Exclude file patterns (comma-separated)")
                .default_value("")
        )
        .get_matches();

    let output = matches.get_one::<String>("output").unwrap().clone();
    let exclude_str = matches.get_one::<String>("exclude").unwrap();
    let exclude = if exclude_str.is_empty() {
        Vec::new()
    } else {
        exclude_str.split(',').map(|s| s.trim().to_string()).collect()
    };

    Args { output, exclude }
}