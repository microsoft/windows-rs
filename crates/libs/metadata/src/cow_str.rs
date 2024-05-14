// Should all windows-metadata table functions return CowStr instead of &str?

// Can't actually use Cow since it doesn't derive PartialEq so it cannot be used in pattern expressions. 🤷
#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum CowStr {
    Borrowed(&'static str),
    Owned(String),
}

impl From<&'static str> for CowStr {
    fn from(from: &'static str) -> Self {
        Self::Borrowed(from)
    }
}

impl std::fmt::Display for CowStr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.as_str())
    }
}

impl std::ops::Deref for CowStr {
    type Target = str;

    fn deref(&self) -> &str {
        match *self {
            Self::Borrowed(borrowed) => borrowed,
            Self::Owned(ref owned) => owned,
        }
    }
}

impl Default for CowStr {
    fn default() -> Self {
        Self::Borrowed("")
    }
}

impl CowStr {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Borrowed(value) => value,
            Self::Owned(value) => value,
        }
    }
}

impl PartialEq<str> for CowStr {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<str> for &CowStr {
    fn eq(&self, other: &str) -> bool {
        **self == *other
    }
}

impl PartialEq<&str> for CowStr {
    fn eq(&self, other: &&str) -> bool {
        *self == **other
    }
}

impl PartialEq<CowStr> for str {
    fn eq(&self, other: &CowStr) -> bool {
        *other == *self
    }
}

impl PartialEq<CowStr> for &str {
    fn eq(&self, other: &CowStr) -> bool {
        *other == **self
    }
}

impl PartialEq<&CowStr> for str {
    fn eq(&self, other: &&CowStr) -> bool {
        **other == *self
    }
}
