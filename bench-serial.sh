#!/bin/bash

CSV_PATH=$1

set -e
set -x

just bench-run-cli $CSV_PATH 
just bench-run-cli-python $CSV_PATH 
just bench-run-cli-nodejs $CSV_PATH 

cd ../csv-stats-polars-python
/usr/bin/time -l -h -p uv run main.py -c "Amount Received" -f $CSV_PATH 

cd ../csv-stats-polars-rust
/usr/bin/time -l -h -p ./target/release/csv-stats-polars-rust --file-path $CSV_PATH 

cd ../univ-csv-stats
just bench-run-cli-python-native $CSV_PATH 
just bench-run-cli-nodejs-native $CSV_PATH 

