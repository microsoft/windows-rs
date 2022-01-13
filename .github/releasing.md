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
6. Generate new documentation for `windows` locally
7. Push the generated `windows` docs to GitHub pages
