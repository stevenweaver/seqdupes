mod process;

use clap::{App, Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("seqdupes")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Steven Weaver")
        .about("Reduces sequence duplicates")
        .arg(
            Arg::new("fasta")
                .help("The input FASTA file (gzip acceptable).")
                .takes_value(true)
                .required(true)
                .short('f')
                .long("fasta"),
        )
        .arg(
            Arg::new("json")
                .help("The duplicate list in JSON")
                .takes_value(true)
                .required(true)
                .short('j')
                .long("json"),
        )
        .get_matches();

    crate::process::process(
        matches.value_of("fasta").unwrap(),
        matches.value_of("json").unwrap(),
    )
}
