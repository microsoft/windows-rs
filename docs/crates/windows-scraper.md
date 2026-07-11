# windows-scraper

> The shared header-scrape pipeline behind `tool_win32` and `tool_wdk`.

- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/scraper)

`windows-scraper` is an internal (unpublished) library that owns the mechanical
pipeline for turning C/C++ headers into committed RDL + winmd metadata. It sits on top
of [`windows-clang`](windows-clang.md) (headers → RDL) and [`windows-rdl`](windows-rdl.md)
(RDL → winmd), and factors out the orchestration that every whole-header scraper needs
so each individual tool is reduced to *toolchain provisioning + a manifest*.

A scraper resolves its own pinned toolchain (which NuGet packages and versions, which
include/lib directories, per-arch preprocessor defines), fills in a `Config`, and calls
`run`. Everything mechanical after that is shared:

- provision the pinned libclang,
- build one translation unit per target architecture and scrape it with `write_by_header`
  (each declaration routed to its defining-header partition),
- arch-merge the per-arch RDL sets so subset-of-arch symbols get `SupportedArchitecture`
  tags (x64 is canonical; extra arches write throwaway partitions),
- re-derive one unified winmd from the merged corpus.

The committed RDL under `metadata/<tool>/` is the reviewable source of truth; the binary
winmd is git-ignored and rebuilt from it.

## Config

`Config` is the tool-agnostic execution contract — a bundle of fully-resolved paths and
lists:

- **Output:** `root` (metadata namespace), `rdl_dir` (committed RDL), `out_dir` (scratch),
  `winmd`.
- **Targets:** `archs` (name/triple/bits/defines; `archs[0]` is canonical), `parallel`.
- **Compile:** `clang_args`, `force_includes`, `include_shim_dirs`, `include_args`,
  `import_libs`.
- **Scope:** `drop_lib_less`, `scope`, `scope_headers`, `sources` (translation-unit text),
  `reference_winmds` (passed both to the scrape for exclusion and to the reader for
  resolution), `seed` (optional metadata RDL seed).

`run(&Config)` clears the RDL dir, scrapes every arch (optionally in parallel via
`windows-threading`), arch-merges, and writes the final winmd, returning a `Summary`.

## Worked examples

`tool_win32` and `tool_wdk` are the two reference scrapers, and they are deliberately the
same engine with different manifests:

- **`tool_win32`** — a **base** scrape (no reference winmd). Reads `win32.toml`, resolves
  the Windows SDK toolchain, and produces `Windows.Win32.winmd`.
- **`tool_wdk`** — an **additive** scrape. Reads `wdk.toml` and passes the Win32 winmd as a
  `reference_winmd`, so it emits only the surface the WDK adds on top and resolves shared
  types by bare name. Produces `Windows.Wdk.winmd`.

A third-party scraper follows the same shape: resolve its own toolchain, fill a `Config`,
and `run`. The design principles behind the pipeline — faithful metadata, header
partitioning, SAL/annotation handling, the multi-arch merge, and the canonical type remaps
— are documented in [`windows-clang`](windows-clang.md).
