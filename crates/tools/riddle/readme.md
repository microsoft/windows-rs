## Generate Rust bindings for Windows

Update: the `riddle` tool is no longer being updated. You can continue to use the [windows-bindgen](https://crates.io/crates/windows-bindgen) library crate directly.

The [riddle](https://crates.io/crates/riddle) tool automatically generates Rust bindings from Windows metadata.

* [Getting started](https://kennykerr.ca/rust-getting-started/)
* [Samples](https://github.com/microsoft/windows-rs/tree/0.58.0/crates/samples)
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
