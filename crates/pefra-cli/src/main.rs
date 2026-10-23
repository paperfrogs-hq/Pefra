use std::env;
use std::process::ExitCode;

const HELP: &str = "Pefra — experimental programming language

Usage: pefra [--help | --version]

Options:
  -h, --help       Show this help message
  -V, --version    Show the CLI version

Pefra source execution is not implemented yet.";

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    match args.as_slice() {
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
        [arg] => {
            eprintln!(
                "Unsupported argument: {}. Run `pefra --help` for usage.",
                arg.to_string_lossy()
            );
            ExitCode::FAILURE
        }
        _ => {
            eprintln!("Expected one option. Run `pefra --help` for usage.");
            ExitCode::FAILURE
        }
    }
}
