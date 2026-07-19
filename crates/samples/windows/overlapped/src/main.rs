fn main() -> windows::core::Result<()> {
    use windows::{Win32::*, core::WIN32_ERROR, core::*};

    unsafe {
        let mut filename = std::env::current_dir().unwrap();
        filename.push("message.txt");

        let mut string = filename.as_path().to_str().unwrap().to_owned();
        string.push('\0');
        let file = CreateFileA(
            PCSTR(string.as_ptr()),
            FILE_GENERIC_READ,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAG_OVERLAPPED,
            None,
        );
        if file == INVALID_HANDLE_VALUE {
            return Err(Error::from_thread());
        }

        let mut overlapped = OVERLAPPED {
            Anonymous: OVERLAPPED_0 {
                Anonymous: OVERLAPPED_0_0 {
                    Offset: 9,
                    OffsetHigh: 0,
                },
            },
            hEvent: CreateEventA(None, true, false, None),
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
            assert_eq!(error.code(), WIN32_ERROR(ERROR_IO_PENDING).into());
        }

        WaitForSingleObject(overlapped.hEvent, 2000);

        let mut bytes_copied = 0;
        GetOverlappedResult(file, &overlapped, &mut bytes_copied, false).ok()?;
        assert!(bytes_copied == 12);

        println!("{}", String::from_utf8_lossy(&buffer));

        CloseHandle(overlapped.hEvent).ok()?;
        CloseHandle(file).ok()?;
    }

    Ok(())
}
