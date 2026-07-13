# Win32 metadata snapshot

This directory holds the **canonical RDL snapshot** of the Win32 API surface that
[`windows-clang`](../../crates/libs/clang/readme.md) scrapes from the Windows SDK
headers. The whole corpus lives in a **single flat `Windows.Win32` namespace**, split
**one file per defining header** — e.g. `wingdi.rdl` holds exactly the declarations
`wingdi.h` defines — written in the human-readable
[RDL](../../crates/libs/rdl/readme.md) text format. The partitioning follows the
source (the clang cursor's defining header), not an editorial namespace taxonomy.

The merged binary `Windows.Win32.winmd` that downstream tooling consumes is *not*
checked in (winmd is a binary format, and `*.winmd` is git-ignored). This text
snapshot is committed instead so the API surface can be browsed directly and so
changes show up as readable diffs in pull requests — the same reason the generated
`windows` and `windows-sys` crates are committed.

## Generated — do not edit by hand

These files are produced by `tool_win32`:

```sh
cargo run -p tool_win32
```

Re-run the tool after changing the manifest (the `const` slices in
`crates/tools/win32/src/main.rs`), the hand-authored vocabulary seed
(`metadata/win32/metadata.rdl`), or the scraper,
and commit the resulting diff. Hand edits will be overwritten on the next run — the
tool clears the generated partitions (preserving only the seed) and re-emits the
whole closure.

See [`docs/crates/windows-clang.md`](../../docs/crates/windows-clang.md) for the
scraper design — the faithful-metadata principle, header partitioning, and the
editorial-deviation ledger — and [`docs/crates/windows-rdl.md`](../../docs/crates/windows-rdl.md)
for the winmd artifact and layout of the in-house metadata pipeline that replaces the
external `win32metadata` project.
