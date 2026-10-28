# windows-pickers

Windows file-open, folder, and file-save dialogs that return filesystem paths.
The application remains responsible for reading or writing selected files.

- 📦 [crates.io](https://crates.io/crates/windows-pickers)
- 📖 [docs.rs](https://docs.rs/windows-pickers)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/pickers)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/pickers)

## Pick a file

The default `system` feature accepts a `windows-window` owner:

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

The UI thread must use a COM STA. Follow an existing framework's apartment setup or retain a
`windows_core::init_sta()` guard. Keep the owner window alive until the synchronous modal call
returns.

## Configure a picker

| Picker | Result |
| --- | --- |
| `OpenFilePicker` | One file, or several through `show_multiple` |
| `FolderPicker` | One folder, or several through `show_multiple` |
| `SaveFilePicker` | A proposed save path |

Use `suggested_name` and `default_extension` for save names. Use `default_folder` when the Shell
has no remembered location, or `folder` to force the initial location. A stable `client_id` GUID
lets related pickers share Shell history. `initial_filter` is a zero-based index.

Single-selection methods return `Ok(None)` on cancellation; multiple-selection methods return an
empty vector. Other failures remain errors. A save result is only a path: opening it can still
fail or race with another process, even after overwrite confirmation.

## Use another host

For another HWND owner, disable default features and use `show_for_hwnd`. The handle must belong
to the calling UI thread and remain valid throughout the dialog.

For Reactor, enable `reactor` and use `request` or `request_multiple` to map the result into a
component message. Do not call modal `show` inline during an update. A request returning `false`
was not staged; each window accepts one pending native window operation at a time.

See the [standalone][standalone] or [Reactor][reactor] sample.

[standalone]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/pickers/standalone
[reactor]: https://github.com/microsoft/windows-rs/tree/master/crates/samples/pickers/reactor
