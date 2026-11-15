use super::*;

struct Snapshot(HANDLE);

impl Drop for Snapshot {
    fn drop(&mut self) {
        if !unsafe { CloseHandle(self.0) }.as_bool() {
            eprintln!(
                "failed to close module snapshot: {}",
                windows_core::Error::from_thread()
            );
        }
    }
}

pub fn dump(fixture: u32) {
    for (role, process) in [("UIA driver", std::process::id()), ("fixture", fixture)] {
        eprintln!(
            "input platform: {role}, PID {process}, arch {}",
            std::env::consts::ARCH
        );
        if let Err(error) = modules(process) {
            eprintln!("could not inspect {role} modules: {error}");
        }
    }
}

fn modules(process: u32) -> Result<()> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE as u32, process);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(windows_core::Error::from_thread().into());
        }
        let snapshot = Snapshot(snapshot);
        let mut module = MODULEENTRY32W {
            dwSize: size_of::<MODULEENTRY32W>() as u32,
            ..Default::default()
        };
        Module32FirstW(snapshot.0, &mut module).ok()?;
        loop {
            let name = String::from_utf16_lossy(
                &module.szModule[..module.szModule.iter().position(|&c| c == 0).unwrap()],
            );
            if matches!(
                name.to_ascii_lowercase().as_str(),
                "ntdll.dll"
                    | "user32.dll"
                    | "win32u.dll"
                    | "uiautomationcore.dll"
                    | "dcomp.dll"
                    | "coremessagingxp.dll"
                    | "microsoft.ui.xaml.dll"
                    | "microsoft.ui.input.dll"
                    | "microsoft.ui.dll"
                    | "microsoft.internal.frameworkudk.dll"
                    | "microsoft.windowsappruntime.dll"
            ) {
                let path = windows_core::PCWSTR(module.szExePath.as_ptr());
                let version = file_version(path)?;
                eprintln!(
                    "loaded module: {}, file version {}.{}.{}.{}, product version {}.{}.{}.{}",
                    path.to_string()?,
                    version.dwFileVersionMS >> 16,
                    version.dwFileVersionMS & 0xffff,
                    version.dwFileVersionLS >> 16,
                    version.dwFileVersionLS & 0xffff,
                    version.dwProductVersionMS >> 16,
                    version.dwProductVersionMS & 0xffff,
                    version.dwProductVersionLS >> 16,
                    version.dwProductVersionLS & 0xffff,
                );
            }
            if !Module32NextW(snapshot.0, &mut module).as_bool() {
                let error = GetLastError();
                if error == ERROR_NO_MORE_FILES as u32 {
                    break;
                }
                return Err(format!("Module32NextW failed with Win32 error {error}").into());
            }
        }
    }
    Ok(())
}

unsafe fn file_version(path: windows_core::PCWSTR) -> Result<VS_FIXEDFILEINFO> {
    unsafe {
        let size = GetFileVersionInfoSizeW(path, std::ptr::null_mut());
        if size == 0 {
            return Err(windows_core::Error::from_thread().into());
        }
        let mut buffer = vec![0u8; size as usize];
        GetFileVersionInfoW(path, 0, size, buffer.as_mut_ptr().cast()).ok()?;
        let mut value = std::ptr::null_mut();
        let mut length = 0;
        let queried = VerQueryValueW(
            buffer.as_ptr().cast(),
            windows_core::w!("\\"),
            &mut value,
            &mut length,
        );
        if !queried.as_bool()
            || value.is_null()
            || (length as usize) < size_of::<VS_FIXEDFILEINFO>()
        {
            return Err("module has no fixed file version".into());
        }
        Ok(value.cast::<VS_FIXEDFILEINFO>().read_unaligned())
    }
}
