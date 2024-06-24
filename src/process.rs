use bio::io::fasta;
use serde_json::{json, Map};
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::str;
use std::collections::HashMap;

pub fn compress_duplicates<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    output_json: P,
    filter_by_header: bool,
) -> bool {
    let file = Path::new(&filename).to_str().unwrap();
    let records = fasta::Reader::from_file(file).unwrap().records();
    let mut seq_duplicates: HashMap<String, Vec<String>> = HashMap::new();
    let mut dupe_headers = Map::new();
    let mut writer = fasta::Writer::new(io::stdout());

    for record in records {
        let seqrec = record.unwrap();
        let entire_header = format!("{} {}", seqrec.id(), seqrec.desc().unwrap_or(""))
            .trim()
            .to_string();
        let seq_str = str::from_utf8(seqrec.seq()).unwrap();
        let key = if filter_by_header { entire_header.clone() } else { seq_str.to_owned() };

        if let Some(headers) = seq_duplicates.get_mut(&key) {
            headers.push(entire_header);
        } else {
            seq_duplicates.insert(key, vec![entire_header]);
        }
    }

    for (key, value) in &seq_duplicates {
        let sequence_id = value.last().unwrap();
        dupe_headers.insert(sequence_id.to_string(), json!(value));
        writer
            .write(sequence_id, None, key.as_bytes())
            .expect("Error writing record.");
    }

    let mut file = fs::File::create(&output_json).unwrap();
    file.write_all(json!(dupe_headers).to_string().as_bytes());
    true
}

pub(crate) fn process<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    output_json: P,
    filter_by_header: bool,
) -> Result<(), Box<dyn Error>> {
    compress_duplicates(filename, output_json, filter_by_header);
    Ok(())
}

