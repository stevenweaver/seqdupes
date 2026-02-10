use bio::io::fasta;
use bio::io::fastq;
use flate2::read::GzDecoder;
use rustc_hash::FxHashMap;
use serde_json::{json, Map, Value};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::str;

#[derive(Clone, Copy, PartialEq)]
enum Format {
    Fasta,
    Fastq,
}

fn detect_format(reader: &mut BufReader<Box<dyn Read>>) -> Result<Format, Box<dyn Error>> {
    let buf = reader.fill_buf()?;
    match buf.first() {
        Some(b'>') => Ok(Format::Fasta),
        Some(b'@') => Ok(Format::Fastq),
        Some(_) => Err("Unrecognized file format: expected '>' (FASTA) or '@' (FASTQ)".into()),
        None => Ok(Format::Fasta),
    }
}

fn open_input(filename: &str) -> Result<BufReader<Box<dyn Read>>, Box<dyn Error>> {
    if filename == "-" {
        Ok(BufReader::new(Box::new(io::stdin()) as Box<dyn Read>))
    } else if filename.ends_with(".gz") {
        let file = File::open(filename)?;
        Ok(BufReader::new(
            Box::new(GzDecoder::new(file)) as Box<dyn Read>
        ))
    } else {
        let file = File::open(filename)?;
        Ok(BufReader::new(Box::new(file) as Box<dyn Read>))
    }
}

struct ProcessResult {
    total: usize,
    unique: usize,
    dupe_headers: Map<String, Value>,
}

fn process_fasta(
    reader: BufReader<Box<dyn Read>>,
    filter_by_header: bool,
    writer: &mut dyn Write,
) -> Result<ProcessResult, Box<dyn Error>> {
    let records = fasta::Reader::new(reader).records();
    let mut seq_duplicates: FxHashMap<String, (String, Vec<String>)> = FxHashMap::default();
    let mut fasta_writer = fasta::Writer::new(writer);
    let mut total = 0;

    for record in records {
        let seqrec = record?;
        total += 1;
        let entire_header = format!("{} {}", seqrec.id(), seqrec.desc().unwrap_or(""))
            .trim()
            .to_string();
        let seq_str = str::from_utf8(seqrec.seq())?.to_owned();
        let key = if filter_by_header {
            entire_header.clone()
        } else {
            seq_str.clone()
        };

        seq_duplicates
            .entry(key)
            .and_modify(|(_seq, headers)| headers.push(entire_header.clone()))
            .or_insert_with(|| (seq_str, vec![entire_header]));
    }

    let mut dupe_headers = Map::new();
    let unique = seq_duplicates.len();
    for (seq, headers) in seq_duplicates.values() {
        let sequence_id = headers.last().unwrap();
        dupe_headers.insert(sequence_id.to_string(), json!(headers));
        fasta_writer.write(sequence_id, None, seq.as_bytes())?;
    }

    Ok(ProcessResult {
        total,
        unique,
        dupe_headers,
    })
}

fn process_fastq(
    reader: BufReader<Box<dyn Read>>,
    filter_by_header: bool,
    writer: &mut dyn Write,
) -> Result<ProcessResult, Box<dyn Error>> {
    let records = fastq::Reader::new(reader).records();
    let mut seq_duplicates: FxHashMap<String, (String, Vec<u8>, Vec<String>)> =
        FxHashMap::default();
    let mut fastq_writer = fastq::Writer::new(writer);
    let mut total = 0;

    for record in records {
        let seqrec = record?;
        total += 1;
        let entire_header = format!("{} {}", seqrec.id(), seqrec.desc().unwrap_or(""))
            .trim()
            .to_string();
        let seq_str = str::from_utf8(seqrec.seq())?.to_owned();
        let qual = seqrec.qual().to_vec();
        let key = if filter_by_header {
            entire_header.clone()
        } else {
            seq_str.clone()
        };

        seq_duplicates
            .entry(key)
            .and_modify(|(_seq, _qual, headers)| headers.push(entire_header.clone()))
            .or_insert_with(|| (seq_str, qual, vec![entire_header]));
    }

    let mut dupe_headers = Map::new();
    let unique = seq_duplicates.len();
    for (seq, qual, headers) in seq_duplicates.values() {
        let sequence_id = headers.last().unwrap();
        dupe_headers.insert(sequence_id.to_string(), json!(headers));
        fastq_writer.write(sequence_id, None, seq.as_bytes(), qual)?;
    }

    Ok(ProcessResult {
        total,
        unique,
        dupe_headers,
    })
}

pub(crate) fn process(
    filename: &str,
    output_json: &str,
    filter_by_header: bool,
    writer: &mut dyn Write,
) -> Result<(), Box<dyn Error>> {
    let mut reader = open_input(filename)?;
    let format = detect_format(&mut reader)?;

    let result = match format {
        Format::Fasta => process_fasta(reader, filter_by_header, writer)?,
        Format::Fastq => process_fastq(reader, filter_by_header, writer)?,
    };

    let mut json_file = File::create(output_json)?;
    json_file.write_all(json!(result.dupe_headers).to_string().as_bytes())?;

    let duplicates = result.total - result.unique;
    eprintln!(
        "Processed {} sequences: {} unique, {} duplicates removed",
        result.total, result.unique, duplicates
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn fixture(name: &str) -> String {
        format!("tests/fixtures/{}", name)
    }

    fn run_process(fasta: &str, by_header: bool) -> (String, Value) {
        let json_file = tempfile::NamedTempFile::new().unwrap();
        let json_path = json_file.path().to_str().unwrap().to_string();
        let mut output = Vec::new();
        process(&fixture(fasta), &json_path, by_header, &mut output).unwrap();
        let fasta_output = String::from_utf8(output).unwrap();
        let json_content = std::fs::read_to_string(&json_path).unwrap();
        let json_value: Value = serde_json::from_str(&json_content).unwrap();
        (fasta_output, json_value)
    }

    fn count_records(fasta_str: &str) -> usize {
        fasta_str.lines().filter(|l| l.starts_with('>')).count()
    }

    #[test]
    fn test_dedup_by_sequence() {
        let (fasta_out, json) = run_process("simple.fasta", false);
        assert_eq!(count_records(&fasta_out), 3);
        assert_eq!(json.as_object().unwrap().len(), 3);
    }

    #[test]
    fn test_dedup_by_header() {
        let (fasta_out, json) = run_process("header_dupes.fasta", true);
        assert_eq!(count_records(&fasta_out), 2);
        assert_eq!(json.as_object().unwrap().len(), 2);
    }

    #[test]
    fn test_by_header_preserves_sequence() {
        let (fasta_out, _json) = run_process("header_dupes.fasta", true);
        for line in fasta_out.lines() {
            if !line.starts_with('>') && !line.is_empty() {
                assert!(
                    line.chars().all(|c| "ACGTNacgtn".contains(c)),
                    "Sequence line contains non-nucleotide characters: {}",
                    line
                );
            }
        }
    }

    #[test]
    fn test_no_duplicates() {
        let (fasta_out, json) = run_process("no_dupes.fasta", false);
        assert_eq!(count_records(&fasta_out), 3);
        assert_eq!(json.as_object().unwrap().len(), 3);
    }

    #[test]
    fn test_all_duplicates() {
        let (fasta_out, json) = run_process("all_dupes.fasta", false);
        assert_eq!(count_records(&fasta_out), 1);
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        let headers = obj.values().next().unwrap().as_array().unwrap();
        assert_eq!(headers.len(), 3);
    }

    #[test]
    fn test_empty_file() {
        let (fasta_out, json) = run_process("empty.fasta", false);
        assert_eq!(count_records(&fasta_out), 0);
        assert_eq!(json.as_object().unwrap().len(), 0);
    }

    #[test]
    fn test_json_format() {
        let (_fasta_out, json) = run_process("simple.fasta", false);
        for (_key, value) in json.as_object().unwrap() {
            assert!(value.is_array());
            for item in value.as_array().unwrap() {
                assert!(item.is_string());
            }
        }
    }

    #[test]
    fn test_gzip_input() {
        let (fasta_out, json) = run_process("simple.fasta.gz", false);
        assert_eq!(count_records(&fasta_out), 3);
        assert_eq!(json.as_object().unwrap().len(), 3);
    }

    #[test]
    fn test_fastq_dedup() {
        let json_file = tempfile::NamedTempFile::new().unwrap();
        let json_path = json_file.path().to_str().unwrap().to_string();
        let mut output = Vec::new();
        process(&fixture("simple.fastq"), &json_path, false, &mut output).unwrap();
        let fastq_output = String::from_utf8(output).unwrap();
        let json_content = std::fs::read_to_string(&json_path).unwrap();
        let json_value: Value = serde_json::from_str(&json_content).unwrap();
        // simple.fastq has 3 seqs, 2 share the same sequence -> 2 unique
        let record_count = fastq_output.lines().filter(|l| l.starts_with('@')).count();
        assert_eq!(record_count, 2);
        assert_eq!(json_value.as_object().unwrap().len(), 2);
    }
}
