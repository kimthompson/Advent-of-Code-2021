use structopt::StructOpt;
use anyhow::{Context, Result};
use colored::*;

// mod helpers;

/// Count the number of times the example data increments from one number to the next
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    sonar(&content, &mut std::io::stdout());
    sonar_sweep(&content, &mut std::io::stdout());

    Ok(())
}

fn sonar(content: &str, mut writer: impl std::io::Write) {
    let mut comp_num: i32 = 1000000;
    let mut ans: i32 = 0;
    for line in content.lines() {
        let current_num: i32 = line.parse().unwrap();
        if current_num > comp_num {
            ans += 1;
        }
        comp_num = current_num;
    }
    writeln!(writer, "{} {}", "Sonar:".blue().bold(), ans).ok();
}

fn sonar_sweep(content: &str, mut writer: impl std::io::Write) {
    let mut comp_sum: i32 = 1000000;
    let mut ans: i32 = 0;
    let count: usize = content.lines().count();

    for i in 0..count {
        let pos_a: Option<&str> = content.lines().nth(i);
        let pos_b: Option<&str> = content.lines().nth(i + 1);
        let pos_c: Option<&str> = content.lines().nth(i + 2);
        if pos_a != None && pos_b != None && pos_c != None {
            let num_a: i32 = pos_a.unwrap().parse::<i32>().unwrap();
            let num_b: i32 = pos_b.unwrap().parse::<i32>().unwrap();
            let num_c: i32 = pos_c.unwrap().parse::<i32>().unwrap();
            let current_sum: i32 = num_a + num_b + num_c;
            // writeln!(writer, "{}: {}, {}, {} = {}", i, num_a, num_b, num_c, current_sum).ok();
            if current_sum > comp_sum {
                ans += 1;
            }
            comp_sum = current_sum;
        }
    }
    writeln!(writer, "{} {}", "Sonar Sweep:".blue().bold(), ans).ok();
}

// #[test]
// fn find_a_match() {
//     let mut result = Vec::new();
//     find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
//     assert_eq!(result, b"lorem ipsum\n");
// }
