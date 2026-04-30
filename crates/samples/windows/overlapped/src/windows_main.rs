use windows::{
    core::*, Win32::Foundation::*, Win32::Storage::FileSystem::*, Win32::System::Threading::*,
    Win32::System::IO::*,
};

fn main() -> Result<()> {
    unsafe {
        let mut filename = std::env::current_dir().unwrap();
        filename.push("message.txt");

        let mut string = filename.as_path().to_str().unwrap().to_owned();
        string.push('\0');
        let file = Owned::new(CreateFileA(
            PCSTR(string.as_ptr()),
            // See: https://github.com/microsoft/win32metadata/issues/1457
            FILE_GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAG_OVERLAPPED,
            None,
        )?);

        let mut overlapped = OVERLAPPED {
            Anonymous: OVERLAPPED_0 {
                Anonymous: OVERLAPPED_0_0 {
                    Offset: 9,
                    OffsetHigh: 0,
                },
            },
            hEvent: CreateEventA(None, true, false, None)?,
            Internal: 0,
            InternalHigh: 0,
        };

        let mut buffer: [u8; 12] = Default::default();

        if let Err(error) = ReadFile(*file, Some(&mut buffer), None, Some(&mut overlapped)) {
            assert_eq!(error.code(), ERROR_IO_PENDING.into());
        }

        WaitForSingleObject(overlapped.hEvent, 2000);

        let mut bytes_copied = 0;
        GetOverlappedResult(*file, &overlapped, &mut bytes_copied, false)?;
        assert!(bytes_copied == 12);

        println!("{}", String::from_utf8_lossy(&buffer));
    }

    Ok(())
}
