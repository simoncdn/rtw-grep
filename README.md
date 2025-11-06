# rtw-grep

> [!NOTE]
> **RTW Series (Reinvent The Wheel)**
> This project is part of a series called RTW - Reinvent The Wheel. The main goal of this series is not to create alternatives to existing tools, but to deeply understand how these tools work by reimplementing them. Each project suffixed with `rtw-` is a technical and educational exploration.

## Description

`rtw-grep` is a simplified reimplementation of the Unix `grep` tool in Rust. It allows searching for text patterns in files and displays matching lines with colored highlighting.

## What is grep?

`grep` (Global Regular Expression Print) is a command-line utility used to search for text strings in files. It scans the content of one or more files line by line and displays all lines containing a specified pattern.

### Classic Workflow

1. **File Reading**: The program reads the target file line by line
2. **Pattern Matching**: Each line is compared against the search pattern
3. **Filtering**: Only lines containing the pattern are kept
4. **Display**: Matching lines are printed to standard output

## Implemented Features

- Text search in a file
- Case-sensitive search (default)
- Case-insensitive search (via environment variable)
- Colored highlighting of the searched text (red and bold)

## Usage

### Basic search (case-sensitive)
```bash
cargo run <pattern> <file_path>
```

Example:
```bash
cargo run "hello" example.txt
```

### Case-insensitive search
```bash
RTW_GREP_CASE_INSENSITIVE= cargo run <pattern> <file_path>
```

Example:
```bash
RTW_GREP_CASE_INSENSITIVE= cargo run "Hello" example.txt
```

## Code Structure

- `src/main.rs`: Application entry point
- `src/lib.rs`: Core search logic
- `src/config.rs`: Configuration and argument handling

## Tests

Run the tests:
```bash
cargo test
```
