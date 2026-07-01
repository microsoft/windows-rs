# Samples

Runnable examples for the `windows-*` family of crates, grouped to mirror the
[crate index](../../docs/readme.md).

Samples come in two shapes:

- **Umbrella crates** (named `<group>_samples`) collect many small, focused
  programs under an `examples/` folder. Run one with
  `cargo run -p <crate> --example <name>`.
- **Standalone crates** are a single self-contained program (often with their own
  build script, manifest, or assets). Run one with `cargo run -p <crate>`.

The samples compile against the latest, usually pre-release, versions of the
crates. To browse the samples for a published release, use its tag — for example
<https://github.com/microsoft/windows-rs/tree/0.60.0/crates/samples>.

## Core & error handling

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-result](../../docs/crates/windows-result.md) | `result_samples` — building and propagating Windows errors: error, propagate | `cargo run -p result_samples --example <name>` |
| [windows-strings](../../docs/crates/windows-strings.md) | `strings_samples` — the Windows string types: hstring, bstr, wide_ansi | `cargo run -p strings_samples --example <name>` |

## Values & collections

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-time](../../docs/crates/windows-time.md) | `time_samples` — the `TimeSpan` and `DateTime` value types: time_types | `cargo run -p time_samples --example <name>` |

## Async & threading

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-future](../../docs/crates/windows-future.md) | `future_samples` — the stock WinRT async types: spawn | `cargo run -p future_samples --example <name>` |
| [windows-threading](../../docs/crates/windows-threading.md) | `threading_samples` — the Windows thread pool: for_each, pool | `cargo run -p threading_samples --example <name>` |

## System services

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-registry](../../docs/crates/windows-registry.md) | `registry_samples` — reading and writing the registry: read_write, transaction | `cargo run -p registry_samples --example <name>` |
| [windows-services](../../docs/crates/windows-services.md) | `services_simple` · `services_time` | `cargo run -p <crate>` |

## UI & graphics

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-reactor](../../docs/crates/windows-reactor.md) | `reactor_samples` — one demo per WinUI control plus the reactor hooks (97 examples) | `cargo run -p reactor_samples --example <name>` |
| | `reactor_apps` — complete apps: dotsweeper, minesweeper, notepad, solitaire, tictactoe | `cargo run -p reactor_apps --example <name>` |
| | `reactor_gallery` · `reactor_direct2d` · `reactor_webview` · `reactor_swap_chain_panel` · `reactor_self_contained` · `reactor_framework_dependent` | `cargo run -p <crate>` |
| [windows-canvas](../../docs/crates/windows-canvas.md) | `canvas_samples` — Direct2D drawing: bitmap, brush, color, curves, draw_text, gradient, hello, lines, path, shapes, stroke, transform | `cargo run -p canvas_samples --example <name>` |
| | `canvas_circles` · `canvas_clock` · `canvas_editor` · `canvas_hit_test` · `canvas_standalone` | `cargo run -p <crate>` |
| [windows-animation](../../docs/crates/windows-animation.md) | `animation_samples` — the Windows Animation Manager, headless: variable, storyboard | `cargo run -p animation_samples --example <name>` |
| [windows-webview](../../docs/crates/windows-webview.md) | `webview_samples` — WebView2 hosting: cookies, custom_protocol, downloads, events, ipc, local_files, minimal, profile, script | `cargo run -p webview_samples --example <name>` |

## Interop (cross-language)

| Sample | Description |
| --- | --- |
| `robot` · `robot_client` · `robot_client_cpp` · `robot_client_cs` · `robot_cpp` | A component whose metadata is authored once (RDL) and consumed from Rust, C++, and C#. |
| `lang_perf_component` · `lang_perf_rust` · `lang_perf_cpp` · `lang_perf_cs` | Benchmark of WinRT language-projection overhead — C++/WinRT vs C#/WinRT vs Rust calling one Rust-authored no-op component. See its [readme](lang_perf/readme.md). |
| `csharp_component` · `csharp_client` | Author a component in Rust and host it from C#. |

## Whole-API projection

These exercise the umbrella [`windows`](../../docs/crates/windows.md) and
[`windows-sys`](../../docs/crates/windows-sys.md) crates, which project the entire
Windows API surface. For new projects, prefer the focused crates above or generate
a minimal binding with [windows-bindgen](../../docs/crates/windows-bindgen.md).

| Crate | Sample | Run |
| --- | --- | --- |
| [windows](../../docs/crates/windows.md) | `windows_samples` — Win32, COM, and WinRT: bits, com_uri, consent, counter, create_window, credentials, data_protection, delay_load, device_watcher, enum_windows, kernel_event, memory_buffer, message_box, privileges, rss, shell, simple, thread_pool_work, window_message, wmi, xml | `cargo run -p windows_samples --example <name>` |
| | `windows_core_app` · `windows_dcomp` · `windows_direct2d` · `windows_direct3d12` · `windows_file_dialogs` · `windows_ocr` · `windows_overlapped` · `windows_spellchecker` · `windows_task_dialog` · `windows_uiautomation` | `cargo run -p <crate>` |
| [windows-sys](../../docs/crates/windows-sys.md) | `windows_sys_samples` — raw C-style bindings: create_window, message_box, service | `cargo run -p windows_sys_samples --example <name>` |
| | `windows_sys_task_dialog` | `cargo run -p windows_sys_task_dialog` |

---

Many of these samples were inspired by or originally appeared in Kenny's
[articles](https://kennykerrca.wordpress.com/articles/) and
[Pluralsight courses](https://www.pluralsight.com/authors/kenny-kerr).
