# Windows SpellChecker

A sample showing how to use the Windows spell-checking API (`ISpellCheckerFactory`, `ISpellChecker`)
from the `windows` crate. It checks a line of text from the command line and prints corrections to
standard out.

## Running

Make sure you have [Rust installed](https://rustup.rs), and then run the following:

```powershell
cargo run -- "This is text with a word missspelled!"
```
