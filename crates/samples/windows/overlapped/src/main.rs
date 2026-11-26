fn main() -> windows::core::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows::{
        Win32::Foundation::*, Win32::Storage::FileSystem::*, Win32::System::IO::*,
        Win32::System::Threading::*, core::*,
    };

    unsafe {
        let filename: Vec<u16> = std::path::Path::new(env!("OUT_DIR"))
            .join("message.txt")
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect();
        let file = CreateFileW(
            PCWSTR(filename.as_ptr()),
            FILE_GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAG_OVERLAPPED,
            None,
        )?;

        let event = CreateEventW(None, true, false, None)?;

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

        if let Err(error) = ReadFile(file, Some(&mut buffer), None, Some(&mut overlapped)) {
            assert_eq!(error.code(), ERROR_IO_PENDING.into());
        }

        let wait = WaitForSingleObject(overlapped.hEvent, INFINITE);
        if wait == WAIT_FAILED {
            return Err(Error::from_thread());
        }
        assert_eq!(wait, WAIT_OBJECT_0);

        let mut bytes_copied = 0;
        GetOverlappedResult(file, &overlapped, &mut bytes_copied, false)?;
        assert!(bytes_copied == 12);

        println!("{}", String::from_utf8_lossy(&buffer));

        CloseHandle(event)?;
        CloseHandle(file)?;
    }

    Ok(())
}
