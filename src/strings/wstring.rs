use std::borrow::Cow;

/// A null-terminated string of 16 bit wide characters
#[derive(PartialEq, Eq)]
pub struct WString(Cow<'static, [u16]>);

impl WString {
    /// Create a `WString` from a slice of wide chars without performing safety checks
    ///
    /// Safety: The slice must contain no null bytes except one at the end
    pub const unsafe fn from_static_slice_unchecked(slice: &'static [u16]) -> Self {
        Self(Cow::Borrowed(slice))
    }

    /// Create a `WString` from a slice of wide chars.
    ///
    /// Returns `None` if slice is empty, contains any intermediary null bytes or does not end with null
    pub unsafe fn from_static_slice(slice: &'static [u16]) -> Option<Self> {
        if slice.len() > 0
            && slice.iter().take(slice.len() - 1).any(|&s| s == 0)
            && slice[slice.len() - 1] == 0
        {
            return None;
        }
        Some(Self(Cow::Borrowed(slice)))
    }

    /// Gets `WString` as a raw pointer.
    pub fn as_ptr(&self) -> *const u16 {
        self.0.as_ptr()
    }

    /// The length of the string *including* the null byte
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets substring `WString` for provided range
    ///
    /// # Panics
    /// This method panics if the provided range is out of range
    pub fn substring<R: std::ops::RangeBounds<usize>>(&self, range: R) -> Self {
        use std::ops::Bound;
        let start = match range.start_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => b + 1,
            Bound::Unbounded => 0,
        };
        let mut end = match range.end_bound() {
            Bound::Included(b) => b + 1,
            Bound::Excluded(b) => *b,
            Bound::Unbounded => self.len(),
        };
        let reborrow = match self.0 {
            Cow::Borrowed(s) if range.contains(&(s.len() - 2)) => {
                end = s.len();
                Some(&s[start..])
            }
            _ => None,
        };
        if let Some(reborrow) = reborrow {
            WString(Cow::Borrowed(reborrow))
        } else {
            let mut slice = self.0[start..end].to_owned();
            debug_assert!(slice[slice.len() - 1] != 0);
            slice.push(0);
            WString(Cow::Owned(slice))
        }
    }
}

impl std::fmt::Display for WString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        for c in core::char::decode_utf16(self.0.iter().copied().take(self.len() - 1)) {
            f.write_char(c.unwrap())?
        }
        Ok(())
    }
}

impl std::fmt::Debug for WString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<String> for WString {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

impl<'a> From<&'a str> for WString {
    fn from(s: &'a str) -> Self {
        let mut result: Vec<u16> = s.encode_utf16().collect();
        result.push(0);
        WString(Cow::Owned(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn substring() {
        let wide: WString = crate::L!("Hello");
        let sub = wide.substring(0..5);

        assert!(matches!(sub.0, Cow::Borrowed(_)));
        assert_eq!(wide, sub);

        let sub = wide.substring(0..);

        assert!(matches!(sub.0, Cow::Borrowed(_)));
        assert_eq!(wide, sub);

        let sub = wide.substring(0..=5);

        assert!(matches!(sub.0, Cow::Borrowed(_)));
        assert_eq!(wide, sub);

        let sub = wide.substring(1..=3);

        assert!(matches!(sub.0, Cow::Owned(_)));
        assert_eq!(&format!("{}", sub), "ell");

        let sub = wide.substring(1..4);

        assert!(matches!(sub.0, Cow::Owned(_)));
        assert_eq!(&format!("{}", sub), "ell");
    }

    #[test]
    fn format() {
        let wide: WString = crate::L!("Hello");
        let string = format!("{}", wide);

        assert_eq!(string, "Hello");
    }
}
