# RELEASE

## Validations

1. Check out latest master branch
2. `windows-sys` is checked on CI, no need to check that
3. Run `cargo check -p windows --all-features` locally
4. Clone the samples repo, manually build all of those.
5. Build the documentation locally for `windows`

## Releasing

1. Updating versions in a separate branch
3. Validate & merge PR
2. Create a new GitHub release, create a tag, and generate release notes
4. Cargo publish
5. Update all samples to point at new version of crate
6. Generate new documentation for `windows` locally (see "building documentation")
7. Push the generated `windows` docs to GitHub pages

## Building documentation

1. Clone the `windows-rs` docs repo
2. Delete all contents in the repo
3. In the `windows-rs` repo build the docs with the following command:

```powershell
C:\git\windows-rs> cargo doc -p windows --all-features --no-deps --target-dir d:\git\docs-rs\docs
```

## Publishing order

1. All crates under `crates/targets`
2. `crates/libs/quote` and `crates/libs/reader`
4. `crates/libs/bindgen` and `crates/libs/gen`
5. `crates/libs/macros`
6. `crates/libs/sys` and `crates/libs/windows`

## References

- [windows-rs docs repo](https://github.com/microsoft/windows-docs-rs)
