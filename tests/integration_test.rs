use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::NamedTempFile;

fn seqdupes() -> Command {
    #[allow(deprecated)]
    Command::cargo_bin("seqdupes").unwrap()
}

#[test]
fn test_basic_dedup() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();

    seqdupes()
        .args(["-f", "tests/fixtures/simple.fasta", "-j", json_path])
        .assert()
        .success()
        .stdout(predicate::str::contains(">"));

    let json_content = fs::read_to_string(json_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    assert_eq!(json.as_object().unwrap().len(), 3);
}

#[test]
fn test_by_header_flag() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();

    seqdupes()
        .args([
            "-f",
            "tests/fixtures/header_dupes.fasta",
            "-j",
            json_path,
            "--by-header",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(">"));

    let json_content = fs::read_to_string(json_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    assert_eq!(json.as_object().unwrap().len(), 2);
}

#[test]
fn test_missing_json_arg() {
    seqdupes()
        .args(["-f", "tests/fixtures/simple.fasta"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_nonexistent_file() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();

    seqdupes()
        .args(["-f", "tests/fixtures/nonexistent.fasta", "-j", json_path])
        .assert()
        .failure();
}

#[test]
fn test_empty_file() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();

    seqdupes()
        .args(["-f", "tests/fixtures/empty.fasta", "-j", json_path])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn test_all_duplicates() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();

    seqdupes()
        .args(["-f", "tests/fixtures/all_dupes.fasta", "-j", json_path])
        .assert()
        .success();

    let json_content = fs::read_to_string(json_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    // All 3 sequences are the same, so only 1 unique
    assert_eq!(json.as_object().unwrap().len(), 1);
}

#[test]
fn test_gzip_input() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();

    seqdupes()
        .args(["-f", "tests/fixtures/simple.fasta.gz", "-j", json_path])
        .assert()
        .success()
        .stdout(predicate::str::contains(">"));

    let json_content = fs::read_to_string(json_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    assert_eq!(json.as_object().unwrap().len(), 3);
}

#[test]
fn test_fastq_input() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();

    seqdupes()
        .args(["-f", "tests/fixtures/simple.fastq", "-j", json_path])
        .assert()
        .success()
        .stdout(predicate::str::contains("@"));

    let json_content = fs::read_to_string(json_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    assert_eq!(json.as_object().unwrap().len(), 2);
}

#[test]
fn test_stdin_input() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();
    let input = fs::read_to_string("tests/fixtures/simple.fasta").unwrap();

    seqdupes()
        .args(["-f", "-", "-j", json_path])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::str::contains(">"));

    let json_content = fs::read_to_string(json_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
    assert_eq!(json.as_object().unwrap().len(), 3);
}

#[test]
fn test_stderr_stats() {
    let json_out = NamedTempFile::new().unwrap();
    let json_path = json_out.path().to_str().unwrap();

    seqdupes()
        .args(["-f", "tests/fixtures/simple.fasta", "-j", json_path])
        .assert()
        .success()
        .stderr(predicate::str::contains(
            "Processed 4 sequences: 3 unique, 1 duplicates removed",
        ));
}
