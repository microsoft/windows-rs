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

Inspect the finalized metadata shape produced by RDL:

```text
riddle expand api.rdl
```

Validate existing metadata without converting it to RDL:

```text
riddle validate api.winmd
```

Format files in place, or check formatting in CI without changing them:

```text
riddle fmt api.rdl
riddle fmt --check api.rdl
```

Use `--reference dependency.winmd` for additional metadata references. The default Windows
metadata is used for resolution unless `--no-default` is specified. `-` reads RDL from standard
input.

## Implementation

The binary owns command-line parsing, terminal rendering, standard input, file updates, and
exit-code policy. Parsing, RDL validation, resolution, formatting, and metadata encoding remain in
`windows-rdl`. Shared metadata validation remains in `windows-metadata`. `Reader::check` runs the
same pipeline as `Reader::write` but does not create a winmd. `riddle check` uses
`Reader::check_all` to report independent errors from every input rather than stopping at the first
source or declaration. `riddle validate` reads existing winmds into one authored index and keeps
reference metadata in a separate lookup index.

`riddle expand` compiles RDL in memory and prints the finalized metadata types, inheritance,
implemented interfaces, layouts, fields, methods, signatures, raw flags, properties, events, and
custom-attribute identities. It is an inspection view rather than RDL source. Property and event
accessors therefore remain visible as methods alongside their association rows, and overloads show
both metadata and projected names.

RDL diagnostics use exit code 1. Invalid command lines use exit code 2. The renderer prints stable
diagnostic codes, source locations, labeled source lines, notes, help, and a final error count when
more than one error is found. Formatting reads and validates every input before updating any file,
so one invalid input leaves the whole set unchanged.

Run the command tests with:

```text
cargo test -p riddle
```

## Validation test strategy

Shared validator changes should be covered at the lowest layer that can express the invalid state:

| Layer | Purpose |
|-------|---------|
| `test_metadata` synthetic rows | Malformed ECMA relationships and states that RDL cannot author |
| Committed metadata corpus | Check compatibility with committed Windows metadata |
| `test_rdl` validation | Test source-to-metadata validation |
| `riddle` command tests | Test exit codes, rendering, labels, and `check` behavior |
| Merge/remap and roundtrip tests | Guard conversion fidelity |

Every source-authorable validator rule should have an RDL or `riddle check` test in addition to its
metadata-row test. Metadata-only tests are appropriate only when the malformed state has no RDL
spelling. Corpus validation remains mandatory before treating a proposed common rule as valid.
The command tests also inspect `riddle build` output when a lowering detail matters, such as static
global-function calling conventions, rather than treating successful compilation as sufficient.
