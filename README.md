# pnum

Elegantly count the number of files by extension in a directory. Fast, parallel, and friendly output for humans or JSON for machines.

## Features

- Count files grouped by extension in a directory.
- Optional recursive scanning of subdirectories.
- Include or exclude specific extensions.
- Sort by name or count, with reverse ordering.
- Limit output to the top N extensions.
- Quiet mode hides the progress spinner.
- JSON output mode for programmatic consumption.
- Control thread count for scanning.

## Quick Start

Build and run locally:

```bash
# Build
cargo build --release

# Run (binary at target/release/pnum)
target/release/pnum .

# Or use cargo run
cargo run -- .
```

or

```bash
# Install from source
cargo install --git https://github.com/itgrape/pnum.git
```

## Usage

```text
pnum [PATH] [OPTIONS]

Arguments:
  PATH                       Path to scan (default: ".")

Options:
  -s, --sort-by <name|count> Sort output by extension name or count
  -r, --reverse              Reverse the output order
  -R, --recursive            Scan subdirectories recursively
  -d, --detail               Show per-directory stats (not implemented yet)
  -q, --quiet                Hide the progress spinner
      --ignore-hidden        Ignore hidden files and directories
  -i, --include <EXT>        Include only these extensions (repeatable)
  -x, --exclude <EXT>        Exclude these extensions (repeatable)
  -n, --top <N>              Show only the top N extensions
      --json                 Output results in JSON format
  -t, --thread-num <N>       Threads to use for scanning (default: 1)
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples

```bash
# Basic: count extensions in current directory
pnum

# Recurse into subdirectories and ignore hidden files
pnum -R --ignore-hidden

# Sort by count, reverse order, and show top 5
pnum -s count -r -n 5

# Only include Rust and TOML files
pnum -i rs -i toml

# Exclude log and tmp files
pnum -x log -x tmp

# JSON output for scripts or tooling
pnum --json

# Use multiple threads (e.g., 4)
pnum -t 4
```

### Sample Output (Human)

```text
File extension counts for: .
.rs        : 42
.toml      : 3
.md        : 2
```

### Sample Output (JSON)

```json
{
  "rs": 42,
  "toml": 3,
  "md": 2
}
```

Notes:
- Extensions are normalized by removing a leading dot and lowercasing.
- When `include` is empty, all extensions are considered. `exclude` always filters afterward.

## Performance

- Uses `rayon` to parallelize processing across files.
- Uses `ignore::WalkBuilder` for efficient filesystem traversal and hidden-file handling.
- You can tune throughput with `--thread-num`.

## Status

- `--detail` is currently parsed but not implemented; the CLI warns when used.

## License

Licensed under the Apache License, Version 2.0. See `LICENSE` for details.