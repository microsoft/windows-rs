/// A stream of tokens
#[derive(Debug, Clone)]
pub struct TokenStream {
    inner: String,
}

impl From<String> for TokenStream {
    fn from(inner: String) -> Self {
        Self { inner }
    }
}

impl From<&String> for TokenStream {
    fn from(inner: &String) -> Self {
        Self { inner: inner.to_string() }
    }
}

impl From<&str> for TokenStream {
    fn from(inner: &str) -> Self {
        Self { inner: inner.to_string() }
    }
}

impl TokenStream {
    pub fn new() -> Self {
        Self { inner: String::new() }
    }

    pub fn with_capacity() -> Self {
        Self { inner: String::with_capacity(1000) }
    }

    /// Appends another stream to the stream
    ///
    /// note: a space will be inserted before the other stream
    pub fn combine(&mut self, other: &TokenStream) {
        self.push_space();
        self.inner.push_str(&other.inner)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// View the stream as a string
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Convert the stream into a `String`
    pub fn into_string(self) -> String {
        self.inner
    }

    /// Parse the token stream as something
    ///
    /// Mostly used with `proc_macro2::TokenStream` or `proc_macro::TokenStream`
    pub fn parse<T: core::str::FromStr>(self) -> Result<T, T::Err> {
        self.into_string().parse()
    }

    pub(crate) fn push_space(&mut self) {
        match self.last_char() {
            None | Some(' ') => {}
            _ => self.inner.push(' '),
        }
    }

    pub fn push(&mut self, c: char) {
        self.inner.push(c)
    }

    pub fn push_str(&mut self, str: &str) {
        self.inner.push_str(str)
    }

    fn last_char(&self) -> Option<char> {
        self.inner.chars().last()
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::new()
    }
}

impl core::iter::FromIterator<TokenStream> for TokenStream {
    fn from_iter<I: IntoIterator<Item = TokenStream>>(iter: I) -> Self {
        iter.into_iter()
            .fold(None, |accum: Option<TokenStream>, n| {
                let mut ts = accum.unwrap_or_else(TokenStream::new);
                ts.combine(&n);
                Some(ts)
            })
            .unwrap_or_else(TokenStream::new)
    }
}

/// A delimiter around a block of code
#[derive(Copy, Clone)]
pub enum Delimiter {
    /// `[]`
    Bracket,
    /// `{}`
    Brace,
    /// `()`
    Parenthesis,
}

impl Delimiter {
    /// The opening delimiter
    pub fn open(self) -> char {
        match self {
            Delimiter::Bracket => '[',
            Delimiter::Brace => '{',
            Delimiter::Parenthesis => '(',
        }
    }

    /// The closing delimiter
    pub fn close(self) -> char {
        match self {
            Delimiter::Bracket => ']',
            Delimiter::Brace => '}',
            Delimiter::Parenthesis => ')',
        }
    }
}

/// A literal of some sort
pub struct Literal {
    inner: String,
}

macro_rules! unsuffixed {
    ($ty:ty => $name:ident) => {
        pub fn $name(n: $ty) -> Self {
            Self { inner: n.to_string() }
        }
    };
}

impl Literal {
    unsuffixed!(usize => usize_unsuffixed);
    unsuffixed!(u32 => u32_unsuffixed);
    unsuffixed!(u16 => u16_unsuffixed);
    unsuffixed!(u8 => u8_unsuffixed);

    pub fn byte_string(s: &[u8]) -> Self {
        Self {
            inner: format!("b\"{}\"", core::str::from_utf8(s).expect("Could not turn bytes into byte literal")),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl core::fmt::Display for TokenStream {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
