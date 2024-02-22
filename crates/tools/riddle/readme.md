## Generate Rust bindings for Windows

The [riddle](https://crates.io/crates/riddle) tool automatically generates Rust bindings from Windows metadata.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/0.53.0/crates/samples) <!-- link to samples for upcoming release -->
* [Releases](https://github.com/microsoft/windows-rs/releases)

Start by installing `riddle`:

```
> cargo install riddle
```

Generates Rust bindings as needed:

```
> riddle
Usage: riddle.exe [options...]

Options:
  --in  <path>          Path to files and directories containing .winmd and .rdl files
  --out <path>          Path to .winmd, .rdl, or .rs file to generate
  --filter <namespace>  Namespaces to include or !exclude in output
  --config <key=value>  Override a configuration value
  --format              Format .rdl files only
  --etc <path>          File containing command line options
```
