# Windows SpellChecker

A sample showing how to generate and use the Windows spell-checking API
(`ISpellCheckerFactory`, `ISpellChecker`, …) directly from the in-house Win32
metadata with [`windows-bindgen`](https://crates.io/crates/windows-bindgen), rather
than depending on the prebuilt `windows` crate. The `build.rs` filters the handful of
COM types the app needs; `main.rs` includes the generated bindings and checks a line of
text from the command line, printing corrections to standard out.

## Running

Make sure you have [Rust installed](https://rustup.rs), and then run the following:

```powershell
cargo run -- "This is text with a word missspelled!"
```
