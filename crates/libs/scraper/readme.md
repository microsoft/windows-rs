# windows-scraper

Shared orchestration for turning C/C++ headers into committed RDL + winmd metadata with
[`windows-clang`](../clang) and [`windows-rdl`](../rdl).

A scraper fills in a `Config` — headers, include/lib directories, reachability scope, target
architectures, and optional pieces like a metadata seed or a reference winmd — and calls `run`.
The crate owns the mechanical pipeline shared by every scraper: provision the pinned libclang,
parse each translation unit per architecture, route declarations to their defining-header
partitions, arch-merge the per-arch results, and re-derive one unified winmd from the merged
corpus.

`tool_win32` and `tool_wdk` are the two worked examples. `tool_win32` is a base scrape (no
reference winmd); `tool_wdk` is an *additive* scrape that passes the Win32 winmd as a reference
so it emits only the surface the WDK adds on top, resolving shared types by bare name. A
third-party scraper follows the same shape: resolve its own toolchain, fill a `Config`, and run.
