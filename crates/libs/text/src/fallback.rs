use super::*;

/// Families to try, in order, for characters the primary font cannot display.
///
/// After the listed families, the system fallback applies. Cloning is cheap and clones share the
/// same fallback.
#[derive(Clone)]
pub struct FontFallback(IDWriteFontFallback);

// DirectWrite font fallbacks are immutable and thread-safe.
unsafe impl Send for FontFallback {}
unsafe impl Sync for FontFallback {}

impl FontFallback {
    /// Creates a fallback that tries `families` from `collection`, in order. A family is used for a
    /// character only if the face matching the text's weight and style can display it. Families
    /// that the collection does not contain are skipped.
    pub fn new(collection: &FontCollection, families: &[&str]) -> Self {
        unsafe {
            let builder = factory().CreateFontFallbackBuilder().unwrap();

            if !families.is_empty() {
                let names: Vec<Vec<u16>> = families.iter().map(|family| wide(family)).collect();
                let names: Vec<*const u16> = names.iter().map(|name| name.as_ptr()).collect();
                let all = DWRITE_UNICODE_RANGE {
                    first: 0,
                    last: 0x10FFFF,
                };

                builder
                    .AddMapping(
                        &all,
                        1,
                        names.as_ptr(),
                        names.len().try_into().unwrap(),
                        collection.raw(),
                        None,
                        None,
                        1.0,
                    )
                    .unwrap();
            }

            builder
                .AddMappings(&factory().GetSystemFontFallback().unwrap())
                .unwrap();
            Self(builder.CreateFontFallback().unwrap())
        }
    }

    /// Casts the underlying `IDWriteFontFallback` to `T`.
    pub fn to_interface<T: Interface>(&self) -> Result<T> {
        self.0.cast()
    }

    pub(crate) fn raw(&self) -> &IDWriteFontFallback {
        &self.0
    }
}
