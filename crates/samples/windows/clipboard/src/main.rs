use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::System::DataExchange::*,
    Win32::System::Memory::*,
    Win32::System::Ole::*,
};

fn main() -> Result<()> {
    let text = "Hello from windows-rs!";

    set_clipboard_text(text)?;
    println!("Written to clipboard: {text}");

    let result = get_clipboard_text()?;
    println!("Read from clipboard: {result}");

    Ok(())
}

fn set_clipboard_text(text: &str) -> Result<()> {
    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let byte_len = wide.len() * std::mem::size_of::<u16>();

    unsafe {
        let hmem = GlobalAlloc(GMEM_MOVEABLE, byte_len)?;

        let ptr = GlobalLock(hmem).cast::<u16>();
        if ptr.is_null() {
            GlobalFree(Some(hmem))?;
            return Err(Error::from_thread());
        }
        std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr, wide.len());
        let _ = GlobalUnlock(hmem);

        OpenClipboard(None)?;

        let result = (|| {
            EmptyClipboard()?;
            SetClipboardData(CF_UNICODETEXT.0.into(), Some(HANDLE(hmem.0)))?;
            Ok(())
        })();

        CloseClipboard()?;

        if result.is_err() {
            GlobalFree(Some(hmem))?;
        }

        result
    }
}

fn get_clipboard_text() -> Result<String> {
    unsafe {
        OpenClipboard(None)?;

        let result = (|| {
            let handle = GetClipboardData(CF_UNICODETEXT.0.into())?;
            let hmem = HGLOBAL(handle.0);

            let ptr = GlobalLock(hmem).cast::<u16>();
            if ptr.is_null() {
                return Err(Error::from_thread());
            }

            let mut len = 0usize;
            while *ptr.add(len) != 0 {
                len += 1;
            }

            let text = String::from_utf16_lossy(
                std::slice::from_raw_parts(ptr, len)
            ).to_string();

            let _ = GlobalUnlock(hmem);
            Ok(text)
        })();

        CloseClipboard()?;
        result
    }
}