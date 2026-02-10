mod process;

use clap::{Arg, ArgAction, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("seqdupes")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Steven Weaver")
        .about("Reduces sequence duplicates")
        .arg(
            Arg::new("fasta")
                .help("The input FASTA/FASTQ file (gzip acceptable, use '-' for stdin)")
                .short('f')
                .long("fasta")
                .default_value("-"),
        )
        .arg(
            Arg::new("json")
                .help("The duplicate list in JSON")
                .required(true)
                .short('j')
                .long("json"),
        )
        .arg(
            Arg::new("by_header")
                .help("Filters duplicates based on headers")
                .short('b')
                .long("by-header")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let mut stdout = std::io::stdout();
    crate::process::process(
        matches.get_one::<String>("fasta").unwrap(),
        matches.get_one::<String>("json").unwrap(),
        matches.get_flag("by_header"),
        &mut stdout,
    )
}
