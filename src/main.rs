mod commands;
mod sounds;
mod errors;
mod config;
#[allow(deprecated)]
mod utils;
use clap::{Parser, Subcommand};
use commands::start::start;
use console::Term;
use rodio::OutputStream;
use utils::create_pulse_directory;

#[derive(Parser)]
#[clap(name = "Pulse", version = "0.0.1", author = "Shubham Singh")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(name = "start")]
    Start {
        /// debug flag, to print debug information (-d, --debug)
        #[clap(short, long)]
        debug: bool,
    }
}

fn main() {
    tracing_subscriber::fmt::init();
    let _ = create_pulse_directory();
    let _ = Term::buffered_stdout();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let args: CLI = CLI::parse();
    let _ = match args.command {
        Commands::Start { debug } => start(debug, stream_handle),
    };
}
