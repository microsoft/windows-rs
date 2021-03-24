use bindings::{
    windows::win32::debug::*, windows::win32::file_system::*, windows::win32::system_services::*,
    windows::win32::windows_programming::*,
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
            HANDLE(0),
        );

        if file == INVALID_HANDLE_VALUE {
            windows::ErrorCode::from_thread().ok()?;
        }

        let mut overlapped = OVERLAPPED {
            anonymous: OVERLAPPED_0 {
                anonymous: OVERLAPPED_0_0 {
                    offset: 9,
                    offset_high: 0,
                },
            },
            h_event: CreateEventA(std::ptr::null_mut(), true, false, PSTR::default()),
            internal: 0,
            internal_high: 0,
        };

        assert!(overlapped.h_event.0 != 0);

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

        let wait_ok = WaitForSingleObject(overlapped.h_event, 2000);
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
