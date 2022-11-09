use bio::io::fasta;
use serde_json::{Map, Value};
use serde_json::json;
use regex::RegexSet;
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::str;

use std::collections::HashMap;

// String Match Option
pub fn compress_duplicates<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    output_json: P
) -> bool {

    //FASTA file related
    let file = Path::new(&filename).to_str().unwrap();
    let mut records = fasta::Reader::from_file(file).unwrap().records();
    let mut seq_duplicates: HashMap<String, Vec<String>> = HashMap::new();
    let mut dupe_headers = Map::new();

    // write JSON file
    let mut writer = fasta::Writer::new(io::stdout());

    // Add sequence to hash if doesn't already exist, then add header.
    // if the sequence already exists, add header to vector.


    // Gather data from every record
    while let Some(record) = records.next() {

        let seqrec = record.unwrap();

        let sequence_id_bytes = seqrec.id();
        let sequence_description_bytes = seqrec.desc().unwrap_or("");

        let entire_header_raw = [sequence_id_bytes, sequence_description_bytes].join(" ");
        let entire_header = entire_header_raw.trim();
        let seq_str = str::from_utf8(seqrec.seq()).unwrap();

        let seq = seq_duplicates.get_mut(seq_str);

        if let Some(s) = seq {
            s.push(entire_header.to_string());
        } else {
            seq_duplicates.insert(seq_str.to_owned(), vec![entire_header.to_string()]);
        }
    }

    // Write out FASTA with no duplicates, and write out a JSON containing only duplicates
    for (key, value) in &seq_duplicates {
        //println!("{}: {:?}", key, value);
        let sequence_id = value.last().unwrap();
        dupe_headers.insert(sequence_id.to_string(), json!(value));

        writer
            .write(&sequence_id, None, key.as_bytes())
            .expect("Error writing record.");
    }

    let mut file = fs::File::create(&output_json).unwrap();
    file.write_all(json!(dupe_headers).to_string().as_bytes());

    return true;
}

pub(crate) fn process<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    output_json: P, 
) -> Result<(), Box<dyn Error>> {
    compress_duplicates(filename, output_json);
    Ok(())
}
