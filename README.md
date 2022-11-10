Removes duplicates from FASTA file

## Installation

#### Source

Download the source code and run

    cargo install

## Usage

```
seqdupes -f path/to/sequence.fastq -j path/to/output.json > no_dupes.fas
```

Arguments: 

| Parameter                 | Default       | Description   |	
| :------------------------ |:-------------:| :-------------|
| -f --fasta         |	-           |The path to the FASTQ file to use
| -j --json         |	-           |The output path to list duplicates
