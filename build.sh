#!/bin/bash 


cargo build
# cargo test
./target/debug/pscan -rsi ./src/text_files "user" -A 4 -B 3 


