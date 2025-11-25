#!/bin/bash 


cargo build
# cargo test
./target/debug/pscan -rci ./src/text_files "mary" -A 5 -B 3 


