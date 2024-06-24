# seqdupes

Removes duplicates from FASTA files. Supports filtering based on sequence content or header information.

## Installation

### Source

Download the source code and run:

```bash
cargo install
```

## Usage

Run `seqdupes` to process FASTA files. You can specify whether to filter by sequence or by header.

### Filtering by Sequence (default)

```bash
seqdupes -f path/to/sequence.fastq -j path/to/output.json > no_dupes.fas
```

### Filtering by Header

If you prefer to filter duplicates based on headers rather than sequences, use the `--by-header` flag.

```bash
seqdupes -f path/to/sequence.fastq -j path/to/output.json --by-header > no_dupes.fas
```

### Arguments

| Parameter      | Default | Description                                 |
|----------------|---------|---------------------------------------------|
| -f, --fasta    | -       | The path to the FASTQ file to use.          |
| -j, --json     | -       | The output path for listing duplicates.     |
| -b, --by-header| -       | Enables filtering based on headers (optional). |

The tool outputs a FASTA file with duplicates removed to `stdout` and a JSON file containing details of the duplicates to the specified path.
```
