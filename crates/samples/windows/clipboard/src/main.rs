use windows::{
    core::*, Win32::Foundation::*, Win32::System::DataExchange::*, Win32::System::Memory::*,
    Win32::System::Ole::*,
};

fn main() -> Result<()> {
    unsafe {
        let text = "Hello from windows-rs!";

        let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
        let byte_len = wide.len() * std::mem::size_of::<u16>();

        let hmem = GlobalAlloc(GMEM_MOVEABLE, byte_len)?;
        let ptr = GlobalLock(hmem).cast::<u16>();
        std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr, wide.len());
        let _ = GlobalUnlock(hmem);

        OpenClipboard(None)?;
        EmptyClipboard()?;
        SetClipboardData(CF_UNICODETEXT.0.into(), Some(HANDLE(hmem.0)))?;
        CloseClipboard()?;

        println!("Written to clipboard: {text}");

        OpenClipboard(None)?;
        let handle = GetClipboardData(CF_UNICODETEXT.0.into())?;
        let hmem = HGLOBAL(handle.0);
        let ptr = GlobalLock(hmem).cast::<u16>();
        let max_len = GlobalSize(hmem) / std::mem::size_of::<u16>();
        let len = (0..max_len)
            .position(|i| *ptr.add(i) == 0)
            .unwrap_or(max_len);
        let result = String::from_utf16_lossy(std::slice::from_raw_parts(ptr, len));
        let _ = GlobalUnlock(hmem);
        CloseClipboard()?;

        println!("Read from clipboard: {result}");

        Ok(())
    }
}
