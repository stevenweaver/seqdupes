mod process;

use clap::{Arg, Command};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("seqdupes")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Steven Weaver")
        .about("Reduces sequence duplicates")
        .arg(
            Arg::new("fasta")
                .help("The input FASTA file (gzip acceptable).")
                .required(true)
                .short('f')
                .long("fasta"),
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
                .long("by-header"),
        )
        .get_matches();

    crate::process::process(
        matches.get_one::<String>("fasta").unwrap(),
        matches.get_one::<String>("json").unwrap(),
        matches.contains_id("by_header"),
    )
}
