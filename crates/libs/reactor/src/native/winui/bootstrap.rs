use crate::native::winui::bindings::*;
use windows_core::*;

const FRAMEWORK_FAMILY: PCWSTR = w!("Microsoft.WindowsAppRuntime.2_8wekyb3d8bbwe");
const PACKAGE_DEPENDENCY_LIFETIME_KIND_PROCESS: i32 = 0;

static BOOTSTRAPPED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);

/// Initializes the Windows App Runtime for framework-dependent apps by resolving the installed
/// framework package and adding it to the process package graph directly.
///
/// Returns an error (and shows an install dialog) if the runtime framework package is not
/// installed on the machine.
pub fn bootstrap() -> Result<()> {
    let mut bootstrapped = BOOTSTRAPPED.lock().unwrap();
    if *bootstrapped {
        return Ok(());
    }
    unsafe { bootstrap_inner()? };
    *bootstrapped = true;
    Ok(())
}

unsafe fn bootstrap_inner() -> Result<()> {
    let mut dependency_id: PWSTR = PWSTR::null();
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

    // This handle is unused as the package dependency lifetime is configured for the lifetime
    // of the process.
    let mut handle = std::ptr::null_mut();
    let mut package_full_name = PWSTR::null();
    unsafe {
        let add_result = AddPackageDependency(
            PCWSTR(dependency_id.0),
            0,
            0,
            &mut handle,
            &mut package_full_name,
        );

        let _ = HeapFree(
            GetProcessHeap(),
            0,
            dependency_id.0 as *mut std::ffi::c_void,
        );

        let _ = HeapFree(
            GetProcessHeap(),
            0,
            package_full_name.0 as *mut std::ffi::c_void,
        );

        if add_result == STATEREPOSITORY_E_DEPENDENCY_NOT_RESOLVED {
            show_install_dialog();
            return Err(Error::new(
                add_result,
                "Microsoft.WindowsAppRuntime.2 framework package is not installed.",
            ));
        }

        add_result.ok()?;
    };

    Ok(())
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
        .and_then(|path| path.file_name().map(|f| f.to_string_lossy().into_owned()))
        .map_or_else(
            || HSTRING::from("This application could not be started"),
            HSTRING::from,
        )
}

fn show_install_dialog() {
    let caption = process_caption();
    let text = HSTRING::from(format!(
        "You must install Windows App Runtime ({WINDOWSAPPSDK_RUNTIME_VERSION_MAJOR}.{WINDOWSAPPSDK_RUNTIME_VERSION_MINOR}.{WINDOWSAPPSDK_RUNTIME_VERSION_BUILD}.{WINDOWSAPPSDK_RUNTIME_VERSION_REVISION}, {}) to run this application.\n\
         \n\
         Do you want to download it now?",
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
