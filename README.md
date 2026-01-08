# Project Scan 
This script is a grep-like script written in Rust. A tool that I often use on 
large repo's when I can't remember which python file I left a stray `print()`
statement in.

**Example use:**
![Sample Output](resources/output.png)

Passing the `--help` flag will display the following menu
```
pscan - A grep-like tool with detailed match location and context lines

USAGE:
    pscan [OPTIONS] <PATH> <QUERY> [-I ext1 ext2 ...] [-E ext1 ext2 ...]

POSITIONAL ARGUMENTS:
    PATH                Directory or file to search (required)
    QUERY               Text or pattern to search for (required)

SEARCH OPTIONS:
    -i                  Perform case-insensitive matching (default is case-sensitive)
    -r                  Search directories recursively
    -s                  Show parsed arguments before starting search (useful for debugging)
    -t                  Trims white space from any matching lines
    -b                  Enables binary file reading.
    -c                  Only shows the number of matches found per file

CONTEXT CONTROL:
    -A, --after N       Print N lines of trailing context after each match
    -B, --before N      Print N lines of leading context before each match
    -C, --context N     Print N lines of context both before and after each match
                        (equivalent to -B N -A N)

FILE TYPE FILTERING:
    -I, --include-file-types ext1 ext2 ...
                        Only search files whose extension is in the list
                        (e.g. -I rs toml yaml)
    -E, --exclude-file-types ext1 ext2 ...
                        Skip files whose extension is in the list. Will also skip 
                        directories if the directory name is included here.
                        (e.g. -E jpg png gif node_modules)

EXAMPLES:
    pscan ./src "println!"
    pscan -i -r . "error handling"
    pscan -C 2 Cargo.toml "version"
    pscan -I rs toml -r src "unsafe"
    pscan --before 1 --after 3 logs "ERROR"

NOTE:
    Short options can be combined: -ris is equivalent to -r -i -s
    Extensions for -I/-E should be given without leading dot
```

