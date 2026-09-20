use super::bindings::*;
use std::sync::Mutex;
use windows_core::*;

const FRAMEWORK_FAMILY: PCWSTR = w!("Microsoft.WindowsAppRuntime.2_8wekyb3d8bbwe");
const PACKAGE_DEPENDENCY_LIFETIME_KIND_PROCESS: i32 = 0;

static BOOTSTRAPPED: Mutex<bool> = Mutex::new(false);

pub(super) fn bootstrap() -> Result<()> {
    let mut bootstrapped = BOOTSTRAPPED.lock().unwrap();
    if *bootstrapped {
        return Ok(());
    }
    unsafe { bootstrap_inner()? };
    *bootstrapped = true;
    Ok(())
}

unsafe fn bootstrap_inner() -> Result<()> {
    let mut dependency_id = PWSTR::null();
    let hr = unsafe {
        TryCreatePackageDependency(
            std::ptr::null_mut(),
            FRAMEWORK_FAMILY,
            PACKAGE_VERSION {
                Anonymous: PACKAGE_VERSION_0 {
                    Version: WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
                },
            },
            process_architecture().flags | PackageDependencyProcessorArchitectures_Neutral,
            PACKAGE_DEPENDENCY_LIFETIME_KIND_PROCESS,
            PCWSTR::null(),
            0,
            &mut dependency_id,
        )
    };
    if hr == STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED {
        show_install_dialog();
        return Err(Error::new(
            hr,
            "Microsoft.WindowsAppRuntime.2 framework package is not installed.",
        ));
    }
    hr.ok()?;

    let mut handle = std::ptr::null_mut();
    let mut package_full_name = PWSTR::null();
    let result = unsafe {
        AddPackageDependency(
            PCWSTR(dependency_id.0),
            0,
            0,
            &mut handle,
            &mut package_full_name,
        )
    };
    unsafe {
        _ = HeapFree(
            GetProcessHeap(),
            0,
            dependency_id.0.cast::<std::ffi::c_void>(),
        );
        _ = HeapFree(
            GetProcessHeap(),
            0,
            package_full_name.0.cast::<std::ffi::c_void>(),
        );
    }
    if result == STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED {
        show_install_dialog();
        return Err(Error::new(
            result,
            "Microsoft.WindowsAppRuntime.2 framework package is not installed.",
        ));
    }
    result.ok()
}

#[derive(Clone, Copy)]
struct ProcessArchitecture {
    display: &'static str,
    flags: PackageDependencyProcessorArchitectures,
}

fn process_architecture() -> ProcessArchitecture {
    match std::env::consts::ARCH {
        "x86_64" => ProcessArchitecture {
            display: "x64",
            flags: PackageDependencyProcessorArchitectures_X64,
        },
        "x86" => ProcessArchitecture {
            display: "x86",
            flags: PackageDependencyProcessorArchitectures_X86,
        },
        "aarch64" => ProcessArchitecture {
            display: "arm64",
            flags: PackageDependencyProcessorArchitectures_Arm64,
        },
        "arm" => ProcessArchitecture {
            display: "arm",
            flags: PackageDependencyProcessorArchitectures_Arm,
        },
        _ => ProcessArchitecture {
            display: std::env::consts::ARCH,
            flags: PackageDependencyProcessorArchitectures_None,
        },
    }
}

fn process_caption() -> HSTRING {
    std::env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .map_or_else(
            || HSTRING::from("This application could not be started"),
            HSTRING::from,
        )
}

fn show_install_dialog() {
    let caption = process_caption();
    let text = HSTRING::from(format!(
        "You must install Windows App Runtime \
         ({WINDOWSAPPSDK_RUNTIME_VERSION_MAJOR}.{WINDOWSAPPSDK_RUNTIME_VERSION_MINOR}.\
         {WINDOWSAPPSDK_RUNTIME_VERSION_BUILD}.{WINDOWSAPPSDK_RUNTIME_VERSION_REVISION}, {}) \
         to run this application.\n\nDo you want to download it now?",
        process_architecture().display
    ));
    let result = unsafe {
        MessageBoxW(
            HWND::default(),
            PCWSTR::from_raw(text.as_ptr()),
            PCWSTR::from_raw(caption.as_ptr()),
            (MB_YESNO | MB_ICONERROR) as u32,
        )
    };
    if result == IDYES {
        unsafe {
            ShellExecuteW(
                HWND::default(),
                w!("open"),
                w!("https://learn.microsoft.com/windows/apps/windows-app-sdk/downloads"),
                PCWSTR::null(),
                PCWSTR::null(),
                SW_SHOWNORMAL,
            );
        }
    }
}
