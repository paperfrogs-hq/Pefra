use std::env;
use std::process::ExitCode;

const HELP: &str = "Pefra — experimental programming language

Usage: pefra [--help | --version]

Options:
  -h, --help       Show this help message
  -V, --version    Show the CLI version

Pefra source execution is not implemented yet.";

fn main() -> ExitCode {
    match env::args().skip(1).collect::<Vec<_>>().as_slice() {
        [] => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        [arg] if arg == "--help" || arg == "-h" => {
            println!("{HELP}");
            ExitCode::SUCCESS
        }
        [arg] if arg == "--version" || arg == "-V" => {
            println!("pefra {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!("Unsupported arguments. Run `pefra --help` for usage.");
            ExitCode::FAILURE
        }
    }
}
