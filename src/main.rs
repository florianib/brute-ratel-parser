use brute_ratel_parser::{combine, create_excel};
use clap::{arg, command, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(long, action)]
    combine: bool,
    #[arg(short, long)]
    path: PathBuf,
    #[clap(long, action)]
    generate: bool,
}

fn main() {
    let cli = Cli::parse();

    let log_path = cli.path.as_path();

    if cli.combine {
        combine(log_path);
    }

    if cli.generate {
        create_excel(log_path);
    }

    println!("Done");
}
