# windows-pickers

> File and folder pickers backed by the Windows Common Item Dialog.

- 📦 [crates.io](https://crates.io/crates/windows-pickers)
- 📖 [docs.rs](https://docs.rs/windows-pickers)
- 🚀 [Getting started](../../crates/libs/pickers/readme.md)
- 🧩 [Samples](https://github.com/microsoft/windows-rs/tree/master/crates/samples/pickers)
- 📁 [Source](https://github.com/microsoft/windows-rs/tree/master/crates/libs/pickers)
- [Reactor guide](windows-reactor.md)
- [Window guide](windows-window.md)

## When to use it

Use `windows-pickers` when a Windows desktop application needs the system file-open, folder, or
file-save dialog. The crate returns filesystem `PathBuf` values and uses the Windows Shell for
navigation, filtering, recent locations, and overwrite confirmation.

The crate wraps the common picker workflows rather than every `IFileDialog` extension point. Use
raw Windows bindings when an application needs custom dialog controls, dialog events,
non-filesystem Shell items, or other advanced Common Item Dialog APIs.

## The basic idea

Choose a picker, configure it with consuming builder methods, and show or request it:

| Type | Result |
| --- | --- |
| `OpenFilePicker` | One file, or several files through `show_multiple` |
| `FolderPicker` | One folder, or several folders through `show_multiple` |
| `SaveFilePicker` | A filesystem path at which to save a file |

Showing a picker is synchronous. `IFileDialog::Show` runs a nested modal message loop and returns
after the user confirms or cancels the dialog. The crate does not read, create, or modify the
selected file.

## Choose how to host the picker

The default `system` feature accepts a borrowed `windows_window::Window`:

```toml
[dependencies]
windows-core = "0.100"
windows-pickers = "0.100"
windows-window = "0.100"
```

Applications that own an HWND through another windowing library can disable default features and
use `show_for_hwnd`:

```toml
windows-pickers = { version = "0.100", default-features = false }
```

Enable `reactor` to request a picker from a Reactor component:

```toml
windows-pickers = { version = "0.100", default-features = false, features = ["reactor"] }
windows-reactor = "0.100"
```

The `system` and `reactor` features are additive. An application may enable both when it uses both
host types.

## Pick your first file

This complete program initializes COM, creates an owner window, and shows an open-file picker:

```rust,no_run
use windows_pickers::{OpenFilePicker, Result};
use windows_window::Window;

fn main() -> Result<()> {
    windows_core::init_mta()?;
    let window = Window::new("Open a file").create()?;

    let path = OpenFilePicker::new()
        .title("Open a Rust source file")
        .filter_extensions("Rust source", ["rs"])
        .filter_all()
        .show(&window)?;

    if let Some(path) = path {
        println!("{}", path.display());
    }

    Ok(())
}
```

The calling thread must be initialized for COM and suitable for modal UI. An application with an
existing UI framework normally uses that framework's UI apartment rather than initializing COM
again. Keep the owner window alive until `show` returns.

## Pick files and folders

`OpenFilePicker::show` and `FolderPicker::show` return one selected path:

```rust,ignore
let file = OpenFilePicker::new().show(&window)?;
let folder = FolderPicker::new().show(&window)?;
```

Use `show_multiple` when the user may select more than one item:

```rust,ignore
let files = OpenFilePicker::new()
    .title("Attach files")
    .commit_label("Attach")
    .filter_all()
    .show_multiple(&window)?;

let folders = FolderPicker::new()
    .title("Add search folders")
    .show_multiple(&window)?;
```

`commit_label` changes the confirmation button for a task-specific action such as "Attach",
"Import", or "Choose".

## Choose a save path

`SaveFilePicker` configures the proposed file name, extension, filters, and overwrite behavior:

```rust,ignore
let path = SaveFilePicker::new()
    .title("Save report")
    .filter_extensions("Text", ["txt"])
    .suggested_name("report")
    .default_extension("txt")
    .overwrite_prompt(true)
    .show(&window)?;
```

`suggested_name` sets the initial name. `default_extension` supplies the extension when the user
enters a name without one; a leading period is optional. Configured filters restrict the accepted
file types. `overwrite_prompt` explicitly enables or disables replacement confirmation, while
leaving it unset preserves the native dialog default.

The returned path is only the user's choice. The application remains responsible for opening the
file and handling races, permissions, sharing violations, and any final overwrite policy.

## Configure file filters

Add extension filters directly to an open or save picker:

```rust,ignore
let picker = OpenFilePicker::new()
    .filter_extensions("Images", ["png", "jpg", "jpeg"])
    .filter_extensions("Portable Network Graphics", ["png"])
    .filter_all()
    .initial_filter(0);
```

Extensions may include a leading period. Use `filter_patterns` for native wildcard patterns:

```rust,ignore
let picker = OpenFilePicker::new()
    .filter_patterns("Reports", ["report-*.csv", "summary-*.csv"])
    .filter_all();
```

Each filter must have a nonempty name and at least one nonempty extension or pattern.

`FileFilter` remains available when a named filter should be built once and reused:

```rust,ignore
let source = FileFilter::extensions("Source", ["rs", "toml"]);
let open = OpenFilePicker::new().filter(source.clone()).filter_all();
let save = SaveFilePicker::new().filter(source);
```

`initial_filter` uses a zero-based index into the configured filters. An out-of-range index is
reported as an error before the dialog opens.

## Set the initial location

Each picker provides two path-based and two known-folder methods:

| Method | Behavior |
| --- | --- |
| `default_folder(path)` | Used when the Shell has no remembered location |
| `folder(path)` | Forces the initially displayed folder |
| `default_location(location)` | Default using a `PickerLocation` |
| `location(location)` | Forced location using a `PickerLocation` |

`PickerLocation` supports Desktop, Documents, Downloads, Music, Pictures, Videos, Computer, and
3D Objects. When both a default and forced location are configured, the default is applied first
and the forced location determines the initial view.

Path-based locations must resolve to Shell items. An invalid or unavailable configured path is
reported before the dialog opens.

## Preserve Shell state

Pass a stable GUID to `client_id` when the Shell should remember state such as the last visited
folder:

```rust,ignore
let picker = OpenFilePicker::new()
    .client_id(GUID::from_u128(0x6d81d46c_77da_4874_8ca7_754546ba6803));
```

Use the same GUID for picker uses that should share history and different GUIDs for independent
histories. Without a client ID, the crate does not opt the dialog into this persistence mechanism.

## Handle cancellation and errors

Single-selection pickers return `Result<Option<PathBuf>>`:

| Value | Meaning |
| --- | --- |
| `Ok(Some(path))` | The user selected a path |
| `Ok(None)` | The user cancelled |
| `Err(error)` | Configuration, COM, or Shell operation failed |

Multiple-selection methods return `Result<Vec<PathBuf>>`. An empty vector means cancellation
because the dialog cannot successfully confirm an empty selection. Only the native
`ERROR_CANCELLED` result is translated into cancellation; other failures remain
`windows_core::Error` values.

## Use a raw HWND

The raw-handle methods work with another Win32 host:

```rust,ignore
let path = OpenFilePicker::new().show_for_hwnd(hwnd)?;
```

The pointer must be a valid, non-null owner HWND on the calling UI thread and remain live until the
method returns. The calling thread must already be initialized for COM. The raw methods are
available without the `system` feature.

## Request a picker from Reactor

Picker methods cannot call `IFileDialog::Show` inline while Reactor is publishing component state.
The `reactor` feature adds `request` and `request_multiple`, which stage the modal operation and
map its result into a component message:

```rust,ignore
enum Message {
    Open,
    Picked(windows_pickers::Result<Option<PathBuf>>),
}

fn update(&mut self, message: Message, context: &ComponentContext<Self>) {
    match message {
        Message::Open => {
            if !OpenFilePicker::new()
                .filter_all()
                .request(context, Message::Picked)
            {
                self.status = "Another window operation is pending".to_string();
            }
        }
        Message::Picked(Ok(Some(path))) => {
            self.status = path.display().to_string();
        }
        Message::Picked(Ok(None)) => {
            self.status = "Selection cancelled".to_string();
        }
        Message::Picked(Err(error)) => {
            self.status = format!("Picker failed: {error}");
        }
    }
}
```

A `true` return means Reactor staged the request, not that the picker has already opened. Reactor
runs it on the owning window's UI thread after the current publication commits and queues the
mapped message for a later component update. It discards the operation if publication fails, the
component retires, or the window starts closing.

Each Reactor window accepts one pending native window operation at a time. Other Reactor windows
remain independent. No additional COM initialization is needed in a Reactor application.

## Samples

| Sample | What to study |
| --- | --- |
| [`standalone`](../../crates/samples/pickers/standalone) | All picker types with `windows-window` |
| [`reactor`](../../crates/samples/pickers/reactor) | All picker types through component messages |

Run an individual standalone workflow with:

```text
cargo run -p pickers-standalone -- open
cargo run -p pickers-standalone -- open-multiple
cargo run -p pickers-standalone -- folder
cargo run -p pickers-standalone -- folder-multiple
cargo run -p pickers-standalone -- save
```

Run the Reactor sample with:

```text
cargo run -p pickers-reactor
```

---

## Internal documentation

This section is for contributors to `windows-pickers`.

`src/bindings.rs` is generated by `tool-bindings` from
`crates/tools/bindings/src/pickers.txt`. It contains the minimal Common Item Dialog, Shell item,
COM activation, and string-allocation APIs used by the hand-written wrapper.

Open and folder pickers create `FileOpenDialog`; the save picker creates `FileSaveDialog`. All
preserve the native option defaults while adding `FOS_FORCEFILESYSTEM`. Folder selection adds
`FOS_PICKFOLDERS`, multiple selection adds `FOS_ALLOWMULTISELECT`, and save dialogs with filters
add `FOS_STRICTFILETYPES`.

Shared configuration retains UTF-16 backing storage until `Show` returns. Filesystem locations are
converted with `SHCreateItemFromParsingName`, known locations with
`SHCreateItemInKnownFolder`, and selected `IShellItem` values with `SIGDN_FILESYSPATH`. Returned
strings are copied into `PathBuf` values before `CoTaskMemFree`.

Client identity is applied before folder configuration so a forced folder still determines the
initial view. Requested option bits are merged with `GetOptions` rather than replacing unrelated
native defaults.

After changing the binding filter, run:

```text
cargo run -p tool-bindings --quiet
cargo check -p windows-pickers --quiet
```

Unit tests cover configuration and cancellation translation without opening modal Shell UI.
