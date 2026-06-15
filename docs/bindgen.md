# `windows-bindgen`

`windows-bindgen` generates Rust bindings from Windows metadata (.winmd files).
It powers the `windows` and `windows-sys` crates and can generate standalone
bindings for any Windows API.

## Usage

### In a build script

```rust
// build.rs
fn main() {
    windows_bindgen::bindgen([
        "--out", "src/bindings.rs",
        "--flat",
        "--filter",
            "Windows.Win32.Graphics.Direct2D.ID2D1Factory1",
            "Windows.Win32.Graphics.Dxgi.IDXGIFactory2",
    ]).unwrap();
}
```

### With an args file

```rust
// build.rs
fn main() {
    windows_bindgen::bindgen(["--etc", "bindings.txt"]).unwrap();
}
```

```
// bindings.txt
--out src/bindings.rs
--flat

--filter
    Windows.Win32.UI.WindowsAndMessaging.CreateWindowExW
    Windows.Win32.UI.WindowsAndMessaging.DefWindowProcW
    Windows.Win32.UI.WindowsAndMessaging.RegisterClassW
```

### Key Options

| Option | Description |
|--------|-------------|
| `--in <path>` | Input .winmd file(s) (default: bundled Windows metadata) |
| `--out <path>` | Output file path |
| `--filter <types>` | Which types/methods to include |
| `--flat` | Emit all types in a single module (no namespace hierarchy) |
| `--sys` | Generate raw C-style bindings (like `windows-sys`) |
| `--minimal` | Emit only explicitly listed methods; others become `usize` vtable slots |
| `--deps {core,specific,none}` | Dependency mode for shared types (default: `none` with `--sys`, `core` otherwise) |
| `--derive <traits>` | Additional derives on generated types |
| `--rustfmt` | Format output with rustfmt |
| `--implement <ifaces>` | Generate implementation traits for COM interfaces |
| `--link` | Link macro source |
| `--package` | Generate a complete crate with Cargo.toml |

### Filter Syntax

The `--filter` accepts fully-qualified type names from Windows metadata:

| Form | Effect |
|------|--------|
| `Windows.Win32.Graphics.Direct2D.ID2D1Factory` | Include a type |
| `Windows.Win32.Graphics.Direct2D.ID2D1Factory::{CreateDrawingStateBlock}` | Include specific methods only (with `--minimal`) |
| `Windows.Foundation.TimeSpan` | Include a WinRT value type |

With `--minimal`, unlisted methods become `usize` vtable slots — the layout is
preserved but no code is generated for methods you don't call. Dependencies
(parameter/return types) are pulled in automatically.

---

## Design Notes

The user-facing CLI surface is small and orthogonal:

| Axis | Options |
|---|---|
| Input | `--in` |
| Output path / layout | `--out`, `--flat`, `--package` (+`--no-toml`), `--index` |
| Filter | `--filter`, `--implement[…]` |
| Code style | `--sys` (+`--extern`), `--minimal` |
| Dependencies | `--deps {core,specific,none}`, `--link` |
| Misc | `--derive`, `--rustfmt`, `--reference`, `--etc` |

Key design decisions:

- **`--sys` vs `--minimal`** are the same "lean mode" axis but produce
  different ABI contracts — `--minimal` preserves vtable/`_Impl`/`RuntimeType`,
  `--sys` does not. Two flags, one internal enum (`Style`), is the right shape.

- **`--deps none` vs `--sys`** are orthogonal. `--deps none` is also
  meaningful with `--minimal` (for crates that bootstrap themselves).

- **`--filter` method-level grammar** (`?Ns.Type`, `Ns.Type::Method`, etc.)
  is a deliberate DSL with documented escape hatches — better than an expanding
  set of top-level flags.

- **`--package` vs `--flat` vs default modules** are three distinct output
  topologies driving different writer code paths, modelled as a `Layout` enum.
