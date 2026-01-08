#!/bin/bash 


cargo build
# cargo test
./target/debug/pscan -E sub_dir rs -r ./src/text_files "mary" -A 5 -B 3 


