#!/bin/bash 


cargo build
# cargo test
./target/debug/pscan -r ./src/text_files "Mary"

