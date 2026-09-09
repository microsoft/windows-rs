## windows-pickers

Windows Pickers provides file and folder pickers backed by the Windows Common Item Dialog. It
supports standalone Windows applications and components built with `windows-reactor`.

* [Getting
  started](https://github.com/microsoft/windows-rs/blob/master/docs/crates/windows-pickers.md)

Start by adding the following to your Cargo.toml file:

```toml
[dependencies]
windows-pickers = "0.100"
windows-window = "0.100"
```

```rust,no_run
# #[cfg(feature = "system")]
# mod example {
use windows_pickers::{OpenFilePicker, Result};
use windows_window::Window;

fn pick_rust_file(window: &Window) -> Result<()> {
    let path = OpenFilePicker::new()
        .title("Open a Rust source file")
        .filter_extensions("Rust source", ["rs"])
        .filter_all()
        .show(window)?;

    if let Some(path) = path {
        println!("{}", path.display());
    }

    Ok(())
}
# }
```

The calling thread must be a COM single-threaded apartment (STA). Cancellation returns `Ok(None)`;
other Windows failures are returned as errors.
