fn main() -> windows::core::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows::{Win32::*, core::WIN32_ERROR, core::*};

    unsafe {
        let filename: Vec<u16> = std::path::Path::new(env!("OUT_DIR"))
            .join("message.txt")
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        let file = CreateFileW(
            PCWSTR(filename.as_ptr()),
            FILE_GENERIC_READ as u32,
            FILE_SHARE_READ as u32,
            None,
            OPEN_EXISTING as u32,
            FILE_FLAG_OVERLAPPED as u32,
            None,
        );
        if file == INVALID_HANDLE_VALUE {
            return Err(Error::from_thread());
        }

        let event = CreateEventW(None, true, false, None);
        if event.0.is_null() {
            return Err(Error::from_thread());
        }

        let mut overlapped = OVERLAPPED {
            Anonymous: OVERLAPPED_0 {
                Anonymous: OVERLAPPED_0_0 {
                    Offset: 9,
                    OffsetHigh: 0,
                },
            },
            hEvent: event,
            Internal: 0,
            InternalHigh: 0,
        };

        let mut buffer: [u8; 12] = Default::default();

        if let Err(error) = ReadFile(
            file,
            Some(buffer.as_mut_ptr() as *mut core::ffi::c_void),
            buffer.len() as u32,
            None,
            Some(&mut overlapped),
        )
        .ok()
        {
            assert_eq!(error.code(), WIN32_ERROR(ERROR_IO_PENDING as u32).into());
        }

        let wait = WaitForSingleObject(overlapped.hEvent, INFINITE);
        if wait == WAIT_FAILED {
            return Err(Error::from_thread());
        }
        assert_eq!(wait, WAIT_OBJECT_0 as u32);

        let mut bytes_copied = 0;
        GetOverlappedResult(file, &overlapped, &mut bytes_copied, false).ok()?;
        assert!(bytes_copied == 12);

        println!("{}", String::from_utf8_lossy(&buffer));

        CloseHandle(event).ok()?;
        CloseHandle(file).ok()?;
    }

    Ok(())
}
