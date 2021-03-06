#!/bin/bash

set -e

INPUT=testdata/four_reads.pe.fastq

AAA=$(./target/debug/fasten_kmer --kmer-length 3 < $INPUT | grep -m 1 AAA)

if [ "$AAA" != $'AAA\t31' ]; then
  echo "Chandler did not count kmers correctly"
  exit 1
fi

echo "Chandler passed!"
