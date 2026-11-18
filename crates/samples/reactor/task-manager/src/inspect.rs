//! Per-process details queried with `PROCESS_QUERY_LIMITED_INFORMATION`. Every query first
//! verifies the creation time, so values never describe a different process that reused the PID.

use crate::bindings::*;
use crate::model::ProcessId;
use std::io;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::time::Instant;
use windows_core::{PCWSTR, PWSTR};

pub struct Identity {
    pub path: Result<String, String>,
    pub version: Option<Version>,
    pub user: Result<String, String>,
    pub elevated: Result<bool, String>,
    pub architecture: Result<&'static str, String>,
    pub started: String,
}

pub struct Version {
    pub description: Option<String>,
    pub company: Option<String>,
    pub file_version: Option<String>,
}

pub struct Counters {
    pub sampled: Instant,
    pub processors: u32,
    pub kernel: u64,
    pub user: u64,
    pub priority: Result<&'static str, String>,
    pub handles: Result<u32, String>,
    pub gdi_objects: Result<u32, String>,
    pub user_objects: Result<u32, String>,
    pub io: Result<IO_COUNTERS, String>,
    pub memory: Result<PROCESS_MEMORY_COUNTERS_EX, String>,
}

pub fn identity(id: ProcessId) -> Result<Identity, String> {
    let (handle, times) = verified(id)?;
    let path = image_path(&handle);
    Ok(Identity {
        version: path.as_deref().ok().and_then(version),
        path,
        user: token(&handle).and_then(|token| user(&token)),
        elevated: token(&handle).and_then(|token| elevated(&token)),
        architecture: architecture(&handle),
        started: local_time(times.created),
    })
}

pub fn counters(id: ProcessId) -> Result<Counters, String> {
    let (handle, times) = verified(id)?;
    let sampled = Instant::now();
    let raw = handle.as_raw_handle();
    let mut handles = 0;
    let mut io = IO_COUNTERS::default();
    let mut memory = PROCESS_MEMORY_COUNTERS_EX {
        cb: size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32,
        ..Default::default()
    };
    Ok(Counters {
        sampled,
        processors: unsafe { GetActiveProcessorCount(ALL_PROCESSOR_GROUPS as u16) },
        kernel: times.kernel,
        user: times.user,
        priority: match unsafe { GetPriorityClass(raw) } as i32 {
            0 => Err(io::Error::last_os_error().to_string()),
            IDLE_PRIORITY_CLASS => Ok("Idle"),
            BELOW_NORMAL_PRIORITY_CLASS => Ok("Below normal"),
            NORMAL_PRIORITY_CLASS => Ok("Normal"),
            ABOVE_NORMAL_PRIORITY_CLASS => Ok("Above normal"),
            HIGH_PRIORITY_CLASS => Ok("High"),
            REALTIME_PRIORITY_CLASS => Ok("Realtime"),
            _ => Ok("Unknown"),
        },
        handles: check(unsafe { GetProcessHandleCount(raw, &mut handles) }).map(|()| handles),
        gdi_objects: gui_resources(&handle, GR_GDIOBJECTS),
        user_objects: gui_resources(&handle, GR_USEROBJECTS),
        io: check(unsafe { GetProcessIoCounters(raw, &mut io) }).map(|()| io),
        memory: check(unsafe { K32GetProcessMemoryInfo(raw, (&raw mut memory).cast(), memory.cb) })
            .map(|()| memory),
    })
}

// Zero is both a valid count and the failure result, so the last error tells them apart. The
// call fails for processes in another session.
fn gui_resources(handle: &OwnedHandle, kind: i32) -> Result<u32, String> {
    unsafe {
        SetLastError(0);
        match GetGuiResources(handle.as_raw_handle(), kind as u32) {
            0 if GetLastError() != 0 => Err(io::Error::last_os_error().to_string()),
            count => Ok(count),
        }
    }
}

pub fn open(pid: u32) -> io::Result<OwnedHandle> {
    open_with(pid, PROCESS_QUERY_LIMITED_INFORMATION as u32)
}

fn open_with(pid: u32, access: u32) -> io::Result<OwnedHandle> {
    let handle = unsafe { OpenProcess(access, false.into(), pid) };
    if handle.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(unsafe { OwnedHandle::from_raw_handle(handle) })
    }
}

/// Requests termination after verifying that the PID was not reused. Termination is asynchronous,
/// so the process may still be exiting when this returns.
pub fn terminate(id: ProcessId) -> Result<(), String> {
    let created = id.created.ok_or("Process identity is unavailable")?;
    let handle = open_with(
        id.pid,
        (PROCESS_TERMINATE | PROCESS_QUERY_LIMITED_INFORMATION) as u32,
    )
    .map_err(|error| error.to_string())?;
    if times(&handle).map_err(|error| error.to_string())?.created != created {
        return Err("Process exited or its PID was reused".into());
    }
    check(unsafe { TerminateProcess(handle.as_raw_handle(), 1) })
}

pub struct Times {
    pub created: u64,
    pub kernel: u64,
    pub user: u64,
}

pub fn times(handle: &OwnedHandle) -> io::Result<Times> {
    let mut created = FILETIME::default();
    let mut exited = FILETIME::default();
    let mut kernel = FILETIME::default();
    let mut user = FILETIME::default();
    check(unsafe {
        GetProcessTimes(
            handle.as_raw_handle(),
            &mut created,
            &mut exited,
            &mut kernel,
            &mut user,
        )
    })
    .map_err(io::Error::other)?;
    if ticks(exited) != 0 {
        return Err(io::Error::other("Process exited"));
    }
    Ok(Times {
        created: ticks(created),
        kernel: ticks(kernel),
        user: ticks(user),
    })
}

fn verified(id: ProcessId) -> Result<(OwnedHandle, Times), String> {
    let created = id.created.ok_or("Process identity is unavailable")?;
    let handle = open(id.pid).map_err(|error| error.to_string())?;
    let times = times(&handle).map_err(|error| error.to_string())?;
    if times.created != created {
        return Err("Process exited or its PID was reused".into());
    }
    Ok((handle, times))
}

fn check(result: windows_core::BOOL) -> Result<(), String> {
    if result.as_bool() {
        Ok(())
    } else {
        Err(io::Error::last_os_error().to_string())
    }
}

fn ticks(time: FILETIME) -> u64 {
    (u64::from(time.dwHighDateTime) << 32) | u64::from(time.dwLowDateTime)
}

fn image_path(handle: &OwnedHandle) -> Result<String, String> {
    let mut buffer = vec![0u16; 32_768];
    let mut length = buffer.len() as u32;
    check(unsafe {
        QueryFullProcessImageNameW(
            handle.as_raw_handle(),
            0,
            PWSTR(buffer.as_mut_ptr()),
            &mut length,
        )
    })?;
    Ok(String::from_utf16_lossy(&buffer[..length as usize]))
}

fn wide(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

fn version(path: &str) -> Option<Version> {
    let path = wide(path);
    let size = unsafe { GetFileVersionInfoSizeW(PCWSTR(path.as_ptr()), std::ptr::null_mut()) };
    if size == 0 {
        return None;
    }
    // u64 storage keeps the resource's DWORD-aligned blocks aligned.
    let mut data = vec![0u64; (size as usize).div_ceil(size_of::<u64>())];
    check(unsafe { GetFileVersionInfoW(PCWSTR(path.as_ptr()), 0, size, data.as_mut_ptr().cast()) })
        .ok()?;
    let query = |name: &str| -> Option<&[u8]> {
        let name = wide(name);
        let mut value = std::ptr::null_mut();
        let mut length = 0;
        unsafe {
            VerQueryValueW(
                data.as_ptr().cast(),
                PCWSTR(name.as_ptr()),
                &mut value,
                &mut length,
            )
        }
        .as_bool()
        .then(|| unsafe { std::slice::from_raw_parts(value.cast::<u8>(), length as usize) })
    };
    let (language, code_page) = query("\\VarFileInfo\\Translation")
        .filter(|value| value.len() >= 4)
        .map_or((0x0409, 0x04b0), |value| {
            (
                u16::from_le_bytes([value[0], value[1]]),
                u16::from_le_bytes([value[2], value[3]]),
            )
        });
    let string = |name: &str| {
        let name = wide(&format!(
            "\\StringFileInfo\\{language:04x}{code_page:04x}\\{name}"
        ));
        let mut value = std::ptr::null_mut();
        let mut length = 0;
        if !unsafe {
            VerQueryValueW(
                data.as_ptr().cast(),
                PCWSTR(name.as_ptr()),
                &mut value,
                &mut length,
            )
        }
        .as_bool()
        {
            return None;
        }
        // String lengths are in UTF-16 units and may include the terminator.
        let text = unsafe { std::slice::from_raw_parts(value.cast::<u16>(), length as usize) };
        let text = String::from_utf16_lossy(text);
        let text = text.trim_end_matches('\0').trim();
        (!text.is_empty()).then(|| text.to_string())
    };
    Some(Version {
        description: string("FileDescription"),
        company: string("CompanyName"),
        file_version: string("FileVersion"),
    })
}

fn token(handle: &OwnedHandle) -> Result<OwnedHandle, String> {
    let mut token = std::ptr::null_mut();
    check(unsafe { OpenProcessToken(handle.as_raw_handle(), TOKEN_QUERY as u32, &mut token) })?;
    Ok(unsafe { OwnedHandle::from_raw_handle(token) })
}

/// Returns token information in u64-aligned storage.
fn token_information(
    token: &OwnedHandle,
    class: TOKEN_INFORMATION_CLASS,
) -> Result<Vec<u64>, String> {
    let mut length = 0;
    // The size query fails with ERROR_INSUFFICIENT_BUFFER and reports the required length.
    _ = unsafe {
        GetTokenInformation(
            token.as_raw_handle(),
            class,
            std::ptr::null_mut(),
            0,
            &mut length,
        )
    };
    if length == 0 {
        return Err(io::Error::last_os_error().to_string());
    }
    let mut buffer = vec![0u64; (length as usize).div_ceil(size_of::<u64>())];
    check(unsafe {
        GetTokenInformation(
            token.as_raw_handle(),
            class,
            buffer.as_mut_ptr().cast(),
            length,
            &mut length,
        )
    })?;
    Ok(buffer)
}

fn user(token: &OwnedHandle) -> Result<String, String> {
    let buffer = token_information(token, TokenUser)?;
    let sid = unsafe { (*buffer.as_ptr().cast::<TOKEN_USER>()).User.Sid };
    let mut name_length = 0;
    let mut domain_length = 0;
    let mut kind = 0;
    _ = unsafe {
        LookupAccountSidW(
            PCWSTR::null(),
            sid,
            PWSTR::null(),
            &mut name_length,
            PWSTR::null(),
            &mut domain_length,
            &mut kind,
        )
    };
    if name_length == 0 {
        return Err(io::Error::last_os_error().to_string());
    }
    let mut name = vec![0u16; name_length as usize];
    let mut domain = vec![0u16; domain_length.max(1) as usize];
    check(unsafe {
        LookupAccountSidW(
            PCWSTR::null(),
            sid,
            PWSTR(name.as_mut_ptr()),
            &mut name_length,
            PWSTR(domain.as_mut_ptr()),
            &mut domain_length,
            &mut kind,
        )
    })?;
    let name = String::from_utf16_lossy(&name[..name_length as usize]);
    let domain = String::from_utf16_lossy(&domain[..domain_length as usize]);
    Ok(if domain.is_empty() {
        name
    } else {
        format!("{domain}\\{name}")
    })
}

fn elevated(token: &OwnedHandle) -> Result<bool, String> {
    let buffer = token_information(token, TokenElevation)?;
    Ok(unsafe { (*buffer.as_ptr().cast::<TOKEN_ELEVATION>()).TokenIsElevated } != 0)
}

fn architecture(handle: &OwnedHandle) -> Result<&'static str, String> {
    let mut info = PROCESS_MACHINE_INFORMATION::default();
    let machine = if unsafe {
        GetProcessInformation(
            handle.as_raw_handle(),
            ProcessMachineTypeInfo,
            (&raw mut info).cast(),
            size_of::<PROCESS_MACHINE_INFORMATION>() as u32,
        )
    }
    .as_bool()
    {
        info.ProcessMachine
    } else {
        // ProcessMachineTypeInfo requires Windows 11; earlier versions only report WOW64.
        let mut process = 0;
        let mut native = 0;
        check(unsafe { IsWow64Process2(handle.as_raw_handle(), &mut process, &mut native) })?;
        if process == IMAGE_FILE_MACHINE_UNKNOWN as u16 {
            native
        } else {
            process
        }
    };
    Ok(match i32::from(machine) {
        IMAGE_FILE_MACHINE_AMD64 => "x64",
        IMAGE_FILE_MACHINE_ARM64 => "ARM64",
        IMAGE_FILE_MACHINE_I386 => "x86",
        IMAGE_FILE_MACHINE_ARMNT => "ARM",
        _ => "Unknown",
    })
}

fn local_time(created: u64) -> String {
    let file_time = FILETIME {
        dwLowDateTime: created as u32,
        dwHighDateTime: (created >> 32) as u32,
    };
    let mut utc = SYSTEMTIME::default();
    let mut local = SYSTEMTIME::default();
    if unsafe { FileTimeToSystemTime(&file_time, &mut utc) }.as_bool()
        && unsafe { SystemTimeToTzSpecificLocalTime(std::ptr::null(), &utc, &mut local) }.as_bool()
    {
        format!(
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            local.wYear, local.wMonth, local.wDay, local.wHour, local.wMinute, local.wSecond
        )
    } else {
        "Unavailable".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn current() -> ProcessId {
        let pid = std::process::id();
        ProcessId {
            pid,
            created: Some(times(&open(pid).unwrap()).unwrap().created),
            observation: 0,
        }
    }

    #[test]
    fn details_verify_creation_time() {
        let id = current();
        let reused = ProcessId {
            created: Some(id.created.unwrap() + 1),
            ..id
        };
        assert!(identity(reused).is_err());
        assert!(counters(reused).is_err());
    }

    #[test]
    fn terminate_verifies_creation_time() {
        let mut child = std::process::Command::new("ping")
            .args(["-n", "30", "127.0.0.1"])
            .stdout(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let id = ProcessId {
            pid: child.id(),
            created: Some(times(&open(child.id()).unwrap()).unwrap().created),
            observation: 0,
        };
        let reused = ProcessId {
            created: Some(id.created.unwrap() + 1),
            ..id
        };
        assert!(terminate(reused).is_err());
        assert!(child.try_wait().unwrap().is_none());
        terminate(id).unwrap();
        assert_eq!(child.wait().unwrap().code(), Some(1));
    }

    #[test]
    fn current_process_details() {
        let identity = identity(current()).unwrap();
        let path = identity.path.unwrap();
        assert_eq!(
            std::path::Path::new(&path),
            std::env::current_exe().unwrap()
        );
        assert!(identity.user.unwrap().contains('\\'));
        identity.elevated.unwrap();
        assert_ne!(identity.architecture.unwrap(), "Unknown");
        let counters = counters(current()).unwrap();
        // CI runners may start tests below normal priority, so only require a known class.
        assert_ne!(counters.priority.unwrap(), "Unknown");
        assert!(counters.handles.unwrap() > 0);
        counters.gdi_objects.unwrap();
        counters.user_objects.unwrap();
        assert!(counters.memory.unwrap().PrivateUsage > 0);
    }

    #[test]
    fn version_resources_are_read() {
        let system = std::env::var("SystemRoot").unwrap();
        let version = version(&format!("{system}\\System32\\kernel32.dll")).unwrap();
        assert!(version.company.unwrap().contains("Microsoft"));
        assert!(version.description.is_some());
        assert!(version.file_version.unwrap().starts_with("10.0."));
    }
}
