//! `q4`: the command-line referee for quaternary codes.

use std::process::ExitCode;

const USAGE: &str = "\
q4: an open referee for quaternary codes

usage: q4 [--help | --version]

No distance commands exist yet; see ROADMAP.md for what lands in v0.1.";

/// Exit status for a malformed command line. `q4 distance` will reserve 1 for a
/// rejected certificate and 3 for a code too large for the engine.
const USAGE_ERROR: u8 = 2;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["--help" | "-h"] => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        ["--version" | "-V"] => {
            println!("q4 {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        [] => {
            eprintln!("{USAGE}");
            ExitCode::from(USAGE_ERROR)
        }
        [other, ..] => {
            eprintln!("q4: unknown argument `{other}`\n\n{USAGE}");
            ExitCode::from(USAGE_ERROR)
        }
    }
}
