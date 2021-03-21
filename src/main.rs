extern crate structopt;

use structopt::StructOpt;

use std::path::PathBuf;
mod base;

/// The default script to run if none supplied
const DEFAULT_SCRIPT: &'static str = "./run.comfy";

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Cross-platform script/command manager and runner\nBy default tries to run script ./run.comfy"
)]
struct Arguments {
    #[structopt(subcommand)]
    pub subcommand: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Scripting help
    #[structopt(name = "helpf")]
    HelpF,

    /// Run a script
    Run {
        /// Path to Comfy Script to run
        file: Option<PathBuf>,
        /// Show comments from source while running
        #[structopt(short, long)]
        comments: bool,
    },
}

fn main() {
    let args = Arguments::from_args();

    match args.subcommand.unwrap_or_else(|| Command::Run {
        file: None,
        comments: false,
    }) {
        Command::HelpF => print_helpf(),
        Command::Run { file, comments } => base::parse(
            &file.unwrap_or_else(|| PathBuf::from(DEFAULT_SCRIPT)),
            comments,
        ),
    }
}

fn print_helpf() {
    println!("comfy {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("  @[space]function         is how you call a function       ");
    println!("  sleep [int]              sleeps your program for [int] ms ");
    println!();
}
