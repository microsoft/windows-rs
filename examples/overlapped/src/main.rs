use bindings::{
    Windows::Win32::Debug::*, Windows::Win32::FileSystem::*, Windows::Win32::SystemServices::*,
    Windows::Win32::WindowsProgramming::*,
};

fn main() -> windows::Result<()> {
    unsafe {
        let mut filename = std::env::current_dir().unwrap();
        filename.push("message.txt");

        let file = CreateFileA(
            filename.as_path().to_str().unwrap(),
            FILE_ACCESS_FLAGS::FILE_GENERIC_READ,
            FILE_SHARE_MODE::FILE_SHARE_READ,
            std::ptr::null_mut(),
            FILE_CREATION_DISPOSITION::OPEN_EXISTING,
            FILE_FLAGS_AND_ATTRIBUTES::FILE_FLAG_OVERLAPPED,
            HANDLE::NULL,
        );

        if file.is_invalid() {
            windows::ErrorCode::from_thread().ok()?;
        }

        let mut overlapped = OVERLAPPED {
            Anonymous: OVERLAPPED_0 {
                Anonymous: OVERLAPPED_0_0 {
                    Offset: 9,
                    OffsetHigh: 0,
                },
            },
            hEvent: CreateEventA(std::ptr::null_mut(), true, false, PSTR::NULL),
            Internal: 0,
            InternalHigh: 0,
        };

        assert!(overlapped.hEvent.0 != 0);

        let mut buffer: [u8; 12] = Default::default();
        let read_ok = ReadFile(
            file,
            buffer.as_mut_ptr() as _,
            12,
            std::ptr::null_mut(),
            &mut overlapped,
        );

        if !read_ok.as_bool() {
            assert_eq!(GetLastError(), ERROR_IO_PENDING as u32);
        }

        let wait_ok = WaitForSingleObject(overlapped.hEvent, 2000);
        assert!(wait_ok == WAIT_RETURN_CAUSE::WAIT_OBJECT_0);

        let mut bytes_copied = 0;
        let overlapped_ok = GetOverlappedResult(file, &mut overlapped, &mut bytes_copied, false);
        assert!(overlapped_ok.as_bool());
        assert!(bytes_copied == 12);

        let closed_ok = CloseHandle(file);
        assert!(closed_ok.as_bool());

        println!("{}", String::from_utf8_lossy(&buffer));
    }

    Ok(())
}
