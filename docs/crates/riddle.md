# Riddle

The `riddle` binary checks and compiles RDL API descriptions. Install it from the workspace with:

```text
cargo install --path crates/tools/riddle
```

Check RDL without creating metadata:

```text
riddle check api.rdl
```

Compile one or more files or directories into a winmd:

```text
riddle build api.rdl shared-rdl --out api.winmd
```

Use `--reference dependency.winmd` for additional metadata references. The default Windows
metadata is used for resolution unless `--no-default` is specified. `-` reads RDL from standard
input.

## Implementation

The binary owns command-line parsing, terminal rendering, standard input, and exit-code policy.
Parsing, validation, resolution, and metadata encoding remain in `windows-rdl`. `Reader::check`
runs the same pipeline as `Reader::write` but does not create a winmd.

RDL diagnostics use exit code 1. Invalid command lines use exit code 2. The renderer prints stable
diagnostic codes, source locations, labeled source lines, notes, and help.

Run the command tests with:

```text
cargo test -p riddle
```
