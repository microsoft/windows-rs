# Samples

Runnable examples for the `windows-*` family of crates, grouped to mirror the [crate
index](../../docs/readme.md).

Each sample is a standalone Cargo package. Most run with `cargo run -p <crate>`; samples that need
registration or another host include their own instructions.

The samples compile against the latest, usually pre-release, versions of the crates. To browse the
samples for a published release, use its tag - for example
<https://github.com/microsoft/windows-rs/tree/0.60.0/crates/samples>.

## Core & error handling

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-result](../../docs/crates/windows-result.md) | `result-result` | `cargo run -p result-result` |
| [windows-strings](../../docs/crates/windows-strings.md) | `strings-hstring` / `strings-bstr` / `strings-wide-ansi` | `cargo run -p <crate>` |

## Values & collections

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-collections](../../docs/crates/windows-collections.md) | `collections-collections` | `cargo run -p collections-collections` |
| [windows-reference](../../docs/crates/windows-reference.md) | `reference-reference` | `cargo run -p reference-reference` |
| [windows-time](../../docs/crates/windows-time.md) | `time-time-types` | `cargo run -p time-time-types` |

## Async & threading

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-future](../../docs/crates/windows-future.md) | `future-spawn` | `cargo run -p future-spawn` |
| [windows-threading](../../docs/crates/windows-threading.md) | `threading-for-each` / `threading-pool` | `cargo run -p <crate>` |

## System services

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-registry](../../docs/crates/windows-registry.md) | `registry-read-write` / `registry-transaction` | `cargo run -p <crate>` |
| [windows-services](../../docs/crates/windows-services.md) | `services-simple` / `services-time` | `cargo run -p <crate>` |
| [windows-version](../../docs/crates/windows-version.md) | `version-version` | `cargo run -p version-version` |

## UI & graphics

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-reactor](../../docs/crates/windows-reactor.md) | `reactor-gallery` - WinUI control catalog | `cargo run -p reactor-gallery` |
| | `reactor-message-box` - modal native window work | `cargo run -p reactor-message-box` |
| | `reactor-async-state` / `reactor-color-scheme` / `reactor-component-input` / `reactor-context` / `reactor-element-ref` / `reactor-function-component` / `reactor-keyed-list-reorder` / `reactor-use-effect` | `cargo run -p <crate>` |
| | `reactor-drag-drop` / `reactor-exit-transition` / `reactor-lightweight-resources` / `reactor-opacity-transition` / `reactor-responsive-navigation` / `reactor-scale-transition` | `cargo run -p <crate>` |
| | `reactor-pointer-position` / `reactor-pointer-resize` / `reactor-pointer-tracking` / `reactor-secondary-window` / `reactor-text-box-border` / `reactor-tooltip-placement` / `reactor-window` | `cargo run -p <crate>` |
| | `reactor-button-icon` / `reactor-calculator` / `reactor-navigation-view-icons` / `reactor-navigation-view-pane` / `reactor-radio-buttons` / `reactor-tab-view-add-button` | `cargo run -p <crate>` |
| | `reactor-card` / `reactor-icon` / `reactor-icon-elements` / `reactor-image-icon-size` / `reactor-scroll-viewer` / `reactor-shape` / `reactor-text-block` / `reactor-text-trimming` / `reactor-theme-brush` | `cargo run -p <crate>` |
| | `reactor-dotsweeper` / `reactor-minesweeper` / `reactor-notepad` / `reactor-solitaire` / `reactor-stacker` / `reactor-tictactoe` | `cargo run -p <crate>` |
| | `reactor-self-contained` / `reactor-framework-dependent` | `cargo run -p <crate>` |
| | `reactor-controlled` / `reactor-counter` / `reactor-form` / `reactor-navigation` / `reactor-virtual` | `cargo run -p <crate>` |
| | `reactor-startup-perf` - blank app demonstrating startup TraceLogging events | `cargo run -p reactor-startup-perf --release` |
| [windows-canvas](../../docs/crates/windows-canvas.md) | `canvas-bitmap` / `canvas-bitmap-from-bytes` / `canvas-color` / `canvas-curves` / `canvas-draw-text` / `canvas-gradient` / `canvas-hello` / `canvas-invalidate` / `canvas-lines` / `canvas-path` / `canvas-shapes` / `canvas-stroke` / `canvas-transform` | `cargo run -p <crate>` |
| | `canvas-chart` / `canvas-circles` / `canvas-clock` / `canvas-editor` / `canvas-hit-test` / `canvas-image-source` / `canvas-readback` / `canvas-shared-device` / `canvas-standalone` / `canvas-text-layout` | `cargo run -p <crate>` |
| [windows-composition](../../docs/crates/windows-composition.md) | `composition-animation` / `composition-canvas` / `composition-circles` / `composition-host` / `composition-minesweeper` / `composition-standalone` / `composition-toggle` | `cargo run -p <crate>` |
| [windows-animation](../../docs/crates/windows-animation.md) | `animation-storyboard` | `cargo run -p animation-storyboard` |
| [windows-pickers](../../docs/crates/windows-pickers.md) | `pickers-standalone` / `pickers-reactor` | `cargo run -p <crate>` |
| [windows-trayicon](../../docs/crates/windows-trayicon.md) | `trayicon-basic` / `reactor-trayicon` | `cargo run -p <crate>` |
| [windows-webview](../../docs/crates/windows-webview.md) | `webview-cookies` / `webview-custom-protocol` / `webview-devtools` / `webview-downloads` / `webview-events` / `webview-ipc` / `webview-local-files` / `webview-minimal` / `webview-profile` / `webview-reactor` / `webview-reactor-window` / `webview-script` | `cargo run -p <crate>` |

## Interop (cross-language)

| Sample | Description |
| --- | --- |
| `robot-component` / `robot-client` / `robot-client-cpp` / `robot-client-cs` / `robot-component-cpp` | A component whose metadata is authored once (RDL) and consumed from Rust, C++, and C#. |
| `csharp-component` / `csharp-client` | Author a component in Rust and host it from C#. |

## Code generation

| Crate | Sample | Run |
| --- | --- | --- |
| [windows-bindgen](../../docs/crates/windows-bindgen.md) | `bindgen-context-alignment` / `bindgen-vss-backup` | `cargo run -p <crate>` |

## Whole-API projection

These exercise the umbrella [`windows`](../../docs/crates/windows.md) and
[`windows-sys`](../../docs/crates/windows-sys.md) crates, which project the entire Windows API
surface. For new projects, prefer the focused crates above or generate a minimal binding with
[windows-bindgen](../../docs/crates/windows-bindgen.md).

| Crate | Sample | Run |
| --- | --- | --- |
| [windows](../../docs/crates/windows.md) | One crate per Win32, COM, or WinRT sample under [`windows/`](windows) | `cargo run -p windows-<name>` |
| | `windows-dcomp` / `windows-direct2d` / `windows-direct3d12` / `windows-file-dialogs` / `windows-ocr` / `windows-overlapped` / `windows-spellchecker` / `windows-task-dialog` / `windows-uiautomation` | `cargo run -p <crate>` |
| | `windows-core-app` (packaged CoreApplication) | `crates\samples\windows\core_app\register.cmd`, then launch from Start |
| [windows-sys](../../docs/crates/windows-sys.md) | `windows-sys-create-window` / `windows-sys-service` | `cargo run -p <crate>` |
| | `windows-sys-task-dialog` | `cargo run -p windows-sys-task-dialog` |

---

Many of these samples were inspired by or originally appeared in Kenny's
[articles](https://kennykerrca.wordpress.com/articles/) and [Pluralsight
courses](https://www.pluralsight.com/authors/kenny-kerr).
