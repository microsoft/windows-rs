//! Panic-hook diagnostics: stderr report + per-pid crash log under `%TEMP%`.

use std::any::Any;
use std::cell::Cell;
use std::io::Write;

#[cfg(feature = "diagnostics")]
use std::path::PathBuf;
#[cfg(feature = "diagnostics")]
use std::sync::Once;

#[cfg(feature = "diagnostics")]
static INSTALL_ONCE: Once = Once::new();

thread_local! {
    static EXPECT_PANIC: Cell<u32> = const { Cell::new(0) };
}

/// RAII guard suppressing the diagnostics panic hook on the current thread.
pub struct ExpectPanicGuard {
    _not_send: std::marker::PhantomData<*const ()>,
}

impl ExpectPanicGuard {
    pub fn new() -> Self {
        EXPECT_PANIC.with(|c| c.set(c.get().saturating_add(1)));
        Self {
            _not_send: std::marker::PhantomData,
        }
    }
}

impl Default for ExpectPanicGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ExpectPanicGuard {
    fn drop(&mut self) {
        EXPECT_PANIC.with(|c| c.set(c.get().saturating_sub(1)));
    }
}

#[cfg(feature = "diagnostics")]
fn panic_is_expected() -> bool {
    EXPECT_PANIC.with(|c| c.get() > 0)
}

/// Install the diagnostics panic hook (idempotent; no-op without `diagnostics` feature).
pub fn install() {
    #[cfg(feature = "diagnostics")]
    INSTALL_ONCE.call_once(install_inner);
}

#[cfg(feature = "diagnostics")]
fn install_inner() {
    if std::env::var_os("RUST_BACKTRACE").is_none() {
        unsafe {
            std::env::set_var("RUST_BACKTRACE", "1");
        }
    }

    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if panic_is_expected() {
            return;
        }

        prev(info);

        if let Some(path) = crash_log_path()
            && let Ok(mut f) = std::fs::File::create(&path)
        {
            let bt = std::backtrace::Backtrace::force_capture();
            let _ = writeln!(f, "{info}\n\nBacktrace:\n{bt}");
            let _ = f.flush();
            emit(&format!(
                "windows_reactor: panic log written to {}\n",
                path.display()
            ));
        }

        // Force-kill: std::process::abort() doesn't reliably terminate WinUI apps.
        emit("windows_reactor: terminating process due to unrecoverable panic\n");
        std::process::exit(101);
    }));
}

pub fn emit(msg: &str) {
    let mut stderr = std::io::stderr().lock();
    let _ = stderr.write_all(msg.as_bytes());
    if !msg.ends_with('\n') {
        let _ = stderr.write_all(b"\n");
    }
    let _ = stderr.flush();
}

/// Render a panic payload as a string for diagnostic output.
pub fn format_panic_payload(payload: &(dyn Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "<non-string panic payload>".to_string()
    }
}

#[cfg(feature = "diagnostics")]
fn crash_log_path() -> Option<PathBuf> {
    let mut p = std::env::temp_dir();
    let pid = std::process::id();
    p.push(format!("windows-reactor-crash-{pid}.log"));
    Some(p)
}
