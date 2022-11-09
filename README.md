Parses fasta records based on list of either header names or regexes

## Installation

#### Source

Download the source code and run

    cargo install

## Usage

```
seqsplitter -f path/to/sequence.fastq -l path/to/list.txt
```

Arguments: 

| Parameter                 | Default       | Description   |	
| :------------------------ |:-------------:| :-------------|
| -f --fasta         |	-           |The path to the FASTQ file to use
| -l --list          |	-           |List of ids to parse out
| -r --regex         |	false       |Interpret list as regex
| -u --assume_unique |	false       |Assume list is unique and exit when all seqs in list are found
| -v --version       |	-           |Print version information
# seqdupes
# seqdupes
