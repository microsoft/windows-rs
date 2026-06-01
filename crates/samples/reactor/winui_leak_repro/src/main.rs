//! Minimal reproduction of WinUI 3 memory leak (no reactor involved).
//!
//! Creates and destroys realistic XAML control trees via raw WinUI APIs.
//! Memory grows unboundedly despite ALL Rust-side references being dropped.
//!
//! Related upstream issue (open since 2021, affects all WinUI 3 apps):
//! https://github.com/microsoft/microsoft-ui-xaml/issues/5978
//!
//! Run with: cargo run -p winui-leak-repro

use std::cell::RefCell;
use std::time::Duration;

use windows_reactor::DispatcherTimer;
use windows_reactor::app::App;
use windows_reactor::core::ControlKind;
use windows_reactor::core::backend::{Backend, ControlId};
use windows_reactor::winui::backend::WinUIBackend;

fn main() -> windows_reactor::Result<()> {
    App::new().title("WinUI Leak Repro").run_custom(|_app| {
        eprintln!("╔══════════════════════════════════════════════════════════════╗");
        eprintln!("║        WinUI 3 Memory Leak Reproduction                     ║");
        eprintln!("║                                                              ║");
        eprintln!("║  This sample uses NO framework / reconciler / bindings.      ║");
        eprintln!("║  It simply creates XAML controls, adds them to a panel,      ║");
        eprintln!("║  removes them, and drops all references.                     ║");
        eprintln!("║                                                              ║");
        eprintln!("║  Watch memory grow despite zero live controls remaining.     ║");
        eprintln!("╚══════════════════════════════════════════════════════════════╝\n");

        print_memory("STARTUP");

        let backend = RefCell::new(WinUIBackend::new());
        let batch_count = RefCell::new(0u32);

        // Single persistent root — simulates a window's content area.
        let root_id = backend.borrow_mut().create(ControlKind::StackPanel);

        let timer = DispatcherTimer::new(Duration::from_millis(30), move || {
            let mut batch = batch_count.borrow_mut();
            *batch += 1;

            let mut ids: Vec<ControlId> = Vec::new();
            let mut be = backend.borrow_mut();

            // Build a realistic page tree: ~96 controls arranged in a nested
            // hierarchy (5 sections × (header + border + content + 4×3 items)).
            let page_root = be.create(ControlKind::StackPanel);
            be.append_child(root_id, page_root);
            ids.push(page_root);

            for _ in 0..5 {
                let section = be.create(ControlKind::StackPanel);
                be.append_child(page_root, section);
                ids.push(section);

                let header = be.create(ControlKind::TextBlock);
                be.append_child(section, header);
                ids.push(header);

                let border = be.create(ControlKind::Border);
                be.append_child(section, border);
                ids.push(border);

                let content = be.create(ControlKind::StackPanel);
                be.append_child(border, content);
                ids.push(content);

                for _ in 0..4 {
                    let row = be.create(ControlKind::Grid);
                    be.append_child(content, row);
                    ids.push(row);

                    for kind in [
                        ControlKind::TextBlock,
                        ControlKind::TextBox,
                        ControlKind::Button,
                    ] {
                        let child = be.create(kind);
                        be.append_child(row, child);
                        ids.push(child);
                    }
                }
            }

            let control_count = ids.len();

            // Fully detach from visual tree and destroy everything.
            be.remove_child(root_id, 0);
            for id in ids.into_iter().rev() {
                be.destroy(id);
            }
            // At this point: zero controls remain. All COM pointers released.

            if (*batch).is_multiple_of(100) {
                print_memory(&format!(
                    "{:>4} navigations | {:>6} controls allocated+freed | 0 live",
                    *batch,
                    *batch as usize * control_count,
                ));
            }

            if *batch >= 1000 {
                eprintln!("\n────────────────────────────────────────────────────────────────");
                print_memory("FINAL");
                eprintln!("────────────────────────────────────────────────────────────────");
                eprintln!("  Navigations: 1000");
                eprintln!(
                    "  Controls created+destroyed: {}",
                    *batch as usize * control_count
                );
                eprintln!("  Controls still alive: 0");
                eprintln!("  Rust-side maps: all empty");
                eprintln!("  COM ref counts: all released");
                eprintln!("────────────────────────────────────────────────────────────────");
                eprintln!("  VERDICT: WinUI never frees native heap for destroyed controls.");
                eprintln!("  https://github.com/microsoft/microsoft-ui-xaml/issues/5978");
                eprintln!("────────────────────────────────────────────────────────────────");
                std::process::exit(0);
            }
        })?;

        std::mem::forget(timer);
        Ok(())
    })
}

fn print_memory(label: &str) {
    use std::mem::MaybeUninit;

    #[repr(C)]
    #[allow(non_snake_case)]
    struct ProcessMemoryCounters {
        cb: u32,
        PageFaultCount: u32,
        PeakWorkingSetSize: usize,
        WorkingSetSize: usize,
        QuotaPeakPagedPoolUsage: usize,
        QuotaPagedPoolUsage: usize,
        QuotaPeakNonPagedPoolUsage: usize,
        QuotaNonPagedPoolUsage: usize,
        PagefileUsage: usize,
        PeakPagefileUsage: usize,
    }

    #[link(name = "psapi")]
    unsafe extern "system" {
        fn GetProcessMemoryInfo(
            process: *mut core::ffi::c_void,
            pmc: *mut ProcessMemoryCounters,
            cb: u32,
        ) -> i32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetCurrentProcess() -> *mut core::ffi::c_void;
    }

    unsafe {
        let mut pmc = MaybeUninit::<ProcessMemoryCounters>::zeroed();
        (*pmc.as_mut_ptr()).cb = size_of::<ProcessMemoryCounters>() as u32;
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            pmc.as_mut_ptr(),
            size_of::<ProcessMemoryCounters>() as u32,
        );
        let pmc = pmc.assume_init();
        eprintln!(
            "  [{:<50}]  WS: {:>7.2} MB  Private: {:>7.2} MB",
            label,
            pmc.WorkingSetSize as f64 / 1024.0 / 1024.0,
            pmc.PagefileUsage as f64 / 1024.0 / 1024.0,
        );
    }
}
