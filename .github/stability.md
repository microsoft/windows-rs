## Stability

In short: `windows-rs` is still under active development, and none of the crates
under the org are considered "stable". Some crates such as `windows-sys` might
be closer than others, but until a crate reaches `1.0.0` it may be subject to
change.

## Release schedule and versioning

We're set to release new versions of crates every 3 weeks on Tuesdays. Currently all crates
are pre-1.0 and have their semver-minor version bumped on release. We are
looking for ways to version and release crates individually. Once we do, we will
update this section with our versioning strategy.

## Metadata Stability

All crates in `windows-rs` interact with [Windows Metadata (`.winmd`)
files](https://github.com/microsoft/win32metadata) which describe the various
windows APIs. All Windows APIs can be split into two categories:
`win32` APIs and `WinRT/COM` APIs. The way we bind to these APIs is different,
but in the metadata they also have different stability guarantees:

| Metadata category | Stable? |
|-------------------|---------|
| `Win32`           | ❌       |
| `WinRT/COM`       | ✅       |

## Crate Stability

The `windows-rs` project consists of different crates, which interact with
different parts of the Windows APIs. The stability of a crate is determined by
two factors which both must be stable for the crate to be stable:

1. The stability of the underlying schema we're generating bindings for.
2. The stability of the code we generate for the schema (the "projection").

| Crate name            | `Win32` support? | `WinRT/COM` support? | Projection Stable?  | Crate Stable? |
|-----------------------|------------------|----------------------|---------------------|---------------|
| **`windows`**         | ✅                | ✅                    | ❌                   | ❌             |
| **`windows-sys`**     | ✅                | ❌                    | ❌ (but we're close) | ❌             |
| **`windows-bindgen`** | ✅                | ✅                    | ❌                   | ❌             |

Because of the scope of the project (`230.000+` unique types), marking anything
as "stable" is a difficult task. Despite that, it is our explicit intent to
eventually achieve that. Over time we hope to gradually mark more APIs as
stable, propose concrete plans how to get to `1.0.0` stable releases for all
crates, and ensure that any breakage before then is well-communicated to reduce
friction.

## Minimum Supported Rust Version (MSRV)

The crates in `windows-rs` are tested to work on the latest _stable_ version of
Rust. We may introduce MSRV requirements once individual crates reach 1.0
versions. But right now we're expecting to be able to significantly improve our
APIs based on features expected to land in upcoming versions of Rust, and
we do not want to commit to stability before those features land. So for the
time being please assume that when upgrading `windows-rs` versions, you'll also
need to upgrade to the latest stable compiler.
