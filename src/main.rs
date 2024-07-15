use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

// Note the third parameter, it's a trait - allows for our function to be generic, i.e.
// testable
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            // Not sure about the expect usage here
            writeln!(writer, "{}", line).expect("Writing has failed");
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{:?}`", &args.path))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}