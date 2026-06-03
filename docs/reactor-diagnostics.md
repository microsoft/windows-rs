# Diagnostics & Error Handling

## Panic Behavior

On Windows, panics in event handlers unwind through COM boundaries. Neither
`abort()` nor `panic = "abort"` reliably terminates WinUI processes — the XAML
runtime keeps the HWND alive, causing a "silent hang."

**Fix:** The `diagnostics` feature's panic hook calls `std::process::exit(101)`
after printing the backtrace and writing `%TEMP%/windows-reactor-crash-{pid}.log`.
The `ExpectPanicGuard` skips this for panics caught by `ErrorBoundary`.

## Error Categories

| Category | Convention |
|----------|-----------|
| **Invariant violation** | `panic!` / `.unwrap()` → hook terminates process with backtrace |
| **Coverage gap** | `diag::unhandled_*` → warn in debug, no-op in release |
| **COM runtime error** | `if let Err(e)` → `diag::com_error(...)`, continue |

## Rules

1. **Never `panic!` for a missing feature.** Unhandled props/events warn and no-op.
2. **Use `.unwrap()` not `.expect("...")`** — the panic hook provides full context.
3. **`panic!` only for invariant violations** — corrupted state that will cascade.
4. **Use `diag::` helpers, not bare `eprintln!`.** All output prefixed `windows-reactor:`.
5. **Debug-only output.** Warnings gated behind `cfg!(debug_assertions)`.

## Diagnostic Helpers (`winui/backend/diag.rs`)

```rust
diag::unhandled_prop(id, prop, value, handle)   // prop silently ignored
diag::unhandled_modifier(site, prop, handle)    // modifier not applicable
diag::com_error(site, id, err)                  // COM call failed
```
