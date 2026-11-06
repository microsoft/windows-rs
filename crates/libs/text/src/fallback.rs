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
    /// Creates a fallback that tries `families` from `collection`, in order, for the characters
    /// each one covers. Families that the collection does not contain, or that map no characters, are skipped.
    pub fn new(collection: &FontCollection, families: &[&str]) -> Self {
        unsafe {
            let builder = factory().CreateFontFallbackBuilder().unwrap();

            for family in families {
                let Some(face) = collection.find(family, FontWeight::NORMAL, FontStyle::Normal)
                else {
                    continue;
                };
                let ranges = face.unicode_ranges();
                if ranges.is_empty() {
                    continue;
                }
                let name = wide(family);

                builder
                    .AddMapping(
                        ranges.as_ptr(),
                        ranges.len().try_into().unwrap(),
                        &name.as_ptr(),
                        1,
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
