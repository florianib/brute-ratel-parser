# BruteRatelParser
This project creates an XLSX (Excel) file from the provided Brute Ratel logs.

# Build 
```
cargo build --release
```

# Usage
Put all the logs into one directory and provide the path to the `--path` argument.
If you ommit the flags `--no-combine` and `--no-generate`, the tool will combine and generate the excel file. 
```
Usage: brute_ratel_parser.exe [OPTIONS] --path <PATH>

Options:
      --no-combine   
  -p, --path <PATH>  
      --no-generate  
  -h, --help         Print help
  -V, --version      Print version
```


# Example
```
brute_ratel_parser.exe -p C:\temp\logs
  Combining the log files
  Found 9 log files
  Combined the logs
  Generating excel file
  Created excel file
  Done
```
