use super::*;
use std::sync::OnceLock;

// The shared factory is shared by every DirectWrite component in the process and is not tied to a
// thread, nor is the in-memory loader. The metadata does not mark them `[agile]`, so wrap them
// for the process-wide statics.
struct Shared<T>(T);
unsafe impl<T> Send for Shared<T> {}
unsafe impl<T> Sync for Shared<T> {}

/// Returns the process-wide shared DirectWrite factory.
pub(crate) fn factory() -> &'static IDWriteFactory5 {
    static SHARED: OnceLock<Shared<IDWriteFactory5>> = OnceLock::new();

    &SHARED
        .get_or_init(|| {
            let mut factory: Option<IDWriteFactory5> = None;
            unsafe {
                DWriteCreateFactory(
                    DWRITE_FACTORY_TYPE_SHARED,
                    &IDWriteFactory5::IID,
                    &mut factory as *mut _ as *mut _,
                )
                .unwrap();
            }
            Shared(factory.unwrap())
        })
        .0
}

/// Returns the process-wide in-memory font file loader, registered with [`factory`].
pub(crate) fn memory_loader() -> &'static IDWriteInMemoryFontFileLoader {
    static SHARED: OnceLock<Shared<IDWriteInMemoryFontFileLoader>> = OnceLock::new();

    &SHARED
        .get_or_init(|| unsafe {
            let loader = factory().CreateInMemoryFontFileLoader().unwrap();
            factory().RegisterFontFileLoader(&loader).unwrap();
            Shared(loader)
        })
        .0
}

/// Returns the user's default locale name, null-terminated.
pub(crate) fn user_locale() -> &'static [u16] {
    static LOCALE: OnceLock<Vec<u16>> = OnceLock::new();

    LOCALE.get_or_init(|| {
        let mut buffer = [0; LOCALE_NAME_MAX_LENGTH as usize];
        let len =
            unsafe { GetUserDefaultLocaleName(PWSTR(buffer.as_mut_ptr()), buffer.len() as i32) };

        // The length includes the null terminator, and is zero if no locale is set.
        if len > 0 {
            buffer[..len as usize].to_vec()
        } else {
            wide("en-us")
        }
    })
}

/// Encodes `text` as a null-terminated UTF-16 string.
pub(crate) fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Returns the string in `strings` for the user's locale, falling back to US English and then
/// the first string.
pub(crate) fn localized(strings: &IDWriteLocalizedStrings) -> String {
    unsafe {
        let mut index = 0;
        let mut exists = BOOL::default();

        for locale in [user_locale(), &wide("en-us")] {
            strings
                .FindLocaleName(locale.as_ptr(), &mut index, &mut exists)
                .unwrap();
            if exists.as_bool() {
                break;
            }
        }

        if !exists.as_bool() {
            index = 0;
        }

        let mut len = 0;
        strings.GetStringLength(index, &mut len).unwrap();
        let mut buffer = vec![0; len as usize + 1];
        strings
            .GetString(index, buffer.as_mut_ptr(), buffer.len() as u32)
            .unwrap();
        String::from_utf16_lossy(&buffer[..len as usize])
    }
}
