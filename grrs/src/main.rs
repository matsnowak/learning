use std::io::{BufRead, BufReader, Write};

use anyhow::{Context, Result};
use clap::Parser;
#[derive(Parser)]
struct CliArgs {
    pattern: String,
    path: std::path::PathBuf,
}
fn main() -> Result<()> {
    let args = CliArgs::parse();

    let file = std::fs::File::open(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let reader = BufReader::new(file);
    find_matches(reader, &args.pattern, &mut std::io::stdout());

    Ok(())
}

fn find_matches(reader: impl BufRead, pattern: &str, mut writer: impl Write) {
    reader.lines().for_each(|line| {
        if let Ok(line_internal) = line {
            if line_internal.contains(pattern) {
                writeln!(writer, "{line_internal}").expect("Can now write to output");
            }
        }
    });
}
#[cfg(test)]
mod test {
    use std::io::{BufReader, Cursor};

    use crate::find_matches;

    #[test]
    fn should_find_matches() {
        let mut result = Vec::new();
        let input = ["lorem ipsum\n", "dolor sit amet\n"].concat();
        let reader = BufReader::new(Cursor::new(input));
        find_matches(reader, "lorem", &mut result);

        let output = vec![String::from_utf8(result).unwrap()];
        assert_eq!(output, vec!["lorem ipsum\n"]);
    }
}
