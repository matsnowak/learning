use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<()> {
    let args = CliArgs::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let file = std::fs::File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        if let Ok(line_internal) = line {
            if line_internal.contains(&args.pattern) {
                println!("{line_internal}");
            }
        }
    });

    Ok(())
}
