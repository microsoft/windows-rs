#[expect(non_snake_case, clippy::upper_case_acronyms)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use bindings::*;
use core::ffi::c_void;
use std::sync::Mutex;
use std::time::{Duration, Instant};

const SETTLE: Duration = Duration::from_secs(1);
const TIMEOUT: Duration = Duration::from_secs(30);
const CLEANUP: Duration = Duration::from_secs(5);

static SERIAL: Mutex<()> = Mutex::new(());

/// How the harness closes the picker dialog.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Presses the dialog's commit button.
    Accept,
    /// Closes the dialog as if the user dismissed it.
    Cancel,
}

/// What the harness observed about the native dialog before closing it.
#[derive(Debug)]
pub struct Observed {
    /// Whether the dialog's owner is the window passed to the picker, or no window if unowned.
    pub owner_matches: bool,
    /// The text of the dialog's commit button.
    pub commit_label: String,
}

/// Shows a picker the way a framework with a dedicated dialog thread does.
///
/// The calling thread creates the owner window, if `owned`, and keeps processing messages while
/// `show` runs on a separate STA thread. Once the dialog titled `title` is visible, the harness
/// records what it observes and then performs `action`.
pub fn show<T, F>(title: &str, owned: bool, action: Action, show: F) -> (T, Observed)
where
    T: core::fmt::Debug + Send + 'static,
    F: FnOnce(*mut c_void) -> T + Send + 'static,
{
    let _serial = SERIAL.lock().unwrap_or_else(|error| error.into_inner());

    let owner = if owned {
        let class = wide("STATIC");
        let name = wide("test_pickers owner");
        let owner = unsafe {
            CreateWindowExW(
                0,
                class.as_ptr(),
                name.as_ptr(),
                (WS_OVERLAPPEDWINDOW | WS_VISIBLE) as u32,
                100,
                100,
                480,
                320,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };
        assert!(!owner.is_null());
        owner
    } else {
        core::ptr::null_mut()
    };

    let owner_address = owner as usize;
    let worker = std::thread::spawn(move || {
        let _sta = windows_core::init_sta();
        show(owner_address as *mut c_void)
    });

    let class = wide("#32770");
    let title = wide(title);
    let start = Instant::now();
    let mut found = None;
    let mut observed = None;
    let mut posted = true;
    let mut timed_out = false;

    while !worker.is_finished() {
        pump();

        if !timed_out && start.elapsed() >= TIMEOUT {
            // Close the dialog before failing so the next test never runs alongside it.
            timed_out = true;
            if let Some((dialog, _)) = found {
                unsafe { PostMessageW(dialog, WM_CLOSE as u32, 0, 0) };
            }
        } else if timed_out && start.elapsed() >= TIMEOUT + CLEANUP {
            eprintln!("picker dialog did not close after a timeout");
            std::process::abort();
        } else if observed.is_none() {
            match found {
                None => {
                    let dialog = unsafe { FindWindowW(class.as_ptr(), title.as_ptr()) };
                    if !dialog.is_null() && unsafe { IsWindowVisible(dialog) } != 0 {
                        found = Some((dialog, Instant::now()));
                    }
                }
                // Let the dialog finish navigating to its initial folder before acting.
                Some((dialog, at)) if at.elapsed() >= SETTLE => {
                    observed = Some(Observed {
                        owner_matches: unsafe { GetWindow(dialog, GW_OWNER as u32) } == owner,
                        commit_label: text(unsafe { GetDlgItem(dialog, IDOK) }),
                    });
                    posted = 0
                        != match action {
                            Action::Accept => unsafe {
                                PostMessageW(dialog, WM_COMMAND as u32, IDOK as usize, 0)
                            },
                            Action::Cancel => unsafe {
                                PostMessageW(dialog, WM_CLOSE as u32, 0, 0)
                            },
                        };
                }
                Some(_) => {}
            }
        }

        std::thread::sleep(Duration::from_millis(10));
    }

    let result = worker.join().unwrap();
    if !owner.is_null() {
        unsafe { DestroyWindow(owner) };
    }
    assert!(posted, "failed to post to the picker dialog");
    assert!(!timed_out, "picker dialog did not close: {result:?}");
    let Some(observed) = observed else {
        panic!("picker dialog was not found: {result:?}");
    };
    (result, observed)
}

fn pump() {
    unsafe {
        let mut message = core::mem::zeroed();
        while PeekMessageW(&mut message, core::ptr::null_mut(), 0, 0, PM_REMOVE as u32) != 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

fn text(window: HWND) -> String {
    let mut buffer = [0u16; 256];
    let len = unsafe { GetWindowTextW(window, buffer.as_mut_ptr(), buffer.len() as i32) };
    String::from_utf16_lossy(&buffer[..len as usize])
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain([0]).collect()
}
