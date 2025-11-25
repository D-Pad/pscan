#!/bin/bash 


cargo build
# cargo test
./target/debug/pscan -ri ./src/text_files "mary" -A 4 -B 3 


