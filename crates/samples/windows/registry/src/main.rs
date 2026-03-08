use windows::{core::*, Win32::System::Registry::*, };

fn main() -> Result<()> {
    unsafe {
        let mut key = HKEY::default();
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            w!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion"),
            None,
            KEY_READ,
            &mut key,
        ).ok()?;

        for name in [w!("ProductName"), w!("DisplayVersion"), w!("CurrentBuild")] {
            let mut buf = [0u16; 512];
            let mut buf_len = std::mem::size_of_val(&buf) as u32;

            RegQueryValueExW(
                key,
                name,
                None,
                None,
                Some(buf.as_mut_ptr() as *mut u8),
                Some(&mut buf_len),
            ).ok()?;

            let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
            println!("{}", String::from_utf16_lossy(&buf[..len]));
        }

        RegCloseKey(key).ok()?;
        Ok(())
    }
}