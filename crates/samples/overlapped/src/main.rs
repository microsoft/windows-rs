use windows::{core::*, Win32::Foundation::*, Win32::Storage::FileSystem::*, Win32::System::Threading::*, Win32::System::IO::*};

fn main() -> Result<()> {
    unsafe {
        let mut filename = std::env::current_dir().unwrap();
        filename.push("message.txt");

        let file = CreateFileA(filename.as_path().to_str().unwrap(), FILE_GENERIC_READ, FILE_SHARE_READ, std::ptr::null(), OPEN_EXISTING, FILE_FLAG_OVERLAPPED, None)?;

        let mut overlapped = OVERLAPPED {
            Anonymous: OVERLAPPED_0 { Anonymous: OVERLAPPED_0_0 { Offset: 9, OffsetHigh: 0 } },
            hEvent: CreateEventA(std::ptr::null(), true, false, None)?,
            Internal: 0,
            InternalHigh: 0,
        };

        let mut buffer: [u8; 12] = Default::default();

        let read_ok = ReadFile(file, buffer.as_mut_ptr() as _, 12, std::ptr::null_mut(), &mut overlapped);

        if !read_ok.as_bool() {
            assert_eq!(GetLastError(), ERROR_IO_PENDING);
        }

        let wait_ok = WaitForSingleObject(overlapped.hEvent, 2000);
        assert!(wait_ok == WAIT_OBJECT_0);

        let mut bytes_copied = 0;
        let overlapped_ok = GetOverlappedResult(file, &overlapped, &mut bytes_copied, false);
        assert!(overlapped_ok.as_bool());
        assert!(bytes_copied == 12);

        let closed_ok = CloseHandle(file);
        assert!(closed_ok.as_bool());

        println!("{}", String::from_utf8_lossy(&buffer));
    }

    Ok(())
}
