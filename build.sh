#!/bin/bash 


cargo build
# cargo test
./target/debug/pscan -is ./src/text_files/combined_texts.md "wamboozle" -A 4 -B 3 


