use structopt::StructOpt;
use anyhow::{Context, Result};
use colored::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    print_lines(&content, &mut std::io::stdout());

    Ok(())
}

fn print_lines(content: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        writeln!(writer, "{}", line.magenta()).ok();
    }
}

// #[test]
// fn find_a_match() {
//     let mut result = Vec::new();
//     find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
//     assert_eq!(result, b"lorem ipsum\n");
// }
